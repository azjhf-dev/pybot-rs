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


thread_local! { static G_ALL_BETTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AMBIGUOUS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AMBIGUOUS_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FEED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BLUE_BETWEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BLUE_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BLUE_LOOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_IN_THE_ROOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CALENDAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHEMICAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONFUSED_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONFUSED_SIGN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEATH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DURATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAMILY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_OF_SPEECH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIT_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIT_ROUTINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIT_STILL_RINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GAIT_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GAIT_STEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GREY_BETWEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GREY_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GREY_LOOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GREY_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GREY_PERSON_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INGEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LAUGH_IDIOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MANNER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEDICINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEARLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NORMAL_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOSE_BREATH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOSE_RED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOSE_RED_BESIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOW_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOW_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OBJECTISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OBJECT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONLINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ORDINARY_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAST_MARK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POISON_NOW_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POISON_TODAY_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRONOUN_SAFE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRO_ANSWERING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SECOND_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SETTLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SETTLING_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SMALL_OBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SMALL_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOFT_STATES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STUCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SYMPTOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TODAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TODAY_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TO_PAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERBY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WAKE_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WAKE_OBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WAKE_OCCASION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__dream: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ALL_BETTER() -> Py { ensure_init(); G_ALL_BETTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALL_BETTER(v: Py) { ensure_init(); G_ALL_BETTER.with(|c| *c.borrow_mut() = v); }
pub fn g_AMBIGUOUS() -> Py { ensure_init(); G_AMBIGUOUS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AMBIGUOUS(v: Py) { ensure_init(); G_AMBIGUOUS.with(|c| *c.borrow_mut() = v); }
pub fn g_AMBIGUOUS_CUES() -> Py { ensure_init(); G_AMBIGUOUS_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AMBIGUOUS_CUES(v: Py) { ensure_init(); G_AMBIGUOUS_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKING() -> Py { ensure_init(); G_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING(v: Py) { ensure_init(); G_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FEED() -> Py { ensure_init(); G_A_FEED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FEED(v: Py) { ensure_init(); G_A_FEED.with(|c| *c.borrow_mut() = v); }
pub fn g_BLUE_BETWEEN() -> Py { ensure_init(); G_BLUE_BETWEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BLUE_BETWEEN(v: Py) { ensure_init(); G_BLUE_BETWEEN.with(|c| *c.borrow_mut() = v); }
pub fn g_BLUE_BODY() -> Py { ensure_init(); G_BLUE_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BLUE_BODY(v: Py) { ensure_init(); G_BLUE_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_BLUE_LOOSE() -> Py { ensure_init(); G_BLUE_LOOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BLUE_LOOSE(v: Py) { ensure_init(); G_BLUE_LOOSE.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_IN_THE_ROOM() -> Py { ensure_init(); G_BODY_IN_THE_ROOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_IN_THE_ROOM(v: Py) { ensure_init(); G_BODY_IN_THE_ROOM.with(|c| *c.borrow_mut() = v); }
pub fn g_CALENDAR() -> Py { ensure_init(); G_CALENDAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALENDAR(v: Py) { ensure_init(); G_CALENDAR.with(|c| *c.borrow_mut() = v); }
pub fn g_CHEMICAL() -> Py { ensure_init(); G_CHEMICAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHEMICAL(v: Py) { ensure_init(); G_CHEMICAL.with(|c| *c.borrow_mut() = v); }
pub fn g_CONFUSED_AFTER() -> Py { ensure_init(); G_CONFUSED_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONFUSED_AFTER(v: Py) { ensure_init(); G_CONFUSED_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_CONFUSED_SIGN() -> Py { ensure_init(); G_CONFUSED_SIGN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONFUSED_SIGN(v: Py) { ensure_init(); G_CONFUSED_SIGN.with(|c| *c.borrow_mut() = v); }
pub fn g_DEATH() -> Py { ensure_init(); G_DEATH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEATH(v: Py) { ensure_init(); G_DEATH.with(|c| *c.borrow_mut() = v); }
pub fn g_DURATION() -> Py { ensure_init(); G_DURATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DURATION(v: Py) { ensure_init(); G_DURATION.with(|c| *c.borrow_mut() = v); }
pub fn g_FAMILY() -> Py { ensure_init(); G_FAMILY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAMILY(v: Py) { ensure_init(); G_FAMILY.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_OF_SPEECH() -> Py { ensure_init(); G_FIGURE_OF_SPEECH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_OF_SPEECH(v: Py) { ensure_init(); G_FIGURE_OF_SPEECH.with(|c| *c.borrow_mut() = v); }
pub fn g_FIT_OF() -> Py { ensure_init(); G_FIT_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIT_OF(v: Py) { ensure_init(); G_FIT_OF.with(|c| *c.borrow_mut() = v); }
pub fn g_FIT_ROUTINE() -> Py { ensure_init(); G_FIT_ROUTINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIT_ROUTINE(v: Py) { ensure_init(); G_FIT_ROUTINE.with(|c| *c.borrow_mut() = v); }
pub fn g_FIT_STILL_RINGS() -> Py { ensure_init(); G_FIT_STILL_RINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIT_STILL_RINGS(v: Py) { ensure_init(); G_FIT_STILL_RINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_GAIT_BEFORE() -> Py { ensure_init(); G_GAIT_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GAIT_BEFORE(v: Py) { ensure_init(); G_GAIT_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_GAIT_STEM() -> Py { ensure_init(); G_GAIT_STEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GAIT_STEM(v: Py) { ensure_init(); G_GAIT_STEM.with(|c| *c.borrow_mut() = v); }
pub fn g_GREY_BETWEEN() -> Py { ensure_init(); G_GREY_BETWEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GREY_BETWEEN(v: Py) { ensure_init(); G_GREY_BETWEEN.with(|c| *c.borrow_mut() = v); }
pub fn g_GREY_BODY() -> Py { ensure_init(); G_GREY_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GREY_BODY(v: Py) { ensure_init(); G_GREY_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_GREY_LOOSE() -> Py { ensure_init(); G_GREY_LOOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GREY_LOOSE(v: Py) { ensure_init(); G_GREY_LOOSE.with(|c| *c.borrow_mut() = v); }
pub fn g_GREY_PEOPLE() -> Py { ensure_init(); G_GREY_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GREY_PEOPLE(v: Py) { ensure_init(); G_GREY_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_GREY_PERSON_STATE() -> Py { ensure_init(); G_GREY_PERSON_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GREY_PERSON_STATE(v: Py) { ensure_init(); G_GREY_PERSON_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_INGEST() -> Py { ensure_init(); G_INGEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INGEST(v: Py) { ensure_init(); G_INGEST.with(|c| *c.borrow_mut() = v); }
pub fn g_LAUGH_IDIOM() -> Py { ensure_init(); G_LAUGH_IDIOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LAUGH_IDIOM(v: Py) { ensure_init(); G_LAUGH_IDIOM.with(|c| *c.borrow_mut() = v); }
pub fn g_MANNER() -> Py { ensure_init(); G_MANNER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MANNER(v: Py) { ensure_init(); G_MANNER.with(|c| *c.borrow_mut() = v); }
pub fn g_MEDICINE() -> Py { ensure_init(); G_MEDICINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEDICINE(v: Py) { ensure_init(); G_MEDICINE.with(|c| *c.borrow_mut() = v); }
pub fn g_NEARLY() -> Py { ensure_init(); G_NEARLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEARLY(v: Py) { ensure_init(); G_NEARLY.with(|c| *c.borrow_mut() = v); }
pub fn g_NORMAL_ASK() -> Py { ensure_init(); G_NORMAL_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NORMAL_ASK(v: Py) { ensure_init(); G_NORMAL_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_NOSE_BREATH() -> Py { ensure_init(); G_NOSE_BREATH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOSE_BREATH(v: Py) { ensure_init(); G_NOSE_BREATH.with(|c| *c.borrow_mut() = v); }
pub fn g_NOSE_RED() -> Py { ensure_init(); G_NOSE_RED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOSE_RED(v: Py) { ensure_init(); G_NOSE_RED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOSE_RED_BESIDE() -> Py { ensure_init(); G_NOSE_RED_BESIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOSE_RED_BESIDE(v: Py) { ensure_init(); G_NOSE_RED_BESIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_BODY() -> Py { ensure_init(); G_NOT_A_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_BODY(v: Py) { ensure_init(); G_NOT_A_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOW() -> Py { ensure_init(); G_NOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOW(v: Py) { ensure_init(); G_NOW.with(|c| *c.borrow_mut() = v); }
pub fn g_NOW_REPLIES() -> Py { ensure_init(); G_NOW_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOW_REPLIES(v: Py) { ensure_init(); G_NOW_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_NOW_WORDS() -> Py { ensure_init(); G_NOW_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOW_WORDS(v: Py) { ensure_init(); G_NOW_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_OBJECTISH() -> Py { ensure_init(); G_OBJECTISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OBJECTISH(v: Py) { ensure_init(); G_OBJECTISH.with(|c| *c.borrow_mut() = v); }
pub fn g_OBJECT_WORD() -> Py { ensure_init(); G_OBJECT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OBJECT_WORD(v: Py) { ensure_init(); G_OBJECT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_ONLINE() -> Py { ensure_init(); G_ONLINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONLINE(v: Py) { ensure_init(); G_ONLINE.with(|c| *c.borrow_mut() = v); }
pub fn g_ORDINARY_STATE() -> Py { ensure_init(); G_ORDINARY_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ORDINARY_STATE(v: Py) { ensure_init(); G_ORDINARY_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_PAST_MARK() -> Py { ensure_init(); G_PAST_MARK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAST_MARK(v: Py) { ensure_init(); G_PAST_MARK.with(|c| *c.borrow_mut() = v); }
pub fn g_PETS() -> Py { ensure_init(); G_PETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PETS(v: Py) { ensure_init(); G_PETS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE() -> Py { ensure_init(); G_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE(v: Py) { ensure_init(); G_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_POISON_NOW_REPLIES() -> Py { ensure_init(); G_POISON_NOW_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POISON_NOW_REPLIES(v: Py) { ensure_init(); G_POISON_NOW_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_POISON_TODAY_REPLIES() -> Py { ensure_init(); G_POISON_TODAY_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POISON_TODAY_REPLIES(v: Py) { ensure_init(); G_POISON_TODAY_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_PRONOUN_SAFE() -> Py { ensure_init(); G_PRONOUN_SAFE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRONOUN_SAFE(v: Py) { ensure_init(); G_PRONOUN_SAFE.with(|c| *c.borrow_mut() = v); }
pub fn g_PRO_ANSWERING() -> Py { ensure_init(); G_PRO_ANSWERING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRO_ANSWERING(v: Py) { ensure_init(); G_PRO_ANSWERING.with(|c| *c.borrow_mut() = v); }
pub fn g_SECOND_CLAUSE() -> Py { ensure_init(); G_SECOND_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SECOND_CLAUSE(v: Py) { ensure_init(); G_SECOND_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_SETTLING() -> Py { ensure_init(); G_SETTLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SETTLING(v: Py) { ensure_init(); G_SETTLING.with(|c| *c.borrow_mut() = v); }
pub fn g_SETTLING_PHRASES() -> Py { ensure_init(); G_SETTLING_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SETTLING_PHRASES(v: Py) { ensure_init(); G_SETTLING_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_SMALL_OBJECT() -> Py { ensure_init(); G_SMALL_OBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SMALL_OBJECT(v: Py) { ensure_init(); G_SMALL_OBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_SMALL_PERSON() -> Py { ensure_init(); G_SMALL_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SMALL_PERSON(v: Py) { ensure_init(); G_SMALL_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_SOFT_STATES() -> Py { ensure_init(); G_SOFT_STATES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOFT_STATES(v: Py) { ensure_init(); G_SOFT_STATES.with(|c| *c.borrow_mut() = v); }
pub fn g_STUCK() -> Py { ensure_init(); G_STUCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STUCK(v: Py) { ensure_init(); G_STUCK.with(|c| *c.borrow_mut() = v); }
pub fn g_SYMPTOM() -> Py { ensure_init(); G_SYMPTOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SYMPTOM(v: Py) { ensure_init(); G_SYMPTOM.with(|c| *c.borrow_mut() = v); }
pub fn g_TODAY() -> Py { ensure_init(); G_TODAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TODAY(v: Py) { ensure_init(); G_TODAY.with(|c| *c.borrow_mut() = v); }
pub fn g_TODAY_REPLIES() -> Py { ensure_init(); G_TODAY_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TODAY_REPLIES(v: Py) { ensure_init(); G_TODAY_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_TO_PAIN() -> Py { ensure_init(); G_TO_PAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TO_PAIN(v: Py) { ensure_init(); G_TO_PAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_VERBY() -> Py { ensure_init(); G_VERBY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERBY(v: Py) { ensure_init(); G_VERBY.with(|c| *c.borrow_mut() = v); }
pub fn g_WAKE_CUES() -> Py { ensure_init(); G_WAKE_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WAKE_CUES(v: Py) { ensure_init(); G_WAKE_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g_WAKE_OBJECT() -> Py { ensure_init(); G_WAKE_OBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WAKE_OBJECT(v: Py) { ensure_init(); G_WAKE_OBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_WAKE_OCCASION() -> Py { ensure_init(); G_WAKE_OCCASION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WAKE_OCCASION(v: Py) { ensure_init(); G_WAKE_OCCASION.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD() -> Py { ensure_init(); G_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD(v: Py) { ensure_init(); G_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__dream() -> Py { ensure_init(); G__dream.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__dream(v: Py) { ensure_init(); G__dream.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Someone they look after, in physical trouble, is never met with a stock line.\n\n`bot/compose/trouble.py` answers the trouble the person is having themselves - their back, their\nboiler - and stands aside for anything about another person, because an emergency about a\nrelative belongs to `bot/modules/safety.py`. That division is right, but the safety band's cue\nlist does not hold the state words yet: on main, of twelve plain reports about a relative's body,\nfour reach an emergency line and the other eight reach a pleasantry.\n\n    my mum is unresponsive            -> Noted! What would be most useful to talk about?\n    my son wont wake up               -> Noted! What would be most useful to talk about?\n    my baby has a temperature of 40   -> Noted! What would be most useful to talk about?\n    my wife is fitting                -> Got it. Is there anything you'd like a hand with?\n\nThe cue list is the safety lane's to own. What the composer owns is that a pleasantry of ours\nis never the answer to one of these, so this module proposes the careful line instead, and only\nwhen no module that can do better has anything to say.\n");
    let __try1 = exc::catch(|| {
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
    s__dream(Py::None);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    s_WORD(stdlib_call("re", "compile", &[py_str("[a-z0-9]+(?:'[a-z]+)?")]).unwrap_or_else(|e| exc::raise(e)));
    s_FAMILY(call_builtin("frozenset", &[py_set(vec![py_str("mum"), py_str("mam"), py_str("mom"), py_str("mother"), py_str("mummy"), py_str("mommy"), py_str("dad"), py_str("father"), py_str("daddy"), py_str("son"), py_str("daughter"), py_str("baby"), py_str("toddler"), py_str("child"), py_str("kid"), py_str("boy"), py_str("girl"), py_str("nan"), py_str("nana"), py_str("granny"), py_str("gran"), py_str("grandma"), py_str("grandad"), py_str("grandpa"), py_str("grandmother"), py_str("grandfather"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("boyfriend"), py_str("girlfriend"), py_str("brother"), py_str("sister"), py_str("friend"), py_str("neighbour"), py_str("neighbor"), py_str("uncle"), py_str("aunt"), py_str("auntie"), py_str("cousin"), py_str("stepson"), py_str("stepdaughter"), py_str("stepdad"), py_str("stepmum"), py_str("fiance"), py_str("fiancee"), py_str("flatmate"), py_str("housemate"), py_str("carer"), py_str("patient"), py_str("niece"), py_str("nephew"), py_str("grandson"), py_str("granddaughter"), py_str("stepson"), py_str("stepdaughter"), py_str("godson"), py_str("goddaughter"), py_str("toddler"), py_str("newborn"), py_str("childminder"), py_str("flatmate"), py_str("teenager"), py_str("teen"), py_str("adolescent")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PRO_ANSWERING(stdlib_call("re", "compile", &[py_str("\\b(?:my|his|her|their|our|the)\\s+(?:carers?|care\\s+workers?|support\\s+workers?|social\\s+workers?|personal\\s+assistants?|nurses?|midwives?|midwife|doctors?|gps?|physios?|therapists?)\\b[^.?!]{0,30}\\b(?:isn'?t|is\\s+not|not|hasn'?t|has\\s+not|never|no)\\s+respond(?:ing|ed|e)?\\b|\\bno\\s+response\\s+from\\s+(?:my|the|our)\\s+(?:carer|care\\s+worker|support\\s+worker|social\\s+worker|personal\\s+assistant|nurse|doctor|gp|physio|therapist)s?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_IN_THE_ROOM(stdlib_call("re", "compile", &[py_str("\\b(?:floor|ground|bed|sofa|settee|couch|bath|pavement|street|doorstep)\\b|\\b(?:collapsed|collapsing|fell|fallen|fainted|pass(?:ed)?\\s+out|seizure|fitting)\\b|\\b(?:breathing|breath|breathe|pulse|blue|lips|cold|clammy|sweating|eyes|pale|grey|gray)\\b|\\b(?:unconscious|out\\s+cold|unresponsive|in\\s+front\\s+of\\s+me|with\\s+me|next\\s+to\\s+me)\\b|\\b(?:to\\s+(?:pain|touch|voices?|noise|light|stimulus)|(?:his|her|their)\\s+name)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOW(py_tuple(vec![py_str("unresponsive"), py_str("not responding"), py_str("no response"), py_str("wont wake"), py_str("won't wake"), py_str("cant wake"), py_str("can't wake"), py_str("not waking"), py_str("not breathing"), py_str("stopped breathing"), py_str("isnt breathing"), py_str("isn't breathing"), py_str("struggling to breathe"), py_str("cant breathe"), py_str("can't breathe"), py_str("gasping"), py_str("struggling for breath"), py_str("fighting for breath"), py_str("gasping for breath"), py_str("struggling for air"), py_str("unconscious"), py_str("unconcious"), py_str("collapsed"), py_str("limp"), py_str("floppy"), py_str("fitting"), py_str("seizure"), py_str("convulsing"), py_str("choking"), py_str("turning blue"), py_str("gone blue"), py_str("blue lips"), py_str("no pulse"), py_str("cardiac"), py_str("overdose"), py_str("took too many"), py_str("swallowed his tongue"), py_str("had a fit"), py_str("has a fit"), py_str("having a fit"), py_str("having fits"), py_str("keeps fitting"), py_str("kept fitting"), py_str("keeps having fits"), py_str("kept having fits"), py_str("isnt responding"), py_str("isn't responding"), py_str("isnt responsive"), py_str("isn't responsive"), py_str("not responsive")]));
    s_BLUE_LOOSE(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|looks?|looking|went|goes|getting|turned|turning|has gone|have gone)\\b[^.?!;]{0,14}\\bblue\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BLUE_BETWEEN(py_str("(?:\\s+(?:a\\s+bit|a\\s+little|very|really|quite|all|pale|grey|gray|and|has\\s+gone|have\\s+gone|had\\s+gone|gone|getting|looking|turning|turned|is|are|was|were|looks?|looked|went|goes))*"));
    s_BLUE_BODY(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:lips?|mouth|tongue|face|cheeks?|skin|fingers?|fingertips?|hands?|nails?|feet|foot|toes?|arms?|legs?|body)\\b"), &g_BLUE_BETWEEN()), &py_str("\\s+blue\\b|\\b(?:he|she|they|baby|boy|girl|kid|child|son|daughter|mum|mam|mom|dad|father|mother|wife|husband|partner|brother|sister|nan|granny|gran|grandad|grandma|grandpa|stepson|stepdaughter|nephew|niece|friend|neighbour)\\b")), &g_BLUE_BETWEEN()), &py_str("\\s+blue\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_GREY_LOOSE(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|looks?|looking|went|goes|getting|turned|turning|has gone|have gone|had gone|going|gone)\\b[^.?!;]{0,14}\\b(?:grey|gray)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_GREY_BETWEEN(py_str("(?:\\s+(?:a\\s+bit|a\\s+little|very|really|quite|all|pale|and|has\\s+gone|have\\s+gone|had\\s+gone|has\\s+turned|have\\s+turned|had\\s+turned|is\\s+going|are\\s+going|was\\s+going|were\\s+going|gone|going|goes|getting|looking|turning|turned|is|are|was|were|looks?|looked|went))*"));
    s_GREY_PEOPLE(mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitor(&g_FAMILY(), &py_set(vec![py_str("he"), py_str("she"), py_str("they"), py_str("hes"), py_str("shes"), py_str("everyone")]))); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s_GREY_PERSON_STATE(py_str("(?:\\s+(?:a\\s+bit|a\\s+little|very|really|quite|all|pale|and|has\\s+gone|have\\s+gone|had\\s+gone|has\\s+turned|have\\s+turned|had\\s+turned|gone|goes|went|is|are|was|were|looks?|looked|turned))*"));
    s_GREY_BODY(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:lips?|mouth|tongue|face|cheeks?|skin|fingers?|fingertips?|hands?|nails?|feet|foot|toes?|arms?|legs?|body)\\b"), &g_GREY_BETWEEN()), &py_str("\\s+(?:grey|gray)\\b|\\b(?:")), &g_GREY_PEOPLE()), &py_str(")\\b")), &g_GREY_PERSON_STATE()), &py_str("\\s+(?:grey|gray)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_TODAY(py_tuple(vec![py_str("confused"), py_str("drowsy"), py_str("very sleepy"), py_str("sleepy and"), py_str("temperature of 40"), py_str("40 degrees"), py_str("39 degrees"), py_str("39.5 degrees"), py_str("temperature of 39"), py_str("temperature of 39.5"), py_str("very hot to touch"), py_str("burning up"), py_str("hit his head"), py_str("hit her head"), py_str("hit their head"), py_str("banged his head"), py_str("banged her head"), py_str("fell and hit"), py_str("fell down the stairs"), py_str("wont stop bleeding"), py_str("wont stop vomit"), py_str("dehydrated"), py_str("not had a wee"), py_str("hasnt had a wee"), py_str("hasnt done a wee"), py_str("hasnt been for a wee"), py_str("no wet nappy"), py_str("dry nappy"), py_str("not passing urine")]));
    s_STUCK(stdlib_call("re", "compile", &[py_str("\\bstuck\\b[^.?!]{0,30}\\bthroat\\b|\\bthroat\\b[^.?!]{0,30}\\bstuck\\b|\\b(?:cant|can't|unable to|couldn't|couldnt) (?:talk|speak|breathe)\\b[^.?!]{0,25}\\bthroat\\b|\\bthroat\\b[^.?!]{0,25}\\b(?:cant|can't|couldn't|couldnt) (?:talk|speak|breathe)\\b|\\bcant get it out\\b|\\bcan't get it out\\b|\\bturning purple\\b|\\bgoing purple\\b|\\bgone purple\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_INGEST(call_builtin("frozenset", &[mcall(&py_str("\n    swallowed swallow swallowing ate eaten eating drank drunk drinking chewed chewing\n    licked sucking sucked gulped gulping sipped downed\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CHEMICAL(py_tuple(vec![py_str("washing tablet"), py_str("washing pod"), py_str("laundry tablet"), py_str("laundry pod"), py_str("dishwasher tablet"), py_str("detergent"), py_str("bleach"), py_str("antifreeze"), py_str("drain cleaner"), py_str("oven cleaner"), py_str("disinfectant"), py_str("weedkiller"), py_str("weed killer"), py_str("screenwash"), py_str("screen wash"), py_str("paint thinner"), py_str("methanol"), py_str("hand sanitiser"), py_str("hand sanitizer"), py_str("white spirit"), py_str("caustic"), py_str("paint"), py_str("air freshener"), py_str("perfume"), py_str("aftershave"), py_str("mouthwash"), py_str("nail polish"), py_str("nail varnish"), py_str("polish remover"), py_str("vape"), py_str("e-liquid"), py_str("eliquid"), py_str("e-cigarette"), py_str("lighter fluid"), py_str("petrol"), py_str("paraffin"), py_str("turps"), py_str("turpentine"), py_str("caustic soda")]));
    s_MEDICINE(call_builtin("frozenset", &[mcall(&py_str("\n    tablet tablets pill pills capsule capsules paracetamol ibuprofen aspirin medicine medicines\n    painkiller painkillers supplement supplements vitamins antidepressant antidepressants\n    methadone insulin codeine morphine steroids\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SMALL_OBJECT(py_tuple(vec![py_str("battery"), py_str("batteries"), py_str("magnet"), py_str("magnets"), py_str("coin"), py_str("marble"), py_str("bead"), py_str("button"), py_str("sim card"), py_str("screw"), py_str("drawing pin"), py_str("earring"), py_str("lego")]));
    s_AMBIGUOUS(py_tuple(vec![py_str("unresponsive"), py_str("not responding"), py_str("no response"), py_str("limp"), py_str("floppy"), py_str("fitting"), py_str("confused"), py_str("drowsy"), py_str("burning up"), py_str("very hot to touch")]));
    s_AMBIGUOUS_CUES(op_add(&g_AMBIGUOUS(), &py_tuple(vec![py_str("wont wake"), py_str("won't wake"), py_str("cant wake"), py_str("can't wake"), py_str("not waking"), py_str("isnt responding"), py_str("isn't responding"), py_str("isnt responsive"), py_str("isn't responsive"), py_str("not responsive")])));
    s_NOT_A_BODY(call_builtin("frozenset", &[mcall(&py_str("to by about from into of"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MANNER(call_builtin("frozenset", &[mcall(&py_str("well nicely easily fine ok okay perfectly together up out round around"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SETTLING(call_builtin("frozenset", &[mcall(&py_str("well nicely easily fine perfectly beautifully smoothly brilliantly"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CALENDAR(call_builtin("frozenset", &[mcall(&py_str("today yesterday lately recently nowadays"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SECOND_CLAUSE(stdlib_call("re", "compile", &[py_str("(?:but|and|then|so|because|yet|though|when)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SETTLING_PHRASES(stdlib_call("re", "compile", &[py_str("(?:at|in|into|with)\\s+(?:the\\s+|his\\s+|her\\s+|their\\s+|our\\s+|a\\s+)?(?:new\\s+)?(?:nursery|preschool|playgroup|school|class|clubs?|teams?|groups?|other\\s+kids|the\\s+others)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACE(call_builtin("frozenset", &[mcall(&py_str("\n    floor ground bed bath sofa settee stairs street garden ward car cot pram pushchair chair\n    table hospital home room arms lap arms shoulders knees chest hands\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OBJECT_WORD(call_builtin("frozenset", &[mcall(&py_str("\n    it them the a an my our his her their its this that these those new some carpet door floor\n    tiles kitchen bathroom wardrobe blind window shelves shelf units sink sofa curtains decor\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SYMPTOM(call_builtin("frozenset", &[mcall(&py_str("\n    fever temperature temp cold flu cough rash pain headache seizure shivers sweat sweats\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ORDINARY_STATE(call_builtin("frozenset", &[mcall(&py_str("\n    tired sleepy asleep hungry teething crying cross grumpy upset angry excited playing bored\n    told naughty tantrum tantrums whingey whinging\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DURATION(call_builtin("frozenset", &[mcall(&py_str("minute minutes min hour hours day days week weeks month months year years"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ONLINE(call_builtin("frozenset", &[mcall(&py_str("\n    chat whatsapp messenger facebook instagram twitter tiktok app forum thread threads messages\n    texts email emails phone phonecalls slack discord reddit linkedin zoom teams skype group\n    groups feed forum list mailing\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WAKE_CUES(py_tuple(vec![py_str("wont wake"), py_str("won't wake"), py_str("cant wake"), py_str("can't wake"), py_str("not waking")]));
    s_WAKE_OBJECT(op_bitor(&call_builtin("frozenset", &[py_set(vec![py_str("him"), py_str("her"), py_str("them"), py_str("it"), py_str("my"), py_str("your"), py_str("his"), py_str("our"), py_str("their"), py_str("the"), py_str("this"), py_str("that")])]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("mum"), py_str("mam"), py_str("mom"), py_str("mother"), py_str("dad"), py_str("father"), py_str("son"), py_str("daughter"), py_str("baby"), py_str("toddler"), py_str("child"), py_str("kid"), py_str("boy"), py_str("girl"), py_str("nan"), py_str("nana"), py_str("gran"), py_str("granny"), py_str("grandma"), py_str("grandad"), py_str("grandpa"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("brother"), py_str("sister"), py_str("friend"), py_str("uncle"), py_str("aunt"), py_str("niece"), py_str("nephew"), py_str("newborn"), py_str("patient")])));
    s_DEATH(py_tuple(vec![py_str("died"), py_str("dies"), py_str("dying"), py_str("passed away"), py_str("passed on"), py_str("funeral"), py_str("buried"), py_str("cremation"), py_str("bereave"), py_str("widow"), py_str("death"), py_str("killed")]));
    s_VERBY(py_tuple(vec![py_str("fitting"), py_str("burning up")]));
    s_OBJECTISH(stdlib_call("re", "compile", &[py_str("^\\s+(the|a|an|my|your|his|her|their|our|its|it|me|him|them|us|you|new|some|these|those|this|that|another|two|three|four|five|one|carpet|door|floor|tiles|kitchen|bathroom|wardrobe|blind|window|shelves|shelf|units|sink|sofa)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CONFUSED_SIGN(stdlib_call("re", "compile", &[py_str("\\b(suddenly|very|extremely|really|getting more|more)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CONFUSED_AFTER(stdlib_call("re", "compile", &[py_str("^\\s*(?:and|,|or)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TO_PAIN(py_tuple(vec![py_str("pain"), py_str("the pain"), py_str("any pain"), py_str("stimulus"), py_str("stimuli"), py_str("voice"), py_str("voices"), py_str("sound"), py_str("noise"), py_str("loud"), py_str("touch"), py_str("light"), py_str("nothing"), py_str("anything")]));
    s_SMALL_PERSON(call_builtin("frozenset", &[mcall(&py_str("\n    baby babies newborn infant toddler child kid son daughter boy girl little\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FEED(call_builtin("frozenset", &[mcall(&py_str("\n    milk formula feed feeds feeding bottle breast solids weaning\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOSE_BREATH(stdlib_call("re", "compile", &[py_str("\\b(?:can'?t|cannot|couldn'?t|could\\s+not|struggling|hard|unable|not|trouble)\\s+(?:to\\s+)?breath\\w*[^.?!]{0,18}?\\bthrough\\b[^.?!]{0,12}?\\bnose\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOSE_RED(py_tuple(vec![py_str("\\b(?:going|turning|gone|looks?)\\s+blue\\b"), py_str("\\bblue\\s+(?:lips|round\\s+the\\s+lips|in\\s+the\\s+face)\\b"), py_str("\\bgrey\\b"), py_str("\\bgray\\b"), py_str("\\bgrunt\\w*\\b"), py_str("\\bribs?\\b[^.?!]{0,25}\\b(?:suck\\w*|pulling|pulling\\s+in|drawing|recession)\\b"), py_str("\\b(?:pulling|sucking|drawing)\\s+(?:in|at)\\b[^.?!]{0,25}\\b(?:ribs?|chest)\\b"), py_str("\\bchest\\b[^.?!]{0,25}\\b(?:pulling|sucking|drawing)\\s+in\\b"), py_str("\\blimp\\b"), py_str("\\bfloppy\\b"), py_str("\\bunresponsive\\b"), py_str("\\bnot\\s+responding\\b"), py_str("\\bunconscious\\b"), py_str("\\bhard\\s+to\\s+(?:wake|rouse|arouse|waken)\\b"), py_str("\\bwont\\s+wake\\b"), py_str("\\bwon'?t\\s+wake\\b"), py_str("\\bnot\\s+waking\\b"), py_str("\\b(?:wouldnt|wouldn'?t|didnt|didn'?t|never)\\s+(?:wake|woke)\\b"), py_str("\\b(?:couldnt|couldn'?t)\\s+be\\s+woken\\b"), py_str("\\bisnt\\s+waking\\b"), py_str("\\bisn'?t\\s+waking\\b"), py_str("\\bout\\s+cold\\b"), py_str("\\bseizure\\b"), py_str("\\bfitting\\b"), py_str("\\bconvuls\\w*\\b"), py_str("\\b(?:had|has|having|with)\\s+(?:a|another)\\s+fit\\b"), py_str("\\bsnoring\\b"), py_str("\\bapnoea\\b"), py_str("\\bapnea\\b"), py_str("\\bnot\\s+breathing\\b"), py_str("\\bstops?\\s+breathing\\b"), py_str("\\bchoking\\b"), py_str("\\bchokes?\\b"), py_str("\\bcpr\\b"), py_str("\\bambulance\\b"), py_str("\\bhigh\\s+temperature\\b"), py_str("\\btemperature\\s+of\\s+\\d"), py_str("\\bburning\\s+up\\b")]));
    s_NOSE_RED_BESIDE(stdlib_call("re", "compile", &[mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOSE_RED()) { let mut pattern: Py = __item.clone(); __out.push(op_modulo(&py_str("(?:%s)"), &pattern.clone())); } py_list(__out) })() }])]).unwrap_or_else(|e| exc::raise(e)));
    s_FIT_OF(stdlib_call("re", "compile", &[py_str("\\bfits?\\s+of\\s+(?:cough\\w*|sneez\\w*|giggl\\w*|laughter|laughing|screaming|crying|the\\s+giggles|rage|temper|anger|pique|panic|inspiration)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FIT_ROUTINE(stdlib_call("re", "compile", &[py_str("\\b(?:every\\s+(?:few|couple\\s+of|so\\s+many)\\s+(?:months|weeks|years)|twice\\s+a\\s+year|once\\s+a\\s+year|a\\s+(?:few|couple)\\s+of\\s+times\\s+a\\s+year|now\\s+and\\s+(?:then|again)|occasionally|from\\s+time\\s+to\\s+time|keppra|epilep\\w*)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FIT_STILL_RINGS(py_tuple(vec![py_str("cant breathe"), py_str("can't breathe"), py_str("not breathing"), py_str("stopped breathing"), py_str("turning blue"), py_str("blue lips"), py_str("unconscious"), py_str("collapsed"), py_str("wont stop"), py_str("won't stop"), py_str("isnt stopping"), py_str("isn't stopping"), py_str("keeps going"), py_str("ambulance"), py_str("999"), py_str("on the floor"), py_str("shaking all over"), py_str("convulsing"), py_str("stiff")]));
    s_PRONOUN_SAFE(py_tuple(vec![py_str("had a fit"), py_str("has a fit"), py_str("having a fit"), py_str("having fits"), py_str("keeps fitting"), py_str("kept fitting"), py_str("keeps having fits"), py_str("kept having fits"), py_str("not breathing"), py_str("stopped breathing"), py_str("unconscious"), py_str("went limp"), py_str("gone limp"), py_str("limp in my arms"), py_str("limp in her arms"), py_str("limp in his arms"), py_str("no pulse"), py_str("seizure"), py_str("convulsing"), py_str("choking"), py_str("overdose"), py_str("took too many"), py_str("collapsed"), py_str("cardiac"), py_str("isnt breathing"), py_str("isn't breathing")]));
    s_SOFT_STATES(call_builtin("frozenset", &[py_set(vec![py_str("floppy"), py_str("limp"), py_str("confused"), py_str("drowsy"), py_str("burning up"), py_str("very hot to touch"), py_str("very sleepy"), py_str("sleepy and")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NORMAL_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:is that|is it|is this|should i|do i need to)\\b[^.?!]{0,40}\\b(?:normal|ok|okay|fine|worry|worried|worrying|serious|dangerous)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST_MARK(stdlib_call("re", "compile", &[py_str("\\b(?:ago|last (?:year|month|summer|winter|autumn|spring|week)|in (?:19|20)\\d\\d|years back|months back|when (?:i|he|she|they|it|we) (?:was|were) (?:a |an )?(?:baby|babies|toddler|tot|child|kid|teenager|born|little|small|young|tiny|at school|\\d+|one|two|three|four|five|six|seven|eight|nine|ten)|as a (?:baby|child|kid|toddler|teenager)|used to)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NEARLY(py_tuple(vec![py_str(" almost "), py_str(" nearly "), py_str(" like to have ")]));
    s_NOW_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:now|right now|still|since|today|tonight|this (?:morning|evening|afternoon)|at the moment|any ?more)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ALL_BETTER(stdlib_call("re", "compile", &[py_str("\\b(?:now|since|and)\\b[^.?!]{0,16}\\b(?:he|she|they|it|i|we)\\s*(?:is|are|was|were|'s)?\\s*(?:fine|ok|okay|better|recovered|alright|back to normal|himself again|herself again)\\b|\\b(?:now|since)\\s*(?:hes|shes|theyre|its|im)\\s*(?:fine|ok|okay|better|recovered|alright)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKING(py_tuple(vec![py_str(" about "), py_str(" what "), py_str(" why "), py_str(" how "), py_str(" should i"), py_str(" can i"), py_str(" is that "), py_str(" do i")]));
    s_NOW_REPLIES(py_tuple(vec![py_str("Don't wait on that: ring 999 or 911 now and stay on the line with them. Put them on their side, tilt the head back so the airway stays open, and don't leave them alone."), py_str("That needs an ambulance now. Ring 999 or 911, say exactly what you're seeing, and keep them on their side while you wait; if they stop breathing normally, start CPR and keep going until help comes."), py_str("Ring 999 or 911 straight away and don't try to drive them in yourself. Stay with them, check they're breathing, and follow what the call handler tells you.")]));
    s_POISON_NOW_REPLIES(py_tuple(vec![py_str("Ring 999 or 911 now and say what they swallowed and how long ago; keep the packet or the bottle to read out. Don't make them sick and don't give them anything to drink."), py_str("That's a 999 call now, and have the container ready to name to the handler. Keep them sitting up, wipe their mouth but don't rinse it, and don't try to make them vomit.")]));
    s_POISON_TODAY_REPLIES(py_tuple(vec![py_str("Ring 111 now rather than waiting, and have the packet in your hand: they will tell you whether they need to be seen. Don't make them sick. If they go sleepy, drool, cough or struggle to breathe, that's 999."), py_str("Get advice on that today - 111 or the poisons line, with the packet to hand - and watch them closely for the next few hours. Anything drowsy, vomiting, drooling or breathing oddly means 999.")]));
    s_TODAY_REPLIES(py_tuple(vec![py_str("Get that looked at today rather than waiting: ring 111 or your GP now, and tell them just what you told me. If they get drowsy, hard to wake, or their breathing changes, that's 999."), py_str("That's worth a call today - 111 or the doctor - and mention how it started. If they get harder to wake or go blue or floppy, ring 999 instead of waiting.")]));
    s_GAIT_BEFORE(call_builtin("frozenset", &[mcall(&py_str("a an the this that his her my your their our"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GAIT_STEM(stdlib_call("re", "compile", &[py_str("(?:ing|ed|s)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WAKE_OCCASION(call_builtin("frozenset", &[mcall(&py_str("for to at on before until by"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE_OF_SPEECH(py_tuple(vec![py_str("at the idea"), py_str("at the thought"), py_str("at the prospect"), py_str("at the mention")]));
    s_PETS(call_builtin("frozenset", &[mcall(&py_str("\n    dog dogs puppy puppies cat cats kitten kittens rabbit rabbits bunny hamster gerbil guinea pig\n    parrot budgie bird birds fish goldfish tortoise turtle snake lizard ferret pony horse horses\n    chinchilla\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_LAUGH_IDIOM(stdlib_call("re", "compile", &[py_str("\\b(?:collaps\\w*|went down|doubled over|fell about|fell over)\\s+(?:laughing|giggling|in (?:a )?(?:fit|stitches|hysterics)|with laugh\\w*|helpless)\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _not_a_body(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut rest: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rest") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rest"))) };
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { py_str("") };
    let mut first: Py = Py::None;
    let mut second: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut third: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when what follows the state word says the state is not about a body.");
    words = mcall(&rest.clone(), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    first = geti(&words.clone(), &Py::Int(0));
    second = if py_truthy(&op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { geti(&words.clone(), &Py::Int(1)) } else { py_str("") };
    third = if py_truthy(&op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) { geti(&words.clone(), &Py::Int(2)) } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&g_NOT_A_BODY(), &first.clone()))) {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&first.clone(), &py_str("to"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_TO_PAIN(), &second.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("the"), py_str("any"), py_str("some"), py_str("his"), py_str("her")]), &second.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_TO_PAIN(), &third.clone())) } } } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&first.clone(), &py_str("to"))); if !py_truthy(&__b) { __b } else { _fed_and_unresponsive(&[before.clone(), rest.clone()]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&g_SETTLING(), &mcall(&first.clone(), "strip", &[py_str(".,!?;:")])))) {
    tail = mcall(&mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)]), "strip", &[py_str(" .,!?;:")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&tail.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CALENDAR(), &tail.clone())) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_SECOND_CLAUSE(), "match", &[tail.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g_SETTLING_PHRASES(), "match", &[tail.clone()])]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&first.clone(), &py_str("together"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("my"), py_str("our"), py_str("his"), py_str("her"), py_str("their"), py_str("some"), py_str("these"), py_str("those"), py_str("this")]), &second.clone())) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("when"), py_str("whenever")]), &first.clone()))) {
    return call_builtin("bool", &[op_bitand(&call_builtin("set", &[slice_or_raise(&words.clone(), &Py::None, &Py::Int(6), &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &g_ORDINARY_STATE())]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&Py::Bool(py_eq(&first.clone(), &py_str("in")))) {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("with"), py_str("at")]), &second.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_MANNER(), &second.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_PLACE(), &second.clone())) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("the"), py_str("a"), py_str("an"), py_str("his"), py_str("her"), py_str("their"), py_str("my"), py_str("our")]), &second.clone()))) {
    return Py::Bool(!py_contains(&g_PLACE(), &third.clone()));
    }
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&first.clone(), &py_str("with")))) {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the")]), &second.clone()))) {
    return Py::Bool(!py_contains(&g_SYMPTOM(), &third.clone()));
    }
    return Py::Bool(py_contains(&g_OBJECT_WORD(), &second.clone()));
    }
    if py_truthy(&Py::Bool(py_eq(&first.clone(), &py_str("on")))) {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("the"), py_str("a"), py_str("an"), py_str("my"), py_str("his"), py_str("her"), py_str("their"), py_str("our"), py_str("this"), py_str("that")]), &second.clone()))) {
    return Py::Bool(py_contains(&g_ONLINE(), &third.clone()));
    }
    return Py::Bool(py_contains(&g_ONLINE(), &second.clone()));
    }
    if py_truthy(&Py::Bool(py_eq(&first.clone(), &py_str("for")))) {
    return { let __b = Py::Bool(py_contains(&g_DURATION(), &second.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("one"), py_str("two"), py_str("half")]), &second.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DURATION(), &third.clone())) } } } };
    }
    return Py::Bool(false);
    Py::None
}
pub fn _fed_and_unresponsive(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut before: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut rest: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rest") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rest"))) };
    let mut what: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("Whether this is a small person who will not respond to a feed.");
    who = slice_or_raise(&mcall(&stdlib_call("re", "sub", &[py_str("[^a-z ]"), py_str(" "), mcall(&before.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)), "split", &[]), &op_neg(&Py::Int(3)), &Py::None, &Py::None);
    what = slice_or_raise(&mcall(&stdlib_call("re", "sub", &[py_str("[^a-z ]"), py_str(" "), mcall(&rest.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)), "split", &[]), &Py::None, &Py::Int(4), &Py::None);
    return { let __b = call_builtin("bool", &[op_bitand(&call_builtin("set", &[who.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_SMALL_PERSON())]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[op_bitand(&call_builtin("set", &[what.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_A_FEED())]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn reads_as_a_story(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line puts the event firmly in the past, so no ambulance is needed now.\n\n    `he had a fit last year`, `my mum collapsed last year`, `my dad was unconscious for a minute\n    years ago` are stories. Something still going on (\"... and he is still on the floor now\"),\n    happening now (\"... and is unconscious now\") or said to have passed (\"... and now hes fine\")\n    is not a story, so the answer stays.\n    ");
    return { let __b = call_builtin("bool", &[mcall(&g_PAST_MARK(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_NOW_WORDS(), "search", &[clean.clone()]))); if py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_ALL_BETTER(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e)) } } } };
    Py::None
}
pub fn _named_a_gait(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut before: Py = Py::None;
    let _ = py_str("True when the word before the state word makes `limp` a leg rather than a body.");
    before = mcall(&slice_or_raise(&clean.clone(), &Py::None, &start.clone(), &Py::None), "split", &[]);
    return { let __b = call_builtin("bool", &[before.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_GAIT_BEFORE(), &geti(&before.clone(), &op_neg(&Py::Int(1))))) } };
    Py::None
}
pub fn _walking(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut rest: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rest") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rest"))) };
    let _ = py_str("True when what follows the `limp` cue is the rest of a walk: limping, limped, limps.");
    return call_builtin("bool", &[mcall(&g_GAIT_STEM(), "match", &[mcall(&rest.clone(), "lstrip", &[])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _signs_a_body(args: &[Py]) -> Py {
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
    let mut phrase: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut rest: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rest") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rest"))) };
    let mut start: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { Py::Int(0) };
    let mut phrase_: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether an ambiguous state word is about a body in this line.");
    if py_truthy(&Py::Bool(py_eq(&phrase.clone(), &py_str("limp")))) {
    return { let __b = Py::Bool(!py_truthy(&_named_a_gait(&[clean.clone(), start.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_walking(&[rest.clone()]))) } };
    }
    if py_truthy(&Py::Bool(py_contains(&g_WAKE_CUES(), &phrase.clone()))) {
    words = mcall(&rest.clone(), "split", &[]);
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&words.clone(), &Py::Int(0)), &py_str("up"))) } }) {
    words = slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&g_WAKE_OCCASION(), &geti(&words.clone(), &Py::Int(0))))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[op_bitand(&call_builtin("set", &[slice_or_raise(&words.clone(), &Py::None, &Py::Int(5), &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &g_WAKE_OBJECT())]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&Py::Bool(py_eq(&phrase.clone(), &py_str("burning up")))) {
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FIGURE_OF_SPEECH()) { let mut phrase_: Py = __item.clone(); __out.push(mcall(&mcall(&rest.clone(), "lstrip", &[]), "startswith", &[phrase_.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    }
    if py_truthy(&Py::Bool(py_eq(&phrase.clone(), &py_str("confused")))) {
    return call_builtin("bool", &[{ let __b = mcall(&g_CONFUSED_SIGN(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_CONFUSED_AFTER(), "match", &[rest.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    }
    return Py::Bool(true);
    Py::None
}
pub fn _first(args: &[Py]) -> Py {
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
    let mut phrases: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrases") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrases"))) };
    let mut phrase: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("The first cue the line really carries.\n\n    A cue from AMBIGUOUS_CUES is dropped when what follows it says it is not about a body:\n    `unresponsive to my texts` is a phone, `unresponsive to pain` is a body.\n    ");
    let __iter1 = py_iter_or_raise(&phrases.clone());
    for __item in __iter1 {
    phrase = __item.clone();
    start = mcall(&clean.clone(), "find", &[phrase.clone()]);
    while py_truthy(&op_ge(&start.clone(), &Py::Int(0))) {
    rest = slice_or_raise(&clean.clone(), &op_add(&start.clone(), &call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(!py_contains(&g_AMBIGUOUS_CUES(), &phrase.clone()))) {
    return phrase.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&_signs_a_body(&[clean.clone(), phrase.clone(), rest.clone(), start.clone()])))) {
    start = mcall(&clean.clone(), "find", &[phrase.clone(), op_add(&start.clone(), &Py::Int(1))]);
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_VERBY(), &phrase.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_OBJECTISH(), "match", &[rest.clone()]) } }) {
    start = mcall(&clean.clone(), "find", &[phrase.clone(), op_add(&start.clone(), &Py::Int(1))]);
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_not_a_body(&[rest.clone(), slice_or_raise(&clean.clone(), &Py::None, &start.clone(), &Py::None)])))) {
    return phrase.clone();
    }
    start = mcall(&clean.clone(), "find", &[phrase.clone(), op_add(&start.clone(), &Py::Int(1))]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _pet_is_the_subject(args: &[Py]) -> Py {
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
    let mut i: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    words = mcall(&g_WORD(), "findall", &[clean.clone()]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    i = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&g_FAMILY(), &word.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g_PETS(), &word.clone()))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
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
    else { Py::None };
    let mut ask: Py = Py::None;
    let mut blue: Py = Py::None;
    let mut c: Py = Py::None;
    let mut chem_text: Py = Py::None;
    let mut chemical: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut grey: Py = Py::None;
    let mut hedge: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut nearly: Py = Py::None;
    let mut now: Py = Py::None;
    let mut o: Py = Py::None;
    let mut options: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut poisoned: Py = Py::None;
    let mut pools: Py = Py::None;
    let mut pronoun_grey: Py = Py::None;
    let mut recent: Py = Py::None;
    let mut seed: Py = Py::None;
    let mut state: Py = Py::None;
    let mut swallowed: Py = Py::None;
    let mut t: Py = Py::None;
    let mut today: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A careful line when someone they look after is in trouble, else None.");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&g__dream(), &Py::None)); if !py_truthy(&__b) { __b } else { mcall(&g__dream(), "a_dream_is_not_happening", &[clean.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_NOSE_BREATH(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOSE_RED_BESIDE(), "search", &[clean.clone()]))) } }) {
    return Py::None;
    }
    words = call_builtin("set", &[mcall(&g_WORD(), "findall", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    pronoun_grey = { let __b = call_builtin("bool", &[op_bitand(&words.clone(), &py_set(vec![py_str("he"), py_str("she"), py_str("hes"), py_str("shes")]))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_GREY_BODY(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&op_bitand(&words.clone(), &g_FAMILY()))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&pronoun_grey.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = op_bitand(&words.clone(), &py_set(vec![py_str("he"), py_str("she")])); if !py_truthy(&__b) { __b } else { _first(&[clean.clone(), g_PRONOUN_SAFE()]) } })) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_PRO_ANSWERING(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_BODY_IN_THE_ROOM(), "search", &[clean.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&_pet_is_the_subject(&[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let __b = mcall(&g_FIT_OF(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_FIT_ROUTINE(), "search", &[clean.clone()]) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FIT_STILL_RINGS()) { let mut mark: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &mark.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_LAUGH_IDIOM(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DEATH()) { let mut mark: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &mark.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    nearly = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NEARLY()) { let mut hedge: Py = __item.clone(); __out.push(Py::Bool(py_contains(&op_add(&op_add(&py_str(" "), &clean.clone()), &py_str(" ")), &hedge.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    swallowed = call_builtin("bool", &[op_bitand(&words.clone(), &g_INGEST())]).unwrap_or_else(|e| exc::raise(e));
    chem_text = mcall(&mcall(&clean.clone(), "replace", &[py_str("paint ball"), py_str(" ")]), "replace", &[py_str("paintball"), py_str(" ")]);
    chemical = { let __b = swallowed.clone(); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CHEMICAL()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&chem_text.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    poisoned = { let __b = swallowed.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&chemical.clone())); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[op_bitand(&words.clone(), &g_MEDICINE())]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SMALL_OBJECT()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } } } };
    blue = { let __b = mcall(&g_BLUE_LOOSE(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_BLUE_BODY(), "search", &[clean.clone()]) } };
    grey = { let __b = mcall(&g_GREY_LOOSE(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_GREY_BODY(), "search", &[clean.clone()]) } };
    now = if py_truthy(&nearly.clone()) { Py::None } else { { let __b = _first(&[clean.clone(), g_NOW()]); if py_truthy(&__b) { __b } else { { let __b = if py_truthy(&blue.clone()) { py_str("is blue") } else { Py::None }; if py_truthy(&__b) { __b } else { { let __b = if py_truthy(&grey.clone()) { py_str("is grey") } else { Py::None }; if py_truthy(&__b) { __b } else { { let __b = if py_truthy(&mcall(&g_STUCK(), "search", &[clean.clone()])) { py_str("airway") } else { Py::None }; if py_truthy(&__b) { __b } else { if py_truthy(&chemical.clone()) { py_str("chemical") } else { Py::None } } } } } } } } } };
    if py_truthy(&{ let __b = now.clone(); if !py_truthy(&__b) { __b } else { reads_as_a_story(&[clean.clone()]) } }) {
    now = Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_SOFT_STATES(), &now.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_NORMAL_ASK(), "search", &[clean.clone()]) } }) {
    now = Py::None;
    }
    today = Py::None;
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&clean.clone(), &py_str("?"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ASKING()) { let mut ask: Py = __item.clone(); __out.push(Py::Bool(py_contains(&op_add(&op_add(&py_str(" "), &clean.clone()), &py_str(" ")), &ask.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    today = { let __b = _first(&[clean.clone(), g_TODAY()]); if py_truthy(&__b) { __b } else { if py_truthy(&poisoned.clone()) { py_str("swallowed") } else { Py::None } } };
    if py_truthy(&{ let __b = today.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_PAST_MARK(), "search", &[clean.clone()]) } }) {
    today = Py::None;
    }
    }
    pools = py_dict(vec![(py_str("chemical"), g_POISON_NOW_REPLIES()), (py_str("swallowed"), g_POISON_TODAY_REPLIES())]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![now.clone(), g_NOW_REPLIES()]), py_tuple(vec![today.clone(), g_TODAY_REPLIES()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    state = __unpk2[0].clone();
    options = __unpk2[1].clone();
    options = mcall(&pools.clone(), "get", &[state.clone(), options.clone()]);
    if py_truthy(&state.clone()) {
    turns = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    recent = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(5)), &Py::None, &Py::None)) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&t.clone(), "get", &[py_str("bot")])); } } py_set(__out) })() };
    fresh = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&recent.clone(), &o.clone()))) {  __out.push(o.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { call_builtin("list", &[options.clone()]).unwrap_or_else(|e| exc::raise(e)) } };
    seed = op_add(&op_mul(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7919)), &call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&clean.clone()) { let mut c: Py = __item.clone(); __out.push(call_builtin("ord", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    return mcall(&stdlib_call("random", "Random", &[seed.clone()]).unwrap_or_else(|e| exc::raise(e)), "choice", &[fresh.clone()]);
    }
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.urgent".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ALL_BETTER" => Some(g_ALL_BETTER()),
            "AMBIGUOUS" => Some(g_AMBIGUOUS()),
            "AMBIGUOUS_CUES" => Some(g_AMBIGUOUS_CUES()),
            "ASKING" => Some(g_ASKING()),
            "A_FEED" => Some(g_A_FEED()),
            "BLUE_BETWEEN" => Some(g_BLUE_BETWEEN()),
            "BLUE_BODY" => Some(g_BLUE_BODY()),
            "BLUE_LOOSE" => Some(g_BLUE_LOOSE()),
            "BODY_IN_THE_ROOM" => Some(g_BODY_IN_THE_ROOM()),
            "CALENDAR" => Some(g_CALENDAR()),
            "CHEMICAL" => Some(g_CHEMICAL()),
            "CONFUSED_AFTER" => Some(g_CONFUSED_AFTER()),
            "CONFUSED_SIGN" => Some(g_CONFUSED_SIGN()),
            "DEATH" => Some(g_DEATH()),
            "DURATION" => Some(g_DURATION()),
            "FAMILY" => Some(g_FAMILY()),
            "FIGURE_OF_SPEECH" => Some(g_FIGURE_OF_SPEECH()),
            "FIT_OF" => Some(g_FIT_OF()),
            "FIT_ROUTINE" => Some(g_FIT_ROUTINE()),
            "FIT_STILL_RINGS" => Some(g_FIT_STILL_RINGS()),
            "GAIT_BEFORE" => Some(g_GAIT_BEFORE()),
            "GAIT_STEM" => Some(g_GAIT_STEM()),
            "GREY_BETWEEN" => Some(g_GREY_BETWEEN()),
            "GREY_BODY" => Some(g_GREY_BODY()),
            "GREY_LOOSE" => Some(g_GREY_LOOSE()),
            "GREY_PEOPLE" => Some(g_GREY_PEOPLE()),
            "GREY_PERSON_STATE" => Some(g_GREY_PERSON_STATE()),
            "INGEST" => Some(g_INGEST()),
            "LAUGH_IDIOM" => Some(g_LAUGH_IDIOM()),
            "MANNER" => Some(g_MANNER()),
            "MEDICINE" => Some(g_MEDICINE()),
            "NEARLY" => Some(g_NEARLY()),
            "NORMAL_ASK" => Some(g_NORMAL_ASK()),
            "NOSE_BREATH" => Some(g_NOSE_BREATH()),
            "NOSE_RED" => Some(g_NOSE_RED()),
            "NOSE_RED_BESIDE" => Some(g_NOSE_RED_BESIDE()),
            "NOT_A_BODY" => Some(g_NOT_A_BODY()),
            "NOW" => Some(g_NOW()),
            "NOW_REPLIES" => Some(g_NOW_REPLIES()),
            "NOW_WORDS" => Some(g_NOW_WORDS()),
            "OBJECTISH" => Some(g_OBJECTISH()),
            "OBJECT_WORD" => Some(g_OBJECT_WORD()),
            "ONLINE" => Some(g_ONLINE()),
            "ORDINARY_STATE" => Some(g_ORDINARY_STATE()),
            "PAST_MARK" => Some(g_PAST_MARK()),
            "PETS" => Some(g_PETS()),
            "PLACE" => Some(g_PLACE()),
            "POISON_NOW_REPLIES" => Some(g_POISON_NOW_REPLIES()),
            "POISON_TODAY_REPLIES" => Some(g_POISON_TODAY_REPLIES()),
            "PRONOUN_SAFE" => Some(g_PRONOUN_SAFE()),
            "PRO_ANSWERING" => Some(g_PRO_ANSWERING()),
            "SECOND_CLAUSE" => Some(g_SECOND_CLAUSE()),
            "SETTLING" => Some(g_SETTLING()),
            "SETTLING_PHRASES" => Some(g_SETTLING_PHRASES()),
            "SMALL_OBJECT" => Some(g_SMALL_OBJECT()),
            "SMALL_PERSON" => Some(g_SMALL_PERSON()),
            "SOFT_STATES" => Some(g_SOFT_STATES()),
            "STUCK" => Some(g_STUCK()),
            "SYMPTOM" => Some(g_SYMPTOM()),
            "TODAY" => Some(g_TODAY()),
            "TODAY_REPLIES" => Some(g_TODAY_REPLIES()),
            "TO_PAIN" => Some(g_TO_PAIN()),
            "VERBY" => Some(g_VERBY()),
            "WAKE_CUES" => Some(g_WAKE_CUES()),
            "WAKE_OBJECT" => Some(g_WAKE_OBJECT()),
            "WAKE_OCCASION" => Some(g_WAKE_OCCASION()),
            "WORD" => Some(g_WORD()),
            "_dream" => Some(g__dream()),
            "_fed_and_unresponsive" => Some(py_func("_fed_and_unresponsive", |args: &[Py]| _fed_and_unresponsive(args))),
            "_first" => Some(py_func("_first", |args: &[Py]| _first(args))),
            "_named_a_gait" => Some(py_func("_named_a_gait", |args: &[Py]| _named_a_gait(args))),
            "_not_a_body" => Some(py_func("_not_a_body", |args: &[Py]| _not_a_body(args))),
            "_pet_is_the_subject" => Some(py_func("_pet_is_the_subject", |args: &[Py]| _pet_is_the_subject(args))),
            "_signs_a_body" => Some(py_func("_signs_a_body", |args: &[Py]| _signs_a_body(args))),
            "_walking" => Some(py_func("_walking", |args: &[Py]| _walking(args))),
            "reads_as_a_story" => Some(py_func("reads_as_a_story", |args: &[Py]| reads_as_a_story(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ALL_BETTER" => s_ALL_BETTER(v),
            "AMBIGUOUS" => s_AMBIGUOUS(v),
            "AMBIGUOUS_CUES" => s_AMBIGUOUS_CUES(v),
            "ASKING" => s_ASKING(v),
            "A_FEED" => s_A_FEED(v),
            "BLUE_BETWEEN" => s_BLUE_BETWEEN(v),
            "BLUE_BODY" => s_BLUE_BODY(v),
            "BLUE_LOOSE" => s_BLUE_LOOSE(v),
            "BODY_IN_THE_ROOM" => s_BODY_IN_THE_ROOM(v),
            "CALENDAR" => s_CALENDAR(v),
            "CHEMICAL" => s_CHEMICAL(v),
            "CONFUSED_AFTER" => s_CONFUSED_AFTER(v),
            "CONFUSED_SIGN" => s_CONFUSED_SIGN(v),
            "DEATH" => s_DEATH(v),
            "DURATION" => s_DURATION(v),
            "FAMILY" => s_FAMILY(v),
            "FIGURE_OF_SPEECH" => s_FIGURE_OF_SPEECH(v),
            "FIT_OF" => s_FIT_OF(v),
            "FIT_ROUTINE" => s_FIT_ROUTINE(v),
            "FIT_STILL_RINGS" => s_FIT_STILL_RINGS(v),
            "GAIT_BEFORE" => s_GAIT_BEFORE(v),
            "GAIT_STEM" => s_GAIT_STEM(v),
            "GREY_BETWEEN" => s_GREY_BETWEEN(v),
            "GREY_BODY" => s_GREY_BODY(v),
            "GREY_LOOSE" => s_GREY_LOOSE(v),
            "GREY_PEOPLE" => s_GREY_PEOPLE(v),
            "GREY_PERSON_STATE" => s_GREY_PERSON_STATE(v),
            "INGEST" => s_INGEST(v),
            "LAUGH_IDIOM" => s_LAUGH_IDIOM(v),
            "MANNER" => s_MANNER(v),
            "MEDICINE" => s_MEDICINE(v),
            "NEARLY" => s_NEARLY(v),
            "NORMAL_ASK" => s_NORMAL_ASK(v),
            "NOSE_BREATH" => s_NOSE_BREATH(v),
            "NOSE_RED" => s_NOSE_RED(v),
            "NOSE_RED_BESIDE" => s_NOSE_RED_BESIDE(v),
            "NOT_A_BODY" => s_NOT_A_BODY(v),
            "NOW" => s_NOW(v),
            "NOW_REPLIES" => s_NOW_REPLIES(v),
            "NOW_WORDS" => s_NOW_WORDS(v),
            "OBJECTISH" => s_OBJECTISH(v),
            "OBJECT_WORD" => s_OBJECT_WORD(v),
            "ONLINE" => s_ONLINE(v),
            "ORDINARY_STATE" => s_ORDINARY_STATE(v),
            "PAST_MARK" => s_PAST_MARK(v),
            "PETS" => s_PETS(v),
            "PLACE" => s_PLACE(v),
            "POISON_NOW_REPLIES" => s_POISON_NOW_REPLIES(v),
            "POISON_TODAY_REPLIES" => s_POISON_TODAY_REPLIES(v),
            "PRONOUN_SAFE" => s_PRONOUN_SAFE(v),
            "PRO_ANSWERING" => s_PRO_ANSWERING(v),
            "SECOND_CLAUSE" => s_SECOND_CLAUSE(v),
            "SETTLING" => s_SETTLING(v),
            "SETTLING_PHRASES" => s_SETTLING_PHRASES(v),
            "SMALL_OBJECT" => s_SMALL_OBJECT(v),
            "SMALL_PERSON" => s_SMALL_PERSON(v),
            "SOFT_STATES" => s_SOFT_STATES(v),
            "STUCK" => s_STUCK(v),
            "SYMPTOM" => s_SYMPTOM(v),
            "TODAY" => s_TODAY(v),
            "TODAY_REPLIES" => s_TODAY_REPLIES(v),
            "TO_PAIN" => s_TO_PAIN(v),
            "VERBY" => s_VERBY(v),
            "WAKE_CUES" => s_WAKE_CUES(v),
            "WAKE_OBJECT" => s_WAKE_OBJECT(v),
            "WAKE_OCCASION" => s_WAKE_OCCASION(v),
            "WORD" => s_WORD(v),
            "_dream" => s__dream(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.urgent", module_obj);
    pyrt::xrefs::reg_fn("compose.urgent", "_not_a_body", py_func("_not_a_body", |args: &[Py]| _not_a_body(args)));
    pyrt::xrefs::reg_fn("compose.urgent", "_fed_and_unresponsive", py_func("_fed_and_unresponsive", |args: &[Py]| _fed_and_unresponsive(args)));
    pyrt::xrefs::reg_fn("compose.urgent", "reads_as_a_story", py_func("reads_as_a_story", |args: &[Py]| reads_as_a_story(args)));
    pyrt::xrefs::reg_fn("compose.urgent", "_named_a_gait", py_func("_named_a_gait", |args: &[Py]| _named_a_gait(args)));
    pyrt::xrefs::reg_fn("compose.urgent", "_walking", py_func("_walking", |args: &[Py]| _walking(args)));
    pyrt::xrefs::reg_fn("compose.urgent", "_signs_a_body", py_func("_signs_a_body", |args: &[Py]| _signs_a_body(args)));
    pyrt::xrefs::reg_fn("compose.urgent", "_first", py_func("_first", |args: &[Py]| _first(args)));
    pyrt::xrefs::reg_fn("compose.urgent", "_pet_is_the_subject", py_func("_pet_is_the_subject", |args: &[Py]| _pet_is_the_subject(args)));
    pyrt::xrefs::reg_fn("compose.urgent", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.urgent", "ALL_BETTER", g_ALL_BETTER, s_ALL_BETTER);
    pyrt::xrefs::reg_access("compose.urgent", "AMBIGUOUS", g_AMBIGUOUS, s_AMBIGUOUS);
    pyrt::xrefs::reg_access("compose.urgent", "AMBIGUOUS_CUES", g_AMBIGUOUS_CUES, s_AMBIGUOUS_CUES);
    pyrt::xrefs::reg_access("compose.urgent", "ASKING", g_ASKING, s_ASKING);
    pyrt::xrefs::reg_access("compose.urgent", "A_FEED", g_A_FEED, s_A_FEED);
    pyrt::xrefs::reg_access("compose.urgent", "BLUE_BETWEEN", g_BLUE_BETWEEN, s_BLUE_BETWEEN);
    pyrt::xrefs::reg_access("compose.urgent", "BLUE_BODY", g_BLUE_BODY, s_BLUE_BODY);
    pyrt::xrefs::reg_access("compose.urgent", "BLUE_LOOSE", g_BLUE_LOOSE, s_BLUE_LOOSE);
    pyrt::xrefs::reg_access("compose.urgent", "BODY_IN_THE_ROOM", g_BODY_IN_THE_ROOM, s_BODY_IN_THE_ROOM);
    pyrt::xrefs::reg_access("compose.urgent", "CALENDAR", g_CALENDAR, s_CALENDAR);
    pyrt::xrefs::reg_access("compose.urgent", "CHEMICAL", g_CHEMICAL, s_CHEMICAL);
    pyrt::xrefs::reg_access("compose.urgent", "CONFUSED_AFTER", g_CONFUSED_AFTER, s_CONFUSED_AFTER);
    pyrt::xrefs::reg_access("compose.urgent", "CONFUSED_SIGN", g_CONFUSED_SIGN, s_CONFUSED_SIGN);
    pyrt::xrefs::reg_access("compose.urgent", "DEATH", g_DEATH, s_DEATH);
    pyrt::xrefs::reg_access("compose.urgent", "DURATION", g_DURATION, s_DURATION);
    pyrt::xrefs::reg_access("compose.urgent", "FAMILY", g_FAMILY, s_FAMILY);
    pyrt::xrefs::reg_access("compose.urgent", "FIGURE_OF_SPEECH", g_FIGURE_OF_SPEECH, s_FIGURE_OF_SPEECH);
    pyrt::xrefs::reg_access("compose.urgent", "FIT_OF", g_FIT_OF, s_FIT_OF);
    pyrt::xrefs::reg_access("compose.urgent", "FIT_ROUTINE", g_FIT_ROUTINE, s_FIT_ROUTINE);
    pyrt::xrefs::reg_access("compose.urgent", "FIT_STILL_RINGS", g_FIT_STILL_RINGS, s_FIT_STILL_RINGS);
    pyrt::xrefs::reg_access("compose.urgent", "GAIT_BEFORE", g_GAIT_BEFORE, s_GAIT_BEFORE);
    pyrt::xrefs::reg_access("compose.urgent", "GAIT_STEM", g_GAIT_STEM, s_GAIT_STEM);
    pyrt::xrefs::reg_access("compose.urgent", "GREY_BETWEEN", g_GREY_BETWEEN, s_GREY_BETWEEN);
    pyrt::xrefs::reg_access("compose.urgent", "GREY_BODY", g_GREY_BODY, s_GREY_BODY);
    pyrt::xrefs::reg_access("compose.urgent", "GREY_LOOSE", g_GREY_LOOSE, s_GREY_LOOSE);
    pyrt::xrefs::reg_access("compose.urgent", "GREY_PEOPLE", g_GREY_PEOPLE, s_GREY_PEOPLE);
    pyrt::xrefs::reg_access("compose.urgent", "GREY_PERSON_STATE", g_GREY_PERSON_STATE, s_GREY_PERSON_STATE);
    pyrt::xrefs::reg_access("compose.urgent", "INGEST", g_INGEST, s_INGEST);
    pyrt::xrefs::reg_access("compose.urgent", "LAUGH_IDIOM", g_LAUGH_IDIOM, s_LAUGH_IDIOM);
    pyrt::xrefs::reg_access("compose.urgent", "MANNER", g_MANNER, s_MANNER);
    pyrt::xrefs::reg_access("compose.urgent", "MEDICINE", g_MEDICINE, s_MEDICINE);
    pyrt::xrefs::reg_access("compose.urgent", "NEARLY", g_NEARLY, s_NEARLY);
    pyrt::xrefs::reg_access("compose.urgent", "NORMAL_ASK", g_NORMAL_ASK, s_NORMAL_ASK);
    pyrt::xrefs::reg_access("compose.urgent", "NOSE_BREATH", g_NOSE_BREATH, s_NOSE_BREATH);
    pyrt::xrefs::reg_access("compose.urgent", "NOSE_RED", g_NOSE_RED, s_NOSE_RED);
    pyrt::xrefs::reg_access("compose.urgent", "NOSE_RED_BESIDE", g_NOSE_RED_BESIDE, s_NOSE_RED_BESIDE);
    pyrt::xrefs::reg_access("compose.urgent", "NOT_A_BODY", g_NOT_A_BODY, s_NOT_A_BODY);
    pyrt::xrefs::reg_access("compose.urgent", "NOW", g_NOW, s_NOW);
    pyrt::xrefs::reg_access("compose.urgent", "NOW_REPLIES", g_NOW_REPLIES, s_NOW_REPLIES);
    pyrt::xrefs::reg_access("compose.urgent", "NOW_WORDS", g_NOW_WORDS, s_NOW_WORDS);
    pyrt::xrefs::reg_access("compose.urgent", "OBJECTISH", g_OBJECTISH, s_OBJECTISH);
    pyrt::xrefs::reg_access("compose.urgent", "OBJECT_WORD", g_OBJECT_WORD, s_OBJECT_WORD);
    pyrt::xrefs::reg_access("compose.urgent", "ONLINE", g_ONLINE, s_ONLINE);
    pyrt::xrefs::reg_access("compose.urgent", "ORDINARY_STATE", g_ORDINARY_STATE, s_ORDINARY_STATE);
    pyrt::xrefs::reg_access("compose.urgent", "PAST_MARK", g_PAST_MARK, s_PAST_MARK);
    pyrt::xrefs::reg_access("compose.urgent", "PETS", g_PETS, s_PETS);
    pyrt::xrefs::reg_access("compose.urgent", "PLACE", g_PLACE, s_PLACE);
    pyrt::xrefs::reg_access("compose.urgent", "POISON_NOW_REPLIES", g_POISON_NOW_REPLIES, s_POISON_NOW_REPLIES);
    pyrt::xrefs::reg_access("compose.urgent", "POISON_TODAY_REPLIES", g_POISON_TODAY_REPLIES, s_POISON_TODAY_REPLIES);
    pyrt::xrefs::reg_access("compose.urgent", "PRONOUN_SAFE", g_PRONOUN_SAFE, s_PRONOUN_SAFE);
    pyrt::xrefs::reg_access("compose.urgent", "PRO_ANSWERING", g_PRO_ANSWERING, s_PRO_ANSWERING);
    pyrt::xrefs::reg_access("compose.urgent", "SECOND_CLAUSE", g_SECOND_CLAUSE, s_SECOND_CLAUSE);
    pyrt::xrefs::reg_access("compose.urgent", "SETTLING", g_SETTLING, s_SETTLING);
    pyrt::xrefs::reg_access("compose.urgent", "SETTLING_PHRASES", g_SETTLING_PHRASES, s_SETTLING_PHRASES);
    pyrt::xrefs::reg_access("compose.urgent", "SMALL_OBJECT", g_SMALL_OBJECT, s_SMALL_OBJECT);
    pyrt::xrefs::reg_access("compose.urgent", "SMALL_PERSON", g_SMALL_PERSON, s_SMALL_PERSON);
    pyrt::xrefs::reg_access("compose.urgent", "SOFT_STATES", g_SOFT_STATES, s_SOFT_STATES);
    pyrt::xrefs::reg_access("compose.urgent", "STUCK", g_STUCK, s_STUCK);
    pyrt::xrefs::reg_access("compose.urgent", "SYMPTOM", g_SYMPTOM, s_SYMPTOM);
    pyrt::xrefs::reg_access("compose.urgent", "TODAY", g_TODAY, s_TODAY);
    pyrt::xrefs::reg_access("compose.urgent", "TODAY_REPLIES", g_TODAY_REPLIES, s_TODAY_REPLIES);
    pyrt::xrefs::reg_access("compose.urgent", "TO_PAIN", g_TO_PAIN, s_TO_PAIN);
    pyrt::xrefs::reg_access("compose.urgent", "VERBY", g_VERBY, s_VERBY);
    pyrt::xrefs::reg_access("compose.urgent", "WAKE_CUES", g_WAKE_CUES, s_WAKE_CUES);
    pyrt::xrefs::reg_access("compose.urgent", "WAKE_OBJECT", g_WAKE_OBJECT, s_WAKE_OBJECT);
    pyrt::xrefs::reg_access("compose.urgent", "WAKE_OCCASION", g_WAKE_OCCASION, s_WAKE_OCCASION);
    pyrt::xrefs::reg_access("compose.urgent", "WORD", g_WORD, s_WORD);
    pyrt::xrefs::reg_access("compose.urgent", "_dream", g__dream, s__dream);
}

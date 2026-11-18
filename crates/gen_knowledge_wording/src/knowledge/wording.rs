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


thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BEHAVIOUR_TOWARD_THE_ASKER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_JOB_LEFT_UNDONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHASE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMFORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMFORT_LAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMFORT_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMFORT_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMFORT_SITUATIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMFORT_UNNAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EARLY_SCRIPT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EARLY_SCRIPT_YES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EARLY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOLLOW_UP_AFTER_INTERVIEW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOLLOW_UP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOLLOW_UP_CHASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOLLOW_UP_THANKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOLLOW_UP_WHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INTERVIEW_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAVING_SPEECH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MACHINE_SOUNDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOURNING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_DOG_NOISE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NERVES_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOISE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_THE_PHONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SIGNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLIPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPELLINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TALKING_IS_NOT_A_CHAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THANKS_ANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THANKS_FAVOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THANKS_FLOWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THANKS_GIFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THANKS_LIFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THANKS_MEAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THANKS_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THANKS_STAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THANKS_WRITTEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_OFF_SCRIPT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_OFF_SCRIPT_YES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOLD_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YES_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ALREADY_HAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKING_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_FOR_THE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_TO_HAVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASK_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AWAY_OR_HOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CHILD_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CLOSE_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_COLLEAGUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BABY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BEING_WRITTEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EULOGY_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EULOGY_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HAS_THE_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOUSEHOLD_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOW_LONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HURT_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONTHS_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MOVING_AWAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MOVING_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MY_OWN_SCREENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_SIMPLE_NO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NO_FUSS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ONLY_SAID_ABOUT_A_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PARENT_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RAISE_ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RAISE_ASK_SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RAISE_CAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RAISE_CAT_OUTSIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RAISE_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RAISE_EXAMPLES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RAISE_HOUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RAISE_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RAISE_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RAISE_SMOKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAID_IT_IS_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHARED_HOUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHARE_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SIGN_PLURAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEBODY_AT_HOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEONE_HURT_A_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPELLING_ENTRY_IDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPELLING_SLIPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPELLING_TIPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPELLING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TALKS_OVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THEIR_SCREENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TRADES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__USE_THE_WEEK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__slip: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_owns_the_deed: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_the_fault: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.wording: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKING() -> Py { ensure_init(); G_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING(v: Py) { ensure_init(); G_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BEHAVIOUR_TOWARD_THE_ASKER() -> Py { ensure_init(); G_A_BEHAVIOUR_TOWARD_THE_ASKER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BEHAVIOUR_TOWARD_THE_ASKER(v: Py) { ensure_init(); G_A_BEHAVIOUR_TOWARD_THE_ASKER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_JOB_LEFT_UNDONE() -> Py { ensure_init(); G_A_JOB_LEFT_UNDONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_JOB_LEFT_UNDONE(v: Py) { ensure_init(); G_A_JOB_LEFT_UNDONE.with(|c| *c.borrow_mut() = v); }
pub fn g_CHASE_WORDS() -> Py { ensure_init(); G_CHASE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHASE_WORDS(v: Py) { ensure_init(); G_CHASE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_COMFORT() -> Py { ensure_init(); G_COMFORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMFORT(v: Py) { ensure_init(); G_COMFORT.with(|c| *c.borrow_mut() = v); }
pub fn g_COMFORT_LAST() -> Py { ensure_init(); G_COMFORT_LAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMFORT_LAST(v: Py) { ensure_init(); G_COMFORT_LAST.with(|c| *c.borrow_mut() = v); }
pub fn g_COMFORT_PLAIN() -> Py { ensure_init(); G_COMFORT_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMFORT_PLAIN(v: Py) { ensure_init(); G_COMFORT_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_COMFORT_SAY() -> Py { ensure_init(); G_COMFORT_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMFORT_SAY(v: Py) { ensure_init(); G_COMFORT_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_COMFORT_SITUATIONS() -> Py { ensure_init(); G_COMFORT_SITUATIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMFORT_SITUATIONS(v: Py) { ensure_init(); G_COMFORT_SITUATIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_COMFORT_UNNAMED() -> Py { ensure_init(); G_COMFORT_UNNAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMFORT_UNNAMED(v: Py) { ensure_init(); G_COMFORT_UNNAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_EARLY_SCRIPT() -> Py { ensure_init(); G_EARLY_SCRIPT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EARLY_SCRIPT(v: Py) { ensure_init(); G_EARLY_SCRIPT.with(|c| *c.borrow_mut() = v); }
pub fn g_EARLY_SCRIPT_YES() -> Py { ensure_init(); G_EARLY_SCRIPT_YES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EARLY_SCRIPT_YES(v: Py) { ensure_init(); G_EARLY_SCRIPT_YES.with(|c| *c.borrow_mut() = v); }
pub fn g_EARLY_WORDS() -> Py { ensure_init(); G_EARLY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EARLY_WORDS(v: Py) { ensure_init(); G_EARLY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_FOLLOW_UP_AFTER_INTERVIEW() -> Py { ensure_init(); G_FOLLOW_UP_AFTER_INTERVIEW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOLLOW_UP_AFTER_INTERVIEW(v: Py) { ensure_init(); G_FOLLOW_UP_AFTER_INTERVIEW.with(|c| *c.borrow_mut() = v); }
pub fn g_FOLLOW_UP_ASK() -> Py { ensure_init(); G_FOLLOW_UP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOLLOW_UP_ASK(v: Py) { ensure_init(); G_FOLLOW_UP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_FOLLOW_UP_CHASE() -> Py { ensure_init(); G_FOLLOW_UP_CHASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOLLOW_UP_CHASE(v: Py) { ensure_init(); G_FOLLOW_UP_CHASE.with(|c| *c.borrow_mut() = v); }
pub fn g_FOLLOW_UP_THANKS() -> Py { ensure_init(); G_FOLLOW_UP_THANKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOLLOW_UP_THANKS(v: Py) { ensure_init(); G_FOLLOW_UP_THANKS.with(|c| *c.borrow_mut() = v); }
pub fn g_FOLLOW_UP_WHEN() -> Py { ensure_init(); G_FOLLOW_UP_WHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOLLOW_UP_WHEN(v: Py) { ensure_init(); G_FOLLOW_UP_WHEN.with(|c| *c.borrow_mut() = v); }
pub fn g_INTERVIEW_WORDS() -> Py { ensure_init(); G_INTERVIEW_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INTERVIEW_WORDS(v: Py) { ensure_init(); G_INTERVIEW_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAVING_SPEECH() -> Py { ensure_init(); G_LEAVING_SPEECH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAVING_SPEECH(v: Py) { ensure_init(); G_LEAVING_SPEECH.with(|c| *c.borrow_mut() = v); }
pub fn g_MACHINE_SOUNDS() -> Py { ensure_init(); G_MACHINE_SOUNDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MACHINE_SOUNDS(v: Py) { ensure_init(); G_MACHINE_SOUNDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MOURNING() -> Py { ensure_init(); G_MOURNING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOURNING(v: Py) { ensure_init(); G_MOURNING.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_DOG_NOISE() -> Py { ensure_init(); G_MY_DOG_NOISE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_DOG_NOISE(v: Py) { ensure_init(); G_MY_DOG_NOISE.with(|c| *c.borrow_mut() = v); }
pub fn g_NERVES_BODY() -> Py { ensure_init(); G_NERVES_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NERVES_BODY(v: Py) { ensure_init(); G_NERVES_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOISE_WORDS() -> Py { ensure_init(); G_NOISE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOISE_WORDS(v: Py) { ensure_init(); G_NOISE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_JOB() -> Py { ensure_init(); G_NOT_A_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_JOB(v: Py) { ensure_init(); G_NOT_A_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_THE_PHONE() -> Py { ensure_init(); G_ON_THE_PHONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_THE_PHONE(v: Py) { ensure_init(); G_ON_THE_PHONE.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID() -> Py { ensure_init(); G_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID(v: Py) { ensure_init(); G_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_SAY() -> Py { ensure_init(); G_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAY(v: Py) { ensure_init(); G_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_SIGNS() -> Py { ensure_init(); G_SIGNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SIGNS(v: Py) { ensure_init(); G_SIGNS.with(|c| *c.borrow_mut() = v); }
pub fn g_SLIPS() -> Py { ensure_init(); G_SLIPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SLIPS(v: Py) { ensure_init(); G_SLIPS.with(|c| *c.borrow_mut() = v); }
pub fn g_SPELLINGS() -> Py { ensure_init(); G_SPELLINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPELLINGS(v: Py) { ensure_init(); G_SPELLINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_TALKING_IS_NOT_A_CHAT() -> Py { ensure_init(); G_TALKING_IS_NOT_A_CHAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TALKING_IS_NOT_A_CHAT(v: Py) { ensure_init(); G_TALKING_IS_NOT_A_CHAT.with(|c| *c.borrow_mut() = v); }
pub fn g_THANKS_ANY() -> Py { ensure_init(); G_THANKS_ANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THANKS_ANY(v: Py) { ensure_init(); G_THANKS_ANY.with(|c| *c.borrow_mut() = v); }
pub fn g_THANKS_FAVOUR() -> Py { ensure_init(); G_THANKS_FAVOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THANKS_FAVOUR(v: Py) { ensure_init(); G_THANKS_FAVOUR.with(|c| *c.borrow_mut() = v); }
pub fn g_THANKS_FLOWERS() -> Py { ensure_init(); G_THANKS_FLOWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THANKS_FLOWERS(v: Py) { ensure_init(); G_THANKS_FLOWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_THANKS_GIFT() -> Py { ensure_init(); G_THANKS_GIFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THANKS_GIFT(v: Py) { ensure_init(); G_THANKS_GIFT.with(|c| *c.borrow_mut() = v); }
pub fn g_THANKS_LIFT() -> Py { ensure_init(); G_THANKS_LIFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THANKS_LIFT(v: Py) { ensure_init(); G_THANKS_LIFT.with(|c| *c.borrow_mut() = v); }
pub fn g_THANKS_MEAL() -> Py { ensure_init(); G_THANKS_MEAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THANKS_MEAL(v: Py) { ensure_init(); G_THANKS_MEAL.with(|c| *c.borrow_mut() = v); }
pub fn g_THANKS_PERSON() -> Py { ensure_init(); G_THANKS_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THANKS_PERSON(v: Py) { ensure_init(); G_THANKS_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_THANKS_STAY() -> Py { ensure_init(); G_THANKS_STAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THANKS_STAY(v: Py) { ensure_init(); G_THANKS_STAY.with(|c| *c.borrow_mut() = v); }
pub fn g_THANKS_WRITTEN() -> Py { ensure_init(); G_THANKS_WRITTEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THANKS_WRITTEN(v: Py) { ensure_init(); G_THANKS_WRITTEN.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_OFF_SCRIPT() -> Py { ensure_init(); G_TIME_OFF_SCRIPT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_OFF_SCRIPT(v: Py) { ensure_init(); G_TIME_OFF_SCRIPT.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_OFF_SCRIPT_YES() -> Py { ensure_init(); G_TIME_OFF_SCRIPT_YES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_OFF_SCRIPT_YES(v: Py) { ensure_init(); G_TIME_OFF_SCRIPT_YES.with(|c| *c.borrow_mut() = v); }
pub fn g_TOLD_OFF() -> Py { ensure_init(); G_TOLD_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOLD_OFF(v: Py) { ensure_init(); G_TOLD_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_WORDS() -> Py { ensure_init(); G_WHEN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_WORDS(v: Py) { ensure_init(); G_WHEN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_YES_WORDS() -> Py { ensure_init(); G_YES_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YES_WORDS(v: Py) { ensure_init(); G_YES_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__ALREADY_HAD() -> Py { ensure_init(); G__ALREADY_HAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ALREADY_HAD(v: Py) { ensure_init(); G__ALREADY_HAD.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKING_FRAME() -> Py { ensure_init(); G__ASKING_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKING_FRAME(v: Py) { ensure_init(); G__ASKING_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_FOR_THE_WORDS() -> Py { ensure_init(); G__ASKS_FOR_THE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_FOR_THE_WORDS(v: Py) { ensure_init(); G__ASKS_FOR_THE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_TO_HAVE() -> Py { ensure_init(); G__ASKS_TO_HAVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_TO_HAVE(v: Py) { ensure_init(); G__ASKS_TO_HAVE.with(|c| *c.borrow_mut() = v); }
pub fn g__ASK_VERBS() -> Py { ensure_init(); G__ASK_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASK_VERBS(v: Py) { ensure_init(); G__ASK_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g__AWAY_OR_HOME() -> Py { ensure_init(); G__AWAY_OR_HOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AWAY_OR_HOME(v: Py) { ensure_init(); G__AWAY_OR_HOME.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CHILD_NAMED() -> Py { ensure_init(); G__A_CHILD_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CHILD_NAMED(v: Py) { ensure_init(); G__A_CHILD_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CLOSE_ONE() -> Py { ensure_init(); G__A_CLOSE_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CLOSE_ONE(v: Py) { ensure_init(); G__A_CLOSE_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_COLLEAGUE() -> Py { ensure_init(); G__A_COLLEAGUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_COLLEAGUE(v: Py) { ensure_init(); G__A_COLLEAGUE.with(|c| *c.borrow_mut() = v); }
pub fn g__BABY() -> Py { ensure_init(); G__BABY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BABY(v: Py) { ensure_init(); G__BABY.with(|c| *c.borrow_mut() = v); }
pub fn g__BEING_WRITTEN() -> Py { ensure_init(); G__BEING_WRITTEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BEING_WRITTEN(v: Py) { ensure_init(); G__BEING_WRITTEN.with(|c| *c.borrow_mut() = v); }
pub fn g__EULOGY_LENGTH() -> Py { ensure_init(); G__EULOGY_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EULOGY_LENGTH(v: Py) { ensure_init(); G__EULOGY_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g__EULOGY_START() -> Py { ensure_init(); G__EULOGY_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EULOGY_START(v: Py) { ensure_init(); G__EULOGY_START.with(|c| *c.borrow_mut() = v); }
pub fn g__HAS_THE_TIME() -> Py { ensure_init(); G__HAS_THE_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HAS_THE_TIME(v: Py) { ensure_init(); G__HAS_THE_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g__HOUSEHOLD_MONEY() -> Py { ensure_init(); G__HOUSEHOLD_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOUSEHOLD_MONEY(v: Py) { ensure_init(); G__HOUSEHOLD_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g__HOW_LONG() -> Py { ensure_init(); G__HOW_LONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOW_LONG(v: Py) { ensure_init(); G__HOW_LONG.with(|c| *c.borrow_mut() = v); }
pub fn g__HURT_PART() -> Py { ensure_init(); G__HURT_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HURT_PART(v: Py) { ensure_init(); G__HURT_PART.with(|c| *c.borrow_mut() = v); }
pub fn g__MESS() -> Py { ensure_init(); G__MESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MESS(v: Py) { ensure_init(); G__MESS.with(|c| *c.borrow_mut() = v); }
pub fn g__MONEY() -> Py { ensure_init(); G__MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONEY(v: Py) { ensure_init(); G__MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g__MONTHS_NAMED() -> Py { ensure_init(); G__MONTHS_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONTHS_NAMED(v: Py) { ensure_init(); G__MONTHS_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g__MOVING_AWAY() -> Py { ensure_init(); G__MOVING_AWAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MOVING_AWAY(v: Py) { ensure_init(); G__MOVING_AWAY.with(|c| *c.borrow_mut() = v); }
pub fn g__MOVING_OUT() -> Py { ensure_init(); G__MOVING_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MOVING_OUT(v: Py) { ensure_init(); G__MOVING_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g__MY_OWN_SCREENS() -> Py { ensure_init(); G__MY_OWN_SCREENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MY_OWN_SCREENS(v: Py) { ensure_init(); G__MY_OWN_SCREENS.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_SIMPLE_NO() -> Py { ensure_init(); G__NOT_A_SIMPLE_NO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_SIMPLE_NO(v: Py) { ensure_init(); G__NOT_A_SIMPLE_NO.with(|c| *c.borrow_mut() = v); }
pub fn g__NO_FUSS() -> Py { ensure_init(); G__NO_FUSS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NO_FUSS(v: Py) { ensure_init(); G__NO_FUSS.with(|c| *c.borrow_mut() = v); }
pub fn g__ONLY_SAID_ABOUT_A_CHILD() -> Py { ensure_init(); G__ONLY_SAID_ABOUT_A_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ONLY_SAID_ABOUT_A_CHILD(v: Py) { ensure_init(); G__ONLY_SAID_ABOUT_A_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g__PARENT_MONEY() -> Py { ensure_init(); G__PARENT_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PARENT_MONEY(v: Py) { ensure_init(); G__PARENT_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g__RAISE_ANIMAL() -> Py { ensure_init(); G__RAISE_ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RAISE_ANIMAL(v: Py) { ensure_init(); G__RAISE_ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g__RAISE_ASK_SUBJECTS() -> Py { ensure_init(); G__RAISE_ASK_SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RAISE_ASK_SUBJECTS(v: Py) { ensure_init(); G__RAISE_ASK_SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g__RAISE_CAT() -> Py { ensure_init(); G__RAISE_CAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RAISE_CAT(v: Py) { ensure_init(); G__RAISE_CAT.with(|c| *c.borrow_mut() = v); }
pub fn g__RAISE_CAT_OUTSIDE() -> Py { ensure_init(); G__RAISE_CAT_OUTSIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RAISE_CAT_OUTSIDE(v: Py) { ensure_init(); G__RAISE_CAT_OUTSIDE.with(|c| *c.borrow_mut() = v); }
pub fn g__RAISE_CHILD() -> Py { ensure_init(); G__RAISE_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RAISE_CHILD(v: Py) { ensure_init(); G__RAISE_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g__RAISE_EXAMPLES() -> Py { ensure_init(); G__RAISE_EXAMPLES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RAISE_EXAMPLES(v: Py) { ensure_init(); G__RAISE_EXAMPLES.with(|c| *c.borrow_mut() = v); }
pub fn g__RAISE_HOUSE() -> Py { ensure_init(); G__RAISE_HOUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RAISE_HOUSE(v: Py) { ensure_init(); G__RAISE_HOUSE.with(|c| *c.borrow_mut() = v); }
pub fn g__RAISE_MONEY() -> Py { ensure_init(); G__RAISE_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RAISE_MONEY(v: Py) { ensure_init(); G__RAISE_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g__RAISE_PLAIN() -> Py { ensure_init(); G__RAISE_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RAISE_PLAIN(v: Py) { ensure_init(); G__RAISE_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g__RAISE_SMOKE() -> Py { ensure_init(); G__RAISE_SMOKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RAISE_SMOKE(v: Py) { ensure_init(); G__RAISE_SMOKE.with(|c| *c.borrow_mut() = v); }
pub fn g__SAID_IT_IS_NEWS() -> Py { ensure_init(); G__SAID_IT_IS_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAID_IT_IS_NEWS(v: Py) { ensure_init(); G__SAID_IT_IS_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g__SHARED_HOUSE() -> Py { ensure_init(); G__SHARED_HOUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHARED_HOUSE(v: Py) { ensure_init(); G__SHARED_HOUSE.with(|c| *c.borrow_mut() = v); }
pub fn g__SHARE_MONEY() -> Py { ensure_init(); G__SHARE_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHARE_MONEY(v: Py) { ensure_init(); G__SHARE_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g__SIGN_PLURAL() -> Py { ensure_init(); G__SIGN_PLURAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SIGN_PLURAL(v: Py) { ensure_init(); G__SIGN_PLURAL.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEBODY_AT_HOME() -> Py { ensure_init(); G__SOMEBODY_AT_HOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEBODY_AT_HOME(v: Py) { ensure_init(); G__SOMEBODY_AT_HOME.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEONE_HURT_A_PART() -> Py { ensure_init(); G__SOMEONE_HURT_A_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEONE_HURT_A_PART(v: Py) { ensure_init(); G__SOMEONE_HURT_A_PART.with(|c| *c.borrow_mut() = v); }
pub fn g__SPELLING_ENTRY_IDS() -> Py { ensure_init(); G__SPELLING_ENTRY_IDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPELLING_ENTRY_IDS(v: Py) { ensure_init(); G__SPELLING_ENTRY_IDS.with(|c| *c.borrow_mut() = v); }
pub fn g__SPELLING_SLIPS() -> Py { ensure_init(); G__SPELLING_SLIPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPELLING_SLIPS(v: Py) { ensure_init(); G__SPELLING_SLIPS.with(|c| *c.borrow_mut() = v); }
pub fn g__SPELLING_TIPS() -> Py { ensure_init(); G__SPELLING_TIPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPELLING_TIPS(v: Py) { ensure_init(); G__SPELLING_TIPS.with(|c| *c.borrow_mut() = v); }
pub fn g__SPELLING_WORDS() -> Py { ensure_init(); G__SPELLING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPELLING_WORDS(v: Py) { ensure_init(); G__SPELLING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__TALKS_OVER() -> Py { ensure_init(); G__TALKS_OVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TALKS_OVER(v: Py) { ensure_init(); G__TALKS_OVER.with(|c| *c.borrow_mut() = v); }
pub fn g__THEIR_SCREENS() -> Py { ensure_init(); G__THEIR_SCREENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THEIR_SCREENS(v: Py) { ensure_init(); G__THEIR_SCREENS.with(|c| *c.borrow_mut() = v); }
pub fn g__TRADES() -> Py { ensure_init(); G__TRADES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TRADES(v: Py) { ensure_init(); G__TRADES.with(|c| *c.borrow_mut() = v); }
pub fn g__USE_THE_WEEK() -> Py { ensure_init(); G__USE_THE_WEEK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__USE_THE_WEEK(v: Py) { ensure_init(); G__USE_THE_WEEK.with(|c| *c.borrow_mut() = v); }
pub fn g__slip() -> Py { ensure_init(); G__slip.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__slip(v: Py) { ensure_init(); G__slip.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
pub fn g_owns_the_deed() -> Py { ensure_init(); G_owns_the_deed.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_owns_the_deed(v: Py) { ensure_init(); G_owns_the_deed.with(|c| *c.borrow_mut() = v); }
pub fn g_the_fault() -> Py { ensure_init(); G_the_fault.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_the_fault(v: Py) { ensure_init(); G_the_fault.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("What to say.\n\nThe message or email for a landlord, a refund, a boss, a friend. Each entry is the\nwording itself, ready to adapt, plus the one thing to keep in mind about tone.\n\nThe matcher in bot/knowledge/lookup.py finds the entry; bot/modules/answers.py answers.\n");
    s_owns_the_deed(py_func("owns_the_deed", |args: &[Py]| pyrt::xrefs::xcall("compose.repair", "owns_the_deed", args)));
    s_MACHINE_SOUNDS(pyrt::xrefs::xget("knowledge.lookup", "MACHINE_SOUNDS"));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_the_fault(py_func("the_fault", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "the_fault", args)));
    s_SLIPS(pyrt::xrefs::xget("knowledge.lookup", "SLIPS"));
    fn __data1() -> Py { py_tuple(vec![py_str("noise"), py_str("noises"), py_str("noisy"), py_str("loud"), py_str("racket"), py_str("racketing"), py_str("din"), py_str("clatter"), py_str("clattering"), py_str("thunderous"), py_str("deafening"), py_str("row"), py_str("drone"), py_str("droning"), py_str("palaver")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.NOISE_WORDS"); }
    s_NOISE_WORDS(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("kid"), py_str("kids"), py_str("child"), py_str("children"), py_str("toddler"), py_str("toddlers"), py_str("baby"), py_str("son"), py_str("sons"), py_str("daughter"), py_str("daughters"), py_str("boy"), py_str("girl"), py_str("teen"), py_str("teens"), py_str("teenager"), py_str("teenagers"), py_str("stepson"), py_str("stepdaughter"), py_str("grandchild"), py_str("grandchildren"), py_str("grandson"), py_str("granddaughter"), py_str("year old"), py_str("year olds"), py_str("primary school"), py_str("secondary school"), py_str("nursery"), py_str("school age")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._A_CHILD_NAMED"); }
    s__A_CHILD_NAMED(__data2());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._ONLY_SAID_ABOUT_A_CHILD"); }
    s__ONLY_SAID_ABOUT_A_CHILD(py_tuple(vec![py_str("screen time"), py_str("screentime")]));
    fn __data3() -> Py { py_tuple(vec![py_str("my mum"), py_str("my mom"), py_str("my mother"), py_str("my dad"), py_str("my father"), py_str("my husband"), py_str("my wife"), py_str("my partner"), py_str("my girlfriend"), py_str("my boyfriend"), py_str("my stepson"), py_str("my stepdaughter"), py_str("my flatmate"), py_str("my housemate"), py_str("my room mate"), py_str("my roommate"), py_str("my friend"), py_str("my mate"), py_str("my brother"), py_str("my sister"), py_str("my nan"), py_str("my nana"), py_str("my gran"), py_str("my granny"), py_str("my grandma"), py_str("my grandad"), py_str("my grandpa"), py_str("my uncle"), py_str("my aunt"), py_str("my auntie"), py_str("my cousin"), py_str("my boss"), py_str("my colleague"), py_str("my staff")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._SOMEBODY_AT_HOME"); }
    s__SOMEBODY_AT_HOME(__data3());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._THEIR_SCREENS"); }
    s__THEIR_SCREENS(stdlib_call("re", "compile", &[py_str("\\b(?:his|her|their)\\s+(?:phone|phones|screens?|tablet|ipad|ipads)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._MY_OWN_SCREENS"); }
    s__MY_OWN_SCREENS(stdlib_call("re", "compile", &[py_str("\\b(?:my|the)\\s+(?:phone|phones)\\b|\\bim\\s+(?:always\\s+)?on\\b|\\bi'm\\s+(?:always\\s+)?on\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._NO_FUSS"); }
    s__NO_FUSS(stdlib_call("re", "compile", &[py_str("\\b(hates?|hated|doesnt|does not|doesn't|dont|don't|not|no|never|isnt|isn't)\\b[^.?!]{0,30}\\b(fuss|attention|centre of attention|center of attention|spotlight|big deal|surprise|surprises|party|to do|big do|a do|night out|speech|faff)\\b|\\b(quiet|low key|low-key|private|small|low|no fuss)\\b[^.?!]{0,16}\\b(birthday|do|party|meal|dinner|card|thing)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._A_COLLEAGUE"); }
    s__A_COLLEAGUE(stdlib_call("re", "compile", &[py_str("\\b(?:colleagues?|co-?workers?|workmates?|boss|manager|staff|team|teacher|neighbou?r|acquaintance|someone at work|people at work|the office)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._A_CLOSE_ONE"); }
    s__A_CLOSE_ONE(stdlib_call("re", "compile", &[py_str("\\b(?:nan|nana|gran|granny|grandma|grandmother|grandad|grandpa|grandfather|mum|mom|mam|mother|dad|father|sister|brother|aunt|auntie|uncle|cousin|wife|husband|partner|girlfriend|boyfriend|friends?|mates?|best mate|son|daughter|grandson|granddaughter)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._ASKS_FOR_THE_WORDS"); }
    s__ASKS_FOR_THE_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:what|how)\\b[^.?!]{0,14}?\\b(?:do|should|shall|can|could|would|will|might|to|am i)\\b[^.?!]{0,10}?\\b(?:write|writing|say|saying|put|word|phrase|sign off)\\b[^.?!]{0,24}?\\b(?:card|cards|note|notes|message|messages|letter|letters|inside|envelope)\\b|\\b(?:card|cards|note|notes|message|messages|letter|letters)\\b[^.?!]{0,24}?\\b(?:what|how)\\b[^.?!]{0,12}?\\b(?:write|say|put|word|phrase)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("language-and-writing")]));
    fn __data4() -> Py { py_tuple(vec![py_tuple(vec![py_str("definitely"), py_tuple(vec![py_str("definately"), py_str("definatly"), py_str("definitley"), py_str("defintely")]), py_str("Definitely has **finite** inside it: de-finite-ly. Two i's, and no a anywhere.")]), py_tuple(vec![py_str("separate"), py_tuple(vec![py_str("seperate"), py_str("seperately"), py_str("seperatly"), py_str("seperates")]), py_str("Separate has **a rat** in the middle: sep-a-rat-e. There is an a after the p, and another before the t.")]), py_tuple(vec![py_str("tomorrow"), py_tuple(vec![py_str("tommorow"), py_str("tommorrow"), py_str("tomorow")]), py_str("Tomorrow has one m and two r's: to-m-o-rr-o-w. Say \"to morrow\", then close it up.")]), py_tuple(vec![py_str("because"), py_tuple(vec![py_str("becuase"), py_str("becasue"), py_str("becouse")]), py_str("Because has **cause** in it: be-cause. If you can spell cause, you can spell because.")]), py_tuple(vec![py_str("receive"), py_tuple(vec![py_str("recieve"), py_str("recieved"), py_str("recieving")]), py_str("Receive is i before e except after c: re-cei-ve. Recieve gets it the wrong way round, which is why it is the classic one.")]), py_tuple(vec![py_str("believe"), py_tuple(vec![py_str("beleive"), py_str("belive"), py_str("beleiving")]), py_str("Believe is be-lie-ve: the **lie** is in the middle. Receive is the one that goes -ei-.")]), py_tuple(vec![py_str("accommodation"), py_tuple(vec![py_str("accommodate"), py_str("accomodation"), py_str("acommodation"), py_str("acomodation"), py_str("accomodate")]), py_str("Accommodation has two c's and two m's: a-cc-o-mm-odation, and accommodate keeps both.")]), py_tuple(vec![py_str("necessary"), py_tuple(vec![py_str("neccessary"), py_str("necesary"), py_str("neccesary")]), py_str("Necessary has one c and two s's: picture a shirt with one **c**ollar and two **s**leeves. Necessarily and necessity keep the same shape.")]), py_tuple(vec![py_str("calendar"), py_tuple(vec![py_str("calender")]), py_str("The thing with the months is calendar, with an a before the r. A calender is a machine that presses paper flat.")]), py_tuple(vec![py_str("embarrass"), py_tuple(vec![py_str("embarass"), py_str("embarrassing"), py_str("embarasing")]), py_str("Embarrass has two r's and two s's: em-ba-rr-a-ss. Think \"bare arse\" and you will not forget it.")]), py_tuple(vec![py_str("rhythm"), py_tuple(vec![py_str("rythm"), py_str("rythem"), py_str("rhythym")]), py_str("Rhythm is r-h-y-t-h-m and has no ordinary vowel of its own: \"rhythm helps your two hips move\" spells it.")]), py_tuple(vec![py_str("weird"), py_tuple(vec![py_str("wierd"), py_str("wierdly")]), py_str("Weird breaks the rule: w-e-i-r-d, e before i. The line to remember is \"we are weird\".")]), py_tuple(vec![py_str("friend"), py_tuple(vec![py_str("freind"), py_str("freinds"), py_str("frend")]), py_str("Friend ends in **end**: fri-end. The e goes before the n, unlike most -ie- words.")]), py_tuple(vec![py_str("beautiful"), py_tuple(vec![py_str("beutiful"), py_str("beautifull"), py_str("beatiful")]), py_str("Beautiful is beauty plus -ful: b-e-a-u-t-i-f-u-l. One t, one l.")]), py_tuple(vec![py_str("business"), py_tuple(vec![py_str("buisness"), py_str("busines"), py_str("buisnesses")]), py_str("Business is busy-ness gone crooked: b-u-s-i-n-e-s-s.")]), py_tuple(vec![py_str("beginning"), py_tuple(vec![py_str("begining"), py_str("beginnig")]), py_str("Beginning has two n's in the middle: be-gi-nn-ing. The stress is on the second syllable, so the n doubles.")]), py_tuple(vec![py_str("occurred"), py_tuple(vec![py_str("occured"), py_str("occuring"), py_str("ocurred")]), py_str("Occurred has two c's and two r's: o-cc-u-rr-ed. Occur doubles both when it grows a tail.")]), py_tuple(vec![py_str("until"), py_tuple(vec![py_str("untill"), py_str("untl")]), py_str("Until has one l. Till and until both exist, but the one-l until is the one people write.")]), py_tuple(vec![py_str("address"), py_tuple(vec![py_str("adress"), py_str("adres"), py_str("addres")]), py_str("Address has two d's and two s's: a-dd-re-ss.")]), py_tuple(vec![py_str("argument"), py_tuple(vec![py_str("arguement")]), py_str("Argument loses the e from argue: ar-gu-ment. Arguement has one letter too many.")]), py_tuple(vec![py_str("forty"), py_tuple(vec![py_str("fourty")]), py_str("Forty has no u: four becomes forty by dropping it. Fourteen and fourth keep theirs.")]), py_tuple(vec![py_str("probably"), py_tuple(vec![py_str("probly"), py_str("probally"), py_str("probaly")]), py_str("Probably is prob-ab-ly, all three syllables written even though we say two.")]), py_tuple(vec![py_str("immediately"), py_tuple(vec![py_str("immediatly"), py_str("immediatley"), py_str("imediately")]), py_str("Immediately is im-medi-ate-ly: mediate is in the middle, so the a stays.")]), py_tuple(vec![py_str("environment"), py_tuple(vec![py_str("enviroment"), py_str("enviornment")]), py_str("Environment has an n after the o: env-i-ron-ment, from environ, the surroundings.")]), py_tuple(vec![py_str("government"), py_tuple(vec![py_str("goverment"), py_str("governement")]), py_str("Government has an n in the middle: govern-ment. It is govern with ment on the end.")]), py_tuple(vec![py_str("surprise"), py_tuple(vec![py_str("suprise"), py_str("surprize"), py_str("surpise")]), py_str("Surprise starts sur-, from the French, not sup-. English uses s here, not z.")]), py_tuple(vec![py_str("truly"), py_tuple(vec![py_str("truely"), py_str("truley")]), py_str("Truly drops the e from true: tru-ly. It is one of the few -ly words that loses it, with duly and wholly.")]), py_tuple(vec![py_str("their"), py_tuple(vec![py_str("thier")]), py_str("Their is the belonging one: **their** house, like **her** house with a t. There is a place (it has here inside it), and they're means they are.")]), py_tuple(vec![py_str("you're"), py_tuple(vec![py_str("youre"), py_str("your welcome"), py_str("your right"), py_str("your doing")]), py_str("You're means **you are**, so it keeps the apostrophe; your is belonging, as in your coat. If you can say \"you are\" in the sentence, it is you're.")]), py_tuple(vec![py_str("lose"), py_tuple(vec![py_str("loose weight"), py_str("loosing")]), py_str("Lose has one o and rhymes with news; loose has two, rhymes with goose, and means not tight. \"Loose weight\" says the wrong word.")]), py_tuple(vec![py_str("practice"), py_tuple(vec![py_str("practise or practice")]), py_str("In British English the noun is practice and the verb is practise: \"the practice\" but \"to practise\". American English uses practice for both.")]), py_tuple(vec![py_str("congratulations"), py_tuple(vec![py_str("congradulations"), py_str("congratualtions"), py_str("congratuations")]), py_str("Congratulations has a t after the grat: con-grat-u-la-tions. Say \"gratitude\" to yourself and the t is there.")]), py_tuple(vec![py_str("occasion"), py_tuple(vec![py_str("ocassion"), py_str("occassion"), py_str("ocasion")]), py_str("Occasion has two c's and one s: o-cc-a-s-ion. The double c is the one people drop.")]), py_tuple(vec![py_str("across"), py_tuple(vec![py_str("accross"), py_str("acros")]), py_str("Across is one c and two s's: a-cross. The double c comes from hearing the join as a double sound.")]), py_tuple(vec![py_str("quiet"), py_tuple(vec![py_str("quite or quiet"), py_str("quiett")]), py_str("Quiet, with the e last, is the opposite of noisy: q-u-i-e-t. Quite, with the e before the t, means fairly or completely - a quiet evening, but quite tired.")])]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.SPELLINGS"); }
    s_SPELLINGS(__data4());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.THANKS_ANY"); }
    s_THANKS_ANY(py_str("Name the thing they did, not the day in general: \"Thank you for the lemon cake and for staying to help with the chairs, it made the afternoon easy.\" One line about a specific detail beats a paragraph of adjectives, and send it the same day, because thanks the next morning reads as an afterthought."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.THANKS_FLOWERS"); }
    s_THANKS_FLOWERS(py_tuple(vec![py_str("flower"), py_str("flowers"), py_str("bouquet"), py_str("bouquets"), py_str("orchid"), py_str("orchids")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.THANKS_GIFT"); }
    s_THANKS_GIFT(py_tuple(vec![py_str("present"), py_str("presents"), py_str("gift"), py_str("gifts"), py_str("voucher"), py_str("vouchers"), py_str("hamper")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.THANKS_STAY"); }
    s_THANKS_STAY(py_tuple(vec![py_str("stay"), py_str("stays"), py_str("staying"), py_str("stayed"), py_str("host"), py_str("hosts"), py_str("hosted"), py_str("hosting"), py_str("put me up"), py_str("weekend"), py_str("weekends"), py_str("nights")]));
    fn __data5() -> Py { py_tuple(vec![py_str("meal"), py_str("meals"), py_str("dinner"), py_str("lunch"), py_str("tea"), py_str("supper"), py_str("party"), py_str("barbecue"), py_str("bbq"), py_str("food"), py_str("cooked"), py_str("cook"), py_str("fed"), py_str("cake"), py_str("roast")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.THANKS_MEAL"); }
    s_THANKS_MEAL(__data5());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.THANKS_WRITTEN"); }
    s_THANKS_WRITTEN(py_tuple(vec![py_str("card"), py_str("cards"), py_str("letter"), py_str("letters"), py_str("note"), py_str("notes"), py_str("text"), py_str("texts"), py_str("message"), py_str("messages"), py_str("email"), py_str("emails")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.THANKS_LIFT"); }
    s_THANKS_LIFT(py_tuple(vec![py_str("lift"), py_str("lifts"), py_str("drive"), py_str("drove"), py_str("driving"), py_str("picking up"), py_str("picked up"), py_str("dropping off"), py_str("dropped off"), py_str("run to"), py_str("airport"), py_str("station")]));
    fn __data6() -> Py { py_tuple(vec![py_str("feeding"), py_str("fed"), py_str("watering"), py_str("watered"), py_str("looking after"), py_str("looked after"), py_str("collecting the post"), py_str("collected the post"), py_str("cat"), py_str("dog"), py_str("plants"), py_str("post"), py_str("bin"), py_str("house sitting"), py_str("housesitting")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.THANKS_FAVOUR"); }
    s_THANKS_FAVOUR(__data6());
    fn __data7() -> Py { py_tuple(vec![py_str("teacher"), py_str("nurse"), py_str("carer"), py_str("midwife"), py_str("doctor"), py_str("neighbour"), py_str("neighbor"), py_str("colleague"), py_str("boss"), py_str("manager"), py_str("childminder"), py_str("helped me"), py_str("helped us"), py_str("who helped")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.THANKS_PERSON"); }
    s_THANKS_PERSON(__data7());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._MOVING_OUT"); }
    s__MOVING_OUT(stdlib_call("re", "compile", &[py_str("\\bmoving out\\b|\\bmoving (?:house|flat|home)\\b|\\bleaving home\\b|\\bmoved out\\b|\\bmoving in with\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._MOVING_AWAY"); }
    s__MOVING_AWAY(stdlib_call("re", "compile", &[py_str("\\bmoving away\\b|\\bemigrat\\w*\\b|\\bgoing travelling\\b|\\bmoving abroad\\b|\\bmoving to (?:australia|canada|spain|france|new zealand|america|the us|germany|ireland|scotland)"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._BABY"); }
    s__BABY(stdlib_call("re", "compile", &[py_str("\\bpregnan\\w*\\b|\\bexpecting a baby\\b|\\bbaby on the way\\b|\\bhaving a baby\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._SAID_IT_IS_NEWS"); }
    s__SAID_IT_IS_NEWS(stdlib_call("re", "compile", &[py_str("\\bnews\\b|\\bannounc\\w*\\b|\\bbreak\\s+(?:it|this|the)\\b|\\bbreak\\s+bad\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._MONEY"); }
    s__MONEY(stdlib_call("re", "compile", &[py_str("\\bmoney\\b|\\bfinances?\\b|\\bdebts?\\b|\\bsalary\\b|\\bbills?\\b|\\bwages?\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._SHARED_HOUSE"); }
    s__SHARED_HOUSE(stdlib_call("re", "compile", &[py_str("\\b(?:housemate|flatmate|roommate|house ?mate)s?\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._HOUSEHOLD_MONEY"); }
    s__HOUSEHOLD_MONEY(stdlib_call("re", "compile", &[py_str("\\b(?:we|our|us|partner|husband|wife|spouse|bills?|rent|split|share|sharing|housemates?|flatmates?|roommates?|household|money with|finances)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._PARENT_MONEY"); }
    s__PARENT_MONEY(stdlib_call("re", "compile", &[py_str("\\b(?:my )?(?:parents?|mum|mom|mam|mother|dad|father|family|in ?laws?)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._SHARE_MONEY"); }
    s__SHARE_MONEY(stdlib_call("re", "compile", &[py_str("\\bpay\\w*\\b|\\brent\\b|\\bbills?\\b|\\bshare\\b|\\bowe\\w*\\b|\\bsplit\\b|\\bcouncil tax\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._MESS"); }
    s__MESS(stdlib_call("re", "compile", &[py_str("\\bclean\\w*\\b|\\bwashing up\\b|\\bwashing-up\\b|\\bdishes\\b|\\bmess\\b|\\btidy\\w*\\b|\\bbins?\\b|\\bhoover\\w*\\b|\\bload of washing\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._RAISE_MONEY"); }
    s__RAISE_MONEY(stdlib_call("re", "compile", &[py_str("\\bmoney\\b|\\blent\\b|\\blend\\w*\\b|\\bborrow\\w*\\b|\\bowe\\w*\\b|\\bdebt\\w*\\b|\\bpay\\w*\\b|\\bquid\\b|£"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._RAISE_CAT_OUTSIDE"); }
    s__RAISE_CAT_OUTSIDE(stdlib_call("re", "compile", &[py_str("\\b(?:cat|cats|kitten|kittens)\\b(?=[^.]*\\bgarden\\b)|\\bgarden\\b(?=[^.]*\\b(?:cat|cats|kitten|kittens)\\b)"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._RAISE_CAT"); }
    s__RAISE_CAT(stdlib_call("re", "compile", &[py_str("\\b(?:cat|cats|kitten|kittens)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._RAISE_ANIMAL"); }
    s__RAISE_ANIMAL(stdlib_call("re", "compile", &[py_str("\\b(?:dog|dogs|puppy|pets?|animal|horse|rabbit|parrot)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._RAISE_CHILD"); }
    s__RAISE_CHILD(stdlib_call("re", "compile", &[py_str("\\b(?:child|children|kid|kids|son|daughter|baby|toddler|niece|nephew|boys|girls)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._RAISE_HOUSE"); }
    s__RAISE_HOUSE(stdlib_call("re", "compile", &[py_str("\\b(?:noise|noisy|music|loud|party|parking|garden|hedge|fence|bin|bins|rubbish|alarm)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._RAISE_SMOKE"); }
    s__RAISE_SMOKE(stdlib_call("re", "compile", &[py_str("\\bsmok\\w*|\\bcigarette\\w*|\\bvap\\w*|\\bsmoking\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._RAISE_EXAMPLES"); }
    s__RAISE_EXAMPLES(py_tuple(vec![py_tuple(vec![g__RAISE_CAT_OUTSIDE(), py_str("\"your cat keeps using my veg bed - could you keep her in at night?\"")]), py_tuple(vec![g__RAISE_CAT(), py_str("\"Mog swiped at Nell again on Saturday - could you keep her out of the room while we are there?\"")]), py_tuple(vec![g__RAISE_ANIMAL(), py_str("\"Rex jumped up at Nell again on Saturday - could you keep him in the kitchen while she is here?\"")]), py_tuple(vec![g__RAISE_CHILD(), py_str("\"the boys were a bit rough with Nell on Saturday - could you keep an eye when they are in the garden?\"")]), py_tuple(vec![g__RAISE_SMOKE(), py_str("\"the smoke comes through the wall - could you smoke by the back door?\"")]), py_tuple(vec![g__RAISE_HOUSE(), py_str("\"the music was still going at half one - could we call it midnight next time?\"")]), py_tuple(vec![g__RAISE_MONEY(), py_str("\"I lent you £200 and I have been unsure how to bring it up\"")])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._RAISE_PLAIN"); }
    s__RAISE_PLAIN(py_str("\"I felt left out when the plans changed\""));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._RAISE_ASK_SUBJECTS"); }
    s__RAISE_ASK_SUBJECTS(py_tuple(vec![g__RAISE_CAT_OUTSIDE(), g__RAISE_CAT(), g__RAISE_ANIMAL(), g__RAISE_CHILD(), g__RAISE_SMOKE()]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._HURT_PART"); }
    s__HURT_PART(py_str("(?:back|arm|arms|leg|legs|knee|knees|ankle|ankles|shoulder|shoulders|wrist|wrists|hand|hands|finger|fingers|thumb|toe|toes|foot|feet|hip|hips|head|neck|ribs?|rib|shin|shins|calf|elbow|elbows|eye|jaw|collarbone|collarbones|spine)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._SOMEONE_HURT_A_PART"); }
    s__SOMEONE_HURT_A_PART(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\bhurt\\w*\\s+(?:[a-z-]+\\s+){0,2}(?:his|her|their|its)\\s+(?:[a-z-]+\\s+){0,2}"), &g__HURT_PART()), &py_str("\\b|\\bhurt\\w*\\s+(?:my|his|her|their|your)\\s+")), &g__HURT_PART()), &py_str("\\b|\\bhurt\\w*\\s+(?:himself|herself|themselves|myself)\\b")), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._TALKS_OVER"); }
    s__TALKS_OVER(stdlib_call("re", "compile", &[py_str("\\b(?:talks?|talking|talk) over (?:me|my)\\b|\\bkeeps? (?:talking|interrupting|cutting)\\b|\\b(?:interrupts?|interrupting) (?:me|my)\\b|\\bcuts? me off\\b|\\bkeeps? cutting me off\\b|\\b(?:butts?|butting) in\\b|\\bfinishes? my sentences?\\b|\\btalks? over people\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._SPELLING_WORDS"); }
    s__SPELLING_WORDS(call_builtin("tuple", &[call_builtin("sorted", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SPELLINGS()) { let __unpk1 = py_unpack(&__item.clone(), 3, None); let mut right: Py = __unpk1[0].clone(); let mut slips: Py = __unpk1[1].clone(); let mut _tip: Py = __unpk1[2].clone(); for __item in py_iter_or_raise(&op_add(&py_tuple(vec![right.clone()]), &slips.clone())) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&if py_truthy(&Py::Bool(py_eq(&word.clone(), &right.clone()))) { _inflections(&[word.clone()]) } else { py_tuple(vec![word.clone()]) }) { let mut form: Py = __item.clone(); __out.push(form.clone()); } } } py_set(__out) })() }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._SPELLING_SLIPS"); }
    s__SPELLING_SLIPS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SPELLINGS()) { let __unpk1 = py_unpack(&__item.clone(), 3, None); let mut _right: Py = __unpk1[0].clone(); let mut slips: Py = __unpk1[1].clone(); let mut _tip: Py = __unpk1[2].clone(); for __item in py_iter_or_raise(&slips.clone()) { let mut slip: Py = __item.clone(); __out.push(slip.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._SPELLING_ENTRY_IDS"); }
    s__SPELLING_ENTRY_IDS(py_tuple(vec![py_str("spelling-tricky-word"), py_str("spelling-pick-one")]));
    let __iter8 = py_iter_or_raise(&g__SPELLING_SLIPS());
    for __item in __iter8 {
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._slip"); }
    s__slip(__item.clone());
    let _ = mcall(&mcall(&g_SLIPS(), "setdefault", &[g__slip(), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]), "update", &[g__SPELLING_ENTRY_IDS()]);
    }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._SPELLING_TIPS"); }
    s__SPELLING_TIPS({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&g_SPELLINGS()) { let __unpk1 = py_unpack(&__item.clone(), 3, None); let mut right: Py = __unpk1[0].clone(); let mut slips: Py = __unpk1[1].clone(); let mut tip: Py = __unpk1[2].clone(); for __item in py_iter_or_raise(&op_add(&py_tuple(vec![right.clone()]), &slips.clone())) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&_inflections(&[word.clone()])) { let mut form: Py = __item.clone(); __out.push((form.clone(), tip.clone())); } } } py_dict(__out) })() });
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.SAID"); }
    s_SAID(py_tuple(vec![py_str("what did i say"), py_str("what did we say"), py_str("did i say"), py_str("did i tell"), py_str("you said"), py_str("i said"), py_str("own words"), py_str("i told you")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.SAY"); }
    s_SAY(py_tuple(vec![py_str("what do i say"), py_str("what should i say"), py_str("what to say"), py_str("say to"), py_str("say at")]));
    fn __data9() -> Py { py_tuple(vec![py_str("died"), py_str("dies"), py_str("dying"), py_str("death"), py_str("dead"), py_str("passed away"), py_str("passed on"), py_str("loss"), py_str("lost my"), py_str("funeral"), py_str("burial"), py_str("cremation"), py_str("bereavement"), py_str("grief"), py_str("grieving"), py_str("condolence"), py_str("condolences"), py_str("sympathy"), py_str("widow"), py_str("terminal"), py_str("cancer")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.MOURNING"); }
    s_MOURNING(__data9());
    fn __data10() -> Py { py_tuple(vec![py_str("answerphone"), py_str("answer machine"), py_str("answering machine"), py_str("voicemail"), py_str("voice mail"), py_str("to leave a message"), py_str("leave a message on"), py_str("left a message on"), py_str("take a message"), py_str("took a message"), py_str("who should i leave"), py_str("can i leave a message"), py_str("missed call"), py_str("ring back"), py_str("call back"), py_str("on the phone"), py_str("by phone"), py_str("phone call"), py_str("phonecall"), py_str("him a message"), py_str("her a message"), py_str("them a message"), py_str("message on his"), py_str("message on her"), py_str("message on the")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.ON_THE_PHONE"); }
    s_ON_THE_PHONE(__data10());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._EULOGY_LENGTH"); }
    s__EULOGY_LENGTH(py_tuple(vec![py_str("how long"), py_str("length"), py_str("words"), py_str("word count"), py_str("minutes"), py_str("too long"), py_str("short")]));
    fn __data11() -> Py { py_tuple(vec![py_str("how do i start"), py_str("how to start"), py_str("start"), py_str("opening"), py_str("open"), py_str("begin"), py_str("first line"), py_str("example"), py_str("examples"), py_str("template"), py_str("give me"), py_str("ideas"), py_str("what do i say")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._EULOGY_START"); }
    s__EULOGY_START(__data11());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.FOLLOW_UP_THANKS"); }
    s_FOLLOW_UP_THANKS(py_str("Send a short thank-you the same day: \"Thank you for the conversation today — I'm even keener on the role after hearing about the new system. Happy to answer anything else.\""));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.FOLLOW_UP_CHASE"); }
    s_FOLLOW_UP_CHASE(py_str("Chase once, a week after you applied: two lines, say you are still interested and one thing you would bring to the job, and ask if they need anything else from you. Then leave it - ring only if the advert gave a date and it has passed. Keep applying elsewhere in the meantime."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.INTERVIEW_WORDS"); }
    s_INTERVIEW_WORDS(py_tuple(vec![py_str("interview"), py_str("interviews"), py_str("interviewed"), py_str("panel"), py_str("met them"), py_str("met the team"), py_str("after the interview"), py_str("thank you note"), py_str("thank-you"), py_str("thank you letter"), py_str("spoke to them"), py_str("spoke to the manager")]));
    fn __data12() -> Py { py_tuple(vec![py_str("chase"), py_str("chasing"), py_str("chased"), py_str("follow up"), py_str("following up"), py_str("follows up"), py_str("followed up"), py_str("follow-up"), py_str("heard"), py_str("heard back"), py_str("heard nothing"), py_str("no reply"), py_str("nothing back"), py_str("no answer"), py_str("waiting"), py_str("not heard"), py_str("still waiting"), py_str("when should i")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.CHASE_WORDS"); }
    s_CHASE_WORDS(__data12());
    fn __data13() -> Py { py_tuple(vec![py_str("how long"), py_str("how many days"), py_str("how many day"), py_str("how many weeks"), py_str("how many week"), py_str("how soon"), py_str("how much longer"), py_str("when should i"), py_str("when do i"), py_str("days should i wait"), py_str("weeks should i wait"), py_str("how long should i wait"), py_str("should i wait"), py_str("should i leave it")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.WHEN_WORDS"); }
    s_WHEN_WORDS(__data13());
    fn __data14() -> Py { py_tuple(vec![py_str("builder"), py_str("builders"), py_str("plumber"), py_str("plumbers"), py_str("electrician"), py_str("electricians"), py_str("roofer"), py_str("roofers"), py_str("decorator"), py_str("decorators"), py_str("painter"), py_str("painters"), py_str("plasterer"), py_str("plasterers"), py_str("tiler"), py_str("tilers"), py_str("joiner"), py_str("carpenter"), py_str("gardener"), py_str("landscaper"), py_str("handyman"), py_str("contractor"), py_str("tradesman"), py_str("tradesperson"), py_str("workman"), py_str("mechanic"), py_str("garage"), py_str("fitter"), py_str("installer")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._TRADES"); }
    s__TRADES(__data14());
    fn __data15() -> Py { py_tuple(vec![py_str("how long"), py_str("how many days"), py_str("how many day"), py_str("how many weeks"), py_str("how many week"), py_str("how soon"), py_str("how much longer"), py_str("when should i"), py_str("when do i"), py_str("when to"), py_str("what is the right time"), py_str("what's the right time"), py_str("after a week"), py_str("a week after"), py_str("left it a week"), py_str("leave it"), py_str("how many days before")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.FOLLOW_UP_ASK"); }
    s_FOLLOW_UP_ASK(__data15());
    fn __data16() -> Py { py_tuple(vec![py_str("council"), py_str("landlord"), py_str("letting agent"), py_str("bank"), py_str("refund"), py_str("parcel"), py_str("delivery"), py_str("order"), py_str("insurance"), py_str("warranty"), py_str("broadband"), py_str("energy"), py_str("supplier"), py_str("dentist"), py_str("doctor"), py_str("surgery"), py_str("hospital"), py_str("school"), py_str("nursery"), py_str("vet"), py_str("garage"), py_str("neighbour"), py_str("complaint"), py_str("complaints"), py_str("complaining"), py_str("claim"), py_str("claims"), py_str("compensation")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.NOT_A_JOB"); }
    s_NOT_A_JOB(op_add(&g__TRADES(), &__data16()));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.FOLLOW_UP_AFTER_INTERVIEW"); }
    s_FOLLOW_UP_AFTER_INTERVIEW(py_str("Give it a week after the interview: one short note that thanks them for the time, gives one reason you want the job, and asks plainly for a timescale. Then leave it - ring only if they gave you a date that has passed."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.FOLLOW_UP_WHEN"); }
    s_FOLLOW_UP_WHEN(py_str("Give it a week: a week from the day you sent it, or a week after the date they gave you, long enough that they have news and short enough that you are still live. Then one short message - what you are following up, when you sent it, and a plain ask for a timescale - and after that leave it with them rather than chase again."));
    fn __data17() -> Py { py_tuple(vec![py_str("leave early"), py_str("leaving early"), py_str("finish early"), py_str("finishing early"), py_str("an hour early"), py_str("half day"), py_str("early on friday"), py_str("early tomorrow"), py_str("go early"), py_str("leave at"), py_str("leaving at"), py_str("dip out"), py_str("knock off early"), py_str("leave a bit early"), py_str("leave work early"), py_str("out early")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.EARLY_WORDS"); }
    s_EARLY_WORDS(__data17());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.TIME_OFF_SCRIPT"); }
    s_TIME_OFF_SCRIPT(py_str("Ask early, name the dates and say who covers: \"Could I take 12–16 May off? I've arranged cover with Priya and nothing is due that week.\" One question, one answer, and put it in writing."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.EARLY_SCRIPT"); }
    s_EARLY_SCRIPT(py_str("Give the time and the cover, not the whole story: \"Hi Sam, is it OK if I leave at 3 today? I've moved the 4pm call to Priya and I'll pick the rest up in the morning.\" Ask the day before when you can, keep it to two lines, and if it is more than a one-off ask for a change of hours instead of asking again each week."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.TIME_OFF_SCRIPT_YES"); }
    s_TIME_OFF_SCRIPT_YES(py_str("Yes, if you ask early and name the dates: \"Could I take 12-16 May off? I've arranged cover with Priya and nothing is due that week.\" One question, one answer, and put it in writing."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.EARLY_SCRIPT_YES"); }
    s_EARLY_SCRIPT_YES(py_str("Yes - give the time and who covers it rather than the whole story: \"Hi Sam, is it OK if I leave early on Friday? I've moved the 4pm call to Priya and I'll pick the rest up on Monday.\" Say it the day before if you can, and keep it to two lines."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.YES_WORDS"); }
    s_YES_WORDS(py_tuple(vec![py_str("can i"), py_str("can we"), py_str("could i"), py_str("may i"), py_str("am i allowed"), py_str("is it ok"), py_str("is it okay"), py_str("is it fine"), py_str("would it be ok"), py_str("would it be okay"), py_str("do you think i can"), py_str("should i ask")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._ALREADY_HAD"); }
    s__ALREADY_HAD(py_tuple(vec![py_str("booked"), py_str("approved"), py_str("confirmed"), py_str("in the diary"), py_str("granted"), py_str("got the time off"), py_str("got the week off"), py_str("got my holiday"), py_str("have the time off")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._HAS_THE_TIME"); }
    s__HAS_THE_TIME(py_tuple(vec![py_str("taking next week off"), py_str("taking the week off"), py_str("off work next week"), py_str("off next week"), py_str("next week off"), py_str("im off"), py_str("i'm off"), py_str("i am off"), py_str("off work from"), py_str("off work for"), py_str("off next month"), py_str("off all week")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._AWAY_OR_HOME"); }
    s__AWAY_OR_HOME(py_tuple(vec![py_str("go somewhere"), py_str("going somewhere"), py_str("go away"), py_str("going away"), py_str("get away"), py_str("stay at home"), py_str("staying at home"), py_str("at home or away"), py_str("away or at home"), py_str("somewhere warm"), py_str("a trip"), py_str("trip with")]));
    fn __data18() -> Py { py_tuple(vec![py_str("no plans"), py_str("nothing planned"), py_str("how do i use"), py_str("how do i plan"), py_str("how to plan"), py_str("how should i spend"), py_str("how do i spend"), py_str("use a week off"), py_str("use my week off"), py_str("use the week off"), py_str("use the time off"), py_str("spend a week off"), py_str("spend my week off"), py_str("spend the week off"), py_str("plan a week off"), py_str("plan my week off"), py_str("plan the week off"), py_str("a week off well"), py_str("what to do with"), py_str("make the most of"), py_str("sort my"), py_str("week off the booze"), py_str("week off booze"), py_str("week off the drink"), py_str("week off drink"), py_str("week off wine"), py_str("week off alcohol"), py_str("week off the alcohol"), py_str("week off sugar"), py_str("week off dairy"), py_str("week off meat"), py_str("week off caffeine"), py_str("week off takeaways"), py_str("week off vaping"), py_str("week off smoking"), py_str("week off social media"), py_str("week off chocolate"), py_str("week off crisps"), py_str("week off my phone"), py_str("week off the phone"), py_str("week off sweets"), py_str("week off carbs"), py_str("week off bread"), py_str("week off cheese"), py_str("week off gluten")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._USE_THE_WEEK"); }
    s__USE_THE_WEEK(__data18());
    fn __data19() -> Py { py_tuple(vec![py_str("ask"), py_str("asking"), py_str("asked"), py_str("request"), py_str("requesting"), py_str("book"), py_str("booking"), py_str("booked"), py_str("put in for"), py_str("entitled"), py_str("entitlement"), py_str("can i"), py_str("can we"), py_str("could i"), py_str("may i"), py_str("am i allowed"), py_str("is it ok"), py_str("is it okay"), py_str("allowed to"), py_str("permission"), py_str("want to take"), py_str("want to book"), py_str("want to request"), py_str("wanted to take"), py_str("need to take"), py_str("need to book"), py_str("need to request"), py_str("hope to take"), py_str("planning to take"), py_str("how do i"), py_str("how should i"), py_str("how would i"), py_str("how to get"), py_str("how to take"), py_str("what do i say"), py_str("what should i say"), py_str("who do i ask"), py_str("how much notice"), py_str("how many days"), py_str("take a day off"), py_str("take a week off"), py_str("take some time off"), py_str("take time off"), py_str("take it off"), py_str("take the day off"), py_str("take the week off"), py_str("need a"), py_str("need some"), py_str("need time"), py_str("need leave"), py_str("want some"), py_str("want a day off"), py_str("want the day off"), py_str("want time off"), py_str("like a day off"), py_str("like a week off"), py_str("like some time off"), py_str("id like"), py_str("i'd like"), py_str("would like a"), py_str("get a day off"), py_str("get a week off"), py_str("get time off"), py_str("get the day off"), py_str("annual leave"), py_str("leave request"), py_str("request leave"), py_str("book leave"), py_str("holiday form"), py_str("leave form"), py_str("holiday request"), py_str("date of leave")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._ASKING_FRAME"); }
    s__ASKING_FRAME(__data19());
    fn __data20() -> Py { py_tuple(vec![py_str("january"), py_str("february"), py_str("march"), py_str("april"), py_str("may"), py_str("june"), py_str("july"), py_str("august"), py_str("september"), py_str("october"), py_str("november"), py_str("december"), py_str("next month"), py_str("this month"), py_str("end of the month"), py_str("month")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._MONTHS_NAMED"); }
    s__MONTHS_NAMED(__data20());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._HOW_LONG"); }
    s__HOW_LONG(py_tuple(vec![py_str("two weeks"), py_str("three weeks"), py_str("four weeks"), py_str("five weeks"), py_str("ten days"), py_str("a fortnight"), py_str("fortnight off"), py_str("week off in"), py_str("weeks off in")]));
    fn __data21() -> Py { py_tuple(vec![py_str("can i"), py_str("can we"), py_str("could i"), py_str("may i"), py_str("am i allowed"), py_str("is it ok"), py_str("is it okay"), py_str("want to"), py_str("need to"), py_str("wish to"), py_str("hope to"), py_str("planning to"), py_str("like to"), py_str("ask"), py_str("asking"), py_str("request"), py_str("book"), py_str("take"), py_str("put in for"), py_str("claim"), py_str("entitled"), py_str("how many days"), py_str("what do i say"), py_str("how do i get time off"), py_str("need a"), py_str("need some"), py_str("need time"), py_str("need leave"), py_str("want a day off")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._ASKS_TO_HAVE"); }
    s__ASKS_TO_HAVE(__data21());
    fn __data22() -> Py { py_tuple(vec![py_str("can i"), py_str("can we"), py_str("could i"), py_str("may i"), py_str("am i allowed"), py_str("is it ok"), py_str("is it okay"), py_str("how do i ask"), py_str("how do i take"), py_str("how do i book"), py_str("how do i request"), py_str("how do i get time off"), py_str("how do i go about"), py_str("how do you ask"), py_str("how to ask"), py_str("should i ask"), py_str("want to"), py_str("need to"), py_str("need a"), py_str("ask"), py_str("asking"), py_str("book"), py_str("booking"), py_str("request"), py_str("entitled"), py_str("what do i say"), py_str("how should i")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._ASK_VERBS"); }
    s__ASK_VERBS(__data22());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.MY_DOG_NOISE"); }
    s_MY_DOG_NOISE(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("my dog"), py_str("our dog"), py_str("my dogs"), py_str("our dogs"), py_str("my puppy"), py_str("our puppy")])) { let mut owner: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("barks"), py_str("keeps barking"), py_str("is barking"), py_str("barking"), py_str("wont stop barking"), py_str("will not stop barking")])) { let mut verb: Py = __item.clone(); __out.push(op_modulo(&py_str("%s %s"), &py_tuple(vec![owner.clone(), verb.clone()]))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    fn __data23() -> Py { py_tuple(vec![py_str("what do i say"), py_str("what should i say"), py_str("what can i say"), py_str("what to say"), py_str("what do i tell"), py_str("what should i tell"), py_str("what can i tell"), py_str("what to tell"), py_str("how do i comfort"), py_str("how do i console"), py_str("comfort"), py_str("console"), py_str("comforting"), py_str("cheer up"), py_str("cheering up"), py_str("how do i cheer"), py_str("be there for"), py_str("how do i support"), py_str("how do i help them"), py_str("how do i help her"), py_str("how do i help him"), py_str("how do i help my"), py_str("what can i do for")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.COMFORT_SAY"); }
    s_COMFORT_SAY(__data23());
    fn __data24() -> Py { py_tuple(vec![py_str("dumped"), py_str("breakup"), py_str("broke up"), py_str("split up"), py_str("divorce"), py_str("divorced"), py_str("divorcing"), py_str("separated"), py_str("walked out"), py_str("left her"), py_str("left him"), py_str("left them"), py_str("his ex"), py_str("her ex")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.COMFORT"); }
    s_COMFORT(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("crying"), py_str("cries"), py_str("cried"), py_str("tears"), py_str("tearful"), py_str("sobbing"), py_str("sobs"), py_str("broke down"), py_str("in bits")]), py_str("Say the plain thing and then stop talking: \"I'm here, and you don't have to explain.\" A hand on the arm and the silence do more than any advice; when they are ready, ask what they need today - company, a walk, or to be left alone.")]), py_tuple(vec![__data24(), py_str("Say it plainly and skip the silver lining: \"that's rotten, and I'm sorry.\" Let them tell it more than once without running the ex down, and offer one concrete thing this week - a drink, a lift, dinner at yours - rather than \"let me know if you need anything\".")]), py_tuple(vec![py_tuple(vec![py_str("sacked"), py_str("fired"), py_str("redundant"), py_str("unemployed"), py_str("lost her job"), py_str("lost his job"), py_str("lost their job")]), py_str("Lead with them, not with the fix: \"that's a horrible way to start the week - I'm sorry.\" Don't send job links in the first conversation; ask what they want to do today, and offer one practical thing later, like reading over the CV when they ask.")]), py_tuple(vec![py_tuple(vec![py_str("failed"), py_str("flunked"), py_str("rejected"), py_str("gutted"), py_str("didnt get"), py_str("didnt pass"), py_str("turned down"), py_str("missed out")]), py_str("\"That's gutting, and you were right to expect it\" beats \"something better will come along\". Let them be disappointed first; then ask whether they want to talk it through or be distracted, and do whichever they say.")]), py_tuple(vec![py_tuple(vec![py_str("ill"), py_str("sick"), py_str("unwell"), py_str("poorly"), py_str("hospital"), py_str("operation"), py_str("diagnosed"), py_str("diagnosis"), py_str("cancer"), py_str("chemo"), py_str("treatment"), py_str("recovering")]), py_str("Keep it simple and specific: \"I'm thinking of you - is a visit welcome, or would you rather I text?\" Don't ask about the prognosis or offer a cure, and offer a lift, a meal or an afternoon with the kids instead of \"anything I can do\".")]), py_tuple(vec![py_tuple(vec![py_str("homesick"), py_str("lonely"), py_str("missing home"), py_str("moved away"), py_str("knows nobody"), py_str("on her own"), py_str("on his own"), py_str("on their own")]), py_str("Say it out loud: \"that sounds lonely, especially in the evenings.\" Ask about one specific person or place at home, and put a fixed call in the diary - a Tuesday phone call beats \"we must catch up\".")])]));
    fn __data25() -> Py { py_tuple(vec![py_str("feel sick"), py_str("feeling sick"), py_str("felt sick"), py_str("am sick with nerves"), py_str("sick with nerves"), py_str("ill with nerves"), py_str("feel ill"), py_str("queasy"), py_str("butterflies"), py_str("stomach turns"), py_str("stomach turning"), py_str("stomach churns"), py_str("stomach churning"), py_str("churning stomach"), py_str("tummy turns"), py_str("gut churning"), py_str("cant eat"), py_str("can't eat"), py_str("cannot eat"), py_str("dry mouth"), py_str("shaky hands"), py_str("hands shake"), py_str("shaking hands"), py_str("shake all over"), py_str("shaking all over"), py_str("legs like jelly"), py_str("heart racing"), py_str("heart races"), py_str("heart pounding"), py_str("pounding heart"), py_str("sweaty palms"), py_str("clammy"), py_str("cant breathe"), py_str("can't breathe"), py_str("need the loo"), py_str("running to the loo"), py_str("bright red"), py_str("go red"), py_str("blush")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.NERVES_BODY"); }
    s_NERVES_BODY(__data25());
    fn __data26() -> Py { py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("feel sick"), py_str("feeling sick"), py_str("felt sick"), py_str("am sick"), py_str("sick with nerves"), py_str("ill with nerves"), py_str("feel ill"), py_str("feeling ill"), py_str("nauseous"), py_str("nausea")]), py_str("Feeling sick")]), py_tuple(vec![py_tuple(vec![py_str("queasy"), py_str("stomach turns"), py_str("stomach turning"), py_str("stomach churns"), py_str("stomach churning"), py_str("churning stomach"), py_str("tummy turns"), py_str("gut churning")]), py_str("That churning stomach")]), py_tuple(vec![py_tuple(vec![py_str("butterflies")]), py_str("Butterflies")]), py_tuple(vec![py_tuple(vec![py_str("cant eat"), py_str("can't eat"), py_str("cannot eat")]), py_str("Not being able to eat")]), py_tuple(vec![py_tuple(vec![py_str("dry mouth")]), py_str("A dry mouth")]), py_tuple(vec![py_tuple(vec![py_str("shaky hands"), py_str("hands shake"), py_str("shaking hands")]), py_str("Shaky hands")]), py_tuple(vec![py_tuple(vec![py_str("shake all over"), py_str("shaking all over"), py_str("shaking"), py_str("shaky")]), py_str("That shaking")]), py_tuple(vec![py_tuple(vec![py_str("legs like jelly"), py_str("jelly legs"), py_str("weak at the knees")]), py_str("Legs like jelly")]), py_tuple(vec![py_tuple(vec![py_str("heart racing"), py_str("heart races"), py_str("heart pounding"), py_str("pounding heart"), py_str("heart thumping")]), py_str("A racing heart")]), py_tuple(vec![py_tuple(vec![py_str("sweaty palms"), py_str("sweaty hands"), py_str("clammy")]), py_str("Clammy hands")]), py_tuple(vec![py_tuple(vec![py_str("cant breathe"), py_str("can't breathe"), py_str("hard to breathe")]), py_str("That tight chest")]), py_tuple(vec![py_tuple(vec![py_str("need the loo"), py_str("running to the loo")]), py_str("Needing the loo")]), py_tuple(vec![py_tuple(vec![py_str("bright red"), py_str("go red"), py_str("going red"), py_str("blush"), py_str("blushing")]), py_str("Going red")])]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.SIGNS"); }
    s_SIGNS(__data26());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._SIGN_PLURAL"); }
    s__SIGN_PLURAL(call_builtin("frozenset", &[py_tuple(vec![py_str("Shaky hands"), py_str("Butterflies"), py_str("Legs like jelly"), py_str("Clammy hands")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data27() -> Py { py_tuple(vec![py_str("had a go at me"), py_str("had a go at us"), py_str("had a right go"), py_str("a go at me about"), py_str("had a go at me about"), py_str("told me off"), py_str("told us off"), py_str("tells me off"), py_str("telling me off"), py_str("got told off"), py_str("been told off"), py_str("got a telling off"), py_str("a telling off"), py_str("bollocked"), py_str("bollocking"), py_str("got a bollocking"), py_str("gave me an earful"), py_str("got an earful"), py_str("laid into me"), py_str("went off at me"), py_str("read me the riot act"), py_str("had a pop at me"), py_str("gave me a mouthful"), py_str("balled me out")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.TOLD_OFF"); }
    s_TOLD_OFF(__data27());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.COMFORT_SITUATIONS"); }
    s_COMFORT_SITUATIONS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_COMFORT()) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut words: Py = __unpk1[0].clone(); let mut _reply: Py = __unpk1[1].clone(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.COMFORT_UNNAMED"); }
    s_COMFORT_UNNAMED(py_tuple(vec![py_str("upset"), py_str("struggling"), py_str("hard time"), py_str("rough time"), py_str("going through"), py_str("devastated"), py_str("not coping"), py_str("mental health"), py_str("dumped"), py_str("sacked")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.COMFORT_LAST"); }
    s_COMFORT_LAST(py_str("Say it plainly and don't rush to fix it: \"that's really hard, and I'm glad you told me.\" Ask what they need today - company, quiet, or a distraction - and offer one concrete thing, like a walk or dinner on Thursday, rather than \"let me know if you need anything\"."));
    fn __data28() -> Py { py_tuple(vec![py_str("cancel"), py_str("cancels"), py_str("cancelling"), py_str("cancelled"), py_str("canceled"), py_str("bailed"), py_str("bails"), py_str("no show"), py_str("a no show"), py_str("flaked"), py_str("flaky"), py_str("late"), py_str("lateness"), py_str("always late"), py_str("late again"), py_str("never replies"), py_str("does not reply"), py_str("doesnt reply"), py_str("won't reply"), py_str("wont reply"), py_str("never texts"), py_str("doesnt text"), py_str("doesn't text"), py_str("leaves me on read"), py_str("ignores me"), py_str("ignored me"), py_str("ignoring me"), py_str("stopped texting"), py_str("gone quiet"), py_str("left me out"), py_str("leaves me out"), py_str("leaving me out"), py_str("didnt invite"), py_str("didn't invite"), py_str("not invited"), py_str("no invite"), py_str("owes me"), py_str("owes"), py_str("borrowed"), py_str("borrows"), py_str("teased"), py_str("teasing"), py_str("teases"), py_str("made a joke"), py_str("joke about"), py_str("jokes about"), py_str("joking about"), py_str("took the mick"), py_str("rude"), py_str("put me down"), py_str("puts me down"), py_str("putting me down"), py_str("snide"), py_str("nasty about me"), py_str("talks about me"), py_str("behind my back")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.A_BEHAVIOUR_TOWARD_THE_ASKER"); }
    s_A_BEHAVIOUR_TOWARD_THE_ASKER(__data28());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.COMFORT_PLAIN"); }
    s_COMFORT_PLAIN(py_str("You don't need the perfect words: name it in one line (\"that sounds really hard\"), say you are glad they told you, and offer one concrete thing - a walk, dinner on Thursday - rather than \"let me know if you need anything\". Then ask what they need today, company or quiet or a distraction, and do that instead of fixing it."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._BEING_WRITTEN"); }
    s__BEING_WRITTEN(stdlib_call("re", "compile", &[py_str("\\b(?:how|what|whats|what's|should|could|can|would|do i|help|idea|ideas|need to|needs to|want to|wants to|about to|write|writing|wrote|draft|drafting|word|words|say|saying|tell|telling|template|example|examples|letter|email|e-mail|message|advice|tips?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data30() -> Py { py_tuple(vec![py_str("abroad"), py_str("overseas"), py_str("australia"), py_str("canada"), py_str("usa"), py_str("america"), py_str("spain"), py_str("france"), py_str("italy"), py_str("ireland"), py_str("new zealand"), py_str("europe"), py_str("dubai"), py_str("first class"), py_str("second class"), py_str("envelope"), py_str("envelopes")]) }
    fn __data31() -> Py { py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("wishes"), py_str("wish"), py_str("greeting"), py_str("greetings"), py_str("write"), py_str("writing"), py_str("birthday card"), py_str("birthday cards"), py_str("card for"), py_str("cards for"), py_str("birthday message"), py_str("birthday messages"), py_str("birthday wishes"), py_str("bday card")]) }
    fn __data32() -> Py { py_tuple(vec![py_str("make"), py_str("makes"), py_str("making"), py_str("craft"), py_str("draw"), py_str("drawing"), py_str("design"), py_str("print"), py_str("cut out"), py_str("card shop"), py_str("buy"), py_str("buying"), py_str("bought"), py_str("shop"), py_str("shops"), py_str("cheap"), py_str("price"), py_str("where can i"), py_str("abroad"), py_str("overseas"), py_str("australia"), py_str("canada"), py_str("usa"), py_str("america"), py_str("spain"), py_str("france"), py_str("italy"), py_str("ireland"), py_str("new zealand"), py_str("europe"), py_str("dubai"), py_str("first class"), py_str("second class"), py_str("envelope"), py_str("envelopes")]) }
    fn __data33() -> Py { py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("wishes"), py_str("wish"), py_str("write"), py_str("writing"), py_str("greeting"), py_str("greetings"), py_str("what do i write"), py_str("what to write")]) }
    fn __data34() -> Py { py_tuple(vec![py_str("leaving"), py_str("leaving do"), py_str("last day"), py_str("goodbye"), py_str("farewell"), py_str("moving on"), py_str("emigrating"), py_str("retiring"), py_str("retirement"), py_str("removals"), py_str("van"), py_str("packing"), py_str("deposit"), py_str("mortgage"), py_str("house price"), py_str("buying a house"), py_str("moved out"), py_str("get married"), py_str("speech"), py_str("toast"), py_str("venue"), py_str("guest list")]) }
    fn __data35() -> Py { py_tuple(vec![py_str("refund"), py_str("refunds"), py_str("complaint"), py_str("complain"), py_str("complaining"), py_str("compensation"), py_str("replacement"), py_str("money back"), py_str("owed"), py_str("owes me"), py_str("reference"), py_str("references"), py_str("referee"), py_str("referees"), py_str("testimonial"), py_str("recommendation"), py_str("cv"), py_str("job application"), py_str("employer"), py_str("interview"), py_str("how much"), py_str("how many"), py_str("cash"), py_str("money"), py_str("quid"), py_str("pounds"), py_str("dollars"), py_str("euros")]) }
    fn __data36() -> Py { py_tuple(vec![py_str("buy"), py_str("buying"), py_str("bought"), py_str("present"), py_str("presents"), py_str("gift"), py_str("gifts"), py_str("ideas"), py_str("what should i get"), py_str("cheap"), py_str("price"), py_str("restaurant"), py_str("book a table"), py_str("birthday")]) }
    fn __data37() -> Py { py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("text"), py_str("texts"), py_str("note"), py_str("wishes"), py_str("congratulations"), py_str("congrats"), py_str("congratulate"), py_str("congratulating"), py_str("write"), py_str("writing")]) }
    fn __data38() -> Py { py_tuple(vec![py_str("new baby"), py_str("newborn"), py_str("baby"), py_str("babies"), py_str("birth"), py_str("born"), py_str("has arrived"), py_str("have arrived"), py_str("just arrived"), py_str("arrived yesterday"), py_str("arrived last night"), py_str("arrived this morning"), py_str("having a baby"), py_str("had a baby"), py_str("baby on the way")]) }
    fn __data39() -> Py { py_tuple(vec![py_str("buy"), py_str("buying"), py_str("bought"), py_str("shop"), py_str("shops"), py_str("present"), py_str("presents"), py_str("gift"), py_str("gifts"), py_str("make"), py_str("making"), py_str("craft"), py_str("take"), py_str("taking"), py_str("bring"), py_str("visiting")]) }
    fn __data40() -> Py { py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("text"), py_str("texts"), py_str("note"), py_str("wishes"), py_str("congratulations"), py_str("congrats"), py_str("congratulate"), py_str("congratulating"), py_str("write"), py_str("writing"), py_str("what do i write"), py_str("what to write")]) }
    fn __data41() -> Py { py_tuple(vec![py_str("how long"), py_str("how soon"), py_str("recover from"), py_str("back to work"), py_str("pain"), py_str("take"), py_str("taking"), py_str("bring"), py_str("brings"), py_str("give them"), py_str("visit"), py_str("visiting"), py_str("better at"), py_str("get better at"), py_str("improve"), py_str("improving"), py_str("improvement"), py_str("mp"), py_str("msp"), py_str("councillor"), py_str("council"), py_str("landlord"), py_str("letting agent"), py_str("insurer"), py_str("insurance"), py_str("employer"), py_str("appeal"), py_str("ombudsman"), py_str("dwp")]) }
    fn __data42() -> Py { py_tuple(vec![py_str("make"), py_str("makes"), py_str("making"), py_str("craft"), py_str("draw"), py_str("design"), py_str("print"), py_str("cut out"), py_str("buy"), py_str("buying"), py_str("bought"), py_str("shop"), py_str("shops"), py_str("cheap"), py_str("price"), py_str("where can i")]) }
    fn __data43() -> Py { py_tuple(vec![py_str("deposit"), py_str("moved out"), py_str("moving out"), py_str("washing machine"), py_str("dishwasher"), py_str("fridge"), py_str("freezer"), py_str("oven"), py_str("tumble dryer"), py_str("vacuum"), py_str("hoover"), py_str("car"), py_str("laptop"), py_str("phone")]) }
    fn __data44() -> Py { py_tuple(vec![py_str("landlord"), py_str("landlady"), py_str("letting agent"), py_str("letting agency"), py_str("housing"), py_str("council"), py_str("letter"), py_str("email"), py_str("message"), py_str("text"), py_str("note"), py_str("write"), py_str("writing"), py_str("word"), py_str("wording"), py_str("complain"), py_str("complaint"), py_str("rights"), py_str("refusing"), py_str("refuses"), py_str("ignoring"), py_str("honour")]) }
    fn __data45() -> Py { py_tuple(vec![py_str("aggressive"), py_str("aggression"), py_str("attacks"), py_str("attacking"), py_str("attacked"), py_str("bites"), py_str("bitten"), py_str("bit my"), py_str("bit me"), py_str("growls"), py_str("growling"), py_str("growled"), py_str("snarls"), py_str("snarling"), py_str("snapped at"), py_str("lunges"), py_str("lunging"), py_str("went for"), py_str("goes for"), py_str("dangerous"), py_str("vicious"), py_str("savage"), py_str("out of control"), py_str("terrified of it"), py_str("scared of it"), py_str("chased"), py_str("chasing")]) }
    fn __data46() -> Py { py_tuple(vec![py_str("neighbours"), py_str("neighbour"), py_str("neighbors"), py_str("neighbor"), py_str("next door"), py_str("upstairs"), py_str("downstairs"), py_str("flat above"), py_str("someones dog"), py_str("someone else's dog"), py_str("someone elses dog"), py_str("someone's dog"), py_str("a dog down the road"), py_str("dog down the road"), py_str("their dog")]) }
    fn __data47() -> Py { py_tuple(vec![py_str("music"), py_str("banging"), py_str("barking"), py_str("bark"), py_str("dog"), py_str("party"), py_str("message"), py_str("talk"), py_str("wont shut up"), py_str("won't shut up"), py_str("shut up"), py_str("never shuts up"), py_str("keeps barking"), py_str("drill"), py_str("drilling"), py_str("hammering"), py_str("sawing"), py_str("building work"), py_str("builders"), py_str("renovation"), py_str("renovating"), py_str("extension")]) }
    fn __data48() -> Py { py_tuple(vec![py_str("music"), py_str("banging"), py_str("barking"), py_str("barks"), py_str("bark"), py_str("howling"), py_str("howls"), py_str("all night"), py_str("at night"), py_str("every night"), py_str("all hours"), py_str("cant sleep"), py_str("can't sleep"), py_str("awake"), py_str("wake"), py_str("woken"), py_str("party"), py_str("keep me up"), py_str("keeps me up"), py_str("shut up"), py_str("goes mad"), py_str("yapping"), py_str("whining"), py_str("screaming"), py_str("disturbing"), py_str("drill"), py_str("drilling"), py_str("hammering"), py_str("sawing"), py_str("building work"), py_str("builders"), py_str("renovation"), py_str("renovating"), py_str("extension")]) }
    fn __data49() -> Py { py_tuple(vec![py_str("fun"), py_str("enjoyed"), py_str("enjoy"), py_str("enjoying"), py_str("loved"), py_str("love"), py_str("lovely"), py_str("great"), py_str("nice"), py_str("thanks"), py_str("wedding"), py_str("birthday"), py_str("sad"), py_str("upset"), py_str("anxious"), py_str("worried"), py_str("lonely")]) }
    fn __data50() -> Py { py_tuple(vec![py_str("service"), py_str("back"), py_str("message"), py_str("money"), py_str("broke"), py_str("broken"), py_str("faulty"), py_str("write"), py_str("writing"), py_str("email"), py_str("words"), py_str("word"), py_str("draft"), py_str("letter"), py_str("say"), py_str("saying"), py_str("ask"), py_str("asking"), py_str("phrasing"), py_str("asking for"), py_str("letter asking"), py_str("asking for a refund"), py_str("asking for my money back"), py_str("letter of complaint"), py_str("email asking"), py_str("write asking")]) }
    fn __data51() -> Py { py_tuple(vec![py_str("write"), py_str("writing"), py_str("email"), py_str("message"), py_str("say"), py_str("saying"), py_str("words"), py_str("word"), py_str("ask"), py_str("asking"), py_str("how do i ask"), py_str("draft"), py_str("letter"), py_str("text"), py_str("reply"), py_str("phrasing"), py_str("complain"), py_str("complaining"), py_str("complaint")]) }
    fn __data52() -> Py { py_tuple(vec![py_str("shop"), py_str("shops"), py_str("in store"), py_str("store i"), py_str("receipt"), py_str("entitled"), py_str("consumer rights"), py_str("my rights"), py_str("30 days"), py_str("thirty days"), py_str("can i get"), py_str("am i entitled"), py_str("are they allowed"), py_str("do i have to"), py_str("is it legal"), py_str("should they"), py_str("how long"), py_str("train"), py_str("trains"), py_str("rail"), py_str("railway"), py_str("railcard"), py_str("delay repay"), py_str("flight"), py_str("flights"), py_str("airline"), py_str("plane"), py_str("hotel"), py_str("booking"), py_str("concert"), py_str("gig"), py_str("festival"), py_str("theatre"), py_str("theater"), py_str("cinema"), py_str("match"), py_str("festival ticket"), py_str("train ticket"), py_str("bus ticket"), py_str("coach ticket"), py_str("season ticket"), py_str("ferry"), py_str("ticket"), py_str("tickets"), py_str("holiday"), py_str("package holiday"), py_str("cruise"), py_str("travel agent"), py_str("tour operator"), py_str("cottage"), py_str("villa"), py_str("campsite"), py_str("tax"), py_str("tax refund"), py_str("tax rebate"), py_str("rebate"), py_str("hmrc"), py_str("p800"), py_str("benefit"), py_str("benefits"), py_str("universal credit"), py_str("council tax"), py_str("pension"), py_str("text saying"), py_str("email saying"), py_str("emailed saying"), py_str("rang saying"), py_str("called saying"), py_str("phoned saying"), py_str("message saying"), py_str("telling me"), py_str("saying im"), py_str("saying i am")]) }
    fn __data53() -> Py { py_tuple(vec![py_str("friend"), py_str("sister"), py_str("brother"), py_str("mum"), py_str("dad"), py_str("partner"), py_str("said"), py_str("hurt"), py_str("upset"), py_str("row"), py_str("argument"), py_str("fight"), py_str("write"), py_str("message"), py_str("properly"), py_str("wife"), py_str("husband"), py_str("girlfriend"), py_str("boyfriend"), py_str("partner"), py_str("fiance"), py_str("fiancee"), py_str("flatmate"), py_str("housemate"), py_str("neighbour"), py_str("neighbor"), py_str("son"), py_str("daughter"), py_str("mother"), py_str("father"), py_str("colleague"), py_str("say"), py_str("tell"), py_str("words"), py_str("wording")]) }
    fn __data54() -> Py { py_tuple(vec![py_str("to me"), py_str("said sorry"), py_str("says sorry"), py_str("said it"), py_str("someone said"), py_str("someone apologised"), py_str("someone apologizes"), py_str("someone apologises"), py_str("reply"), py_str("respond"), py_str("answer"), py_str("accept"), py_str("accepting"), py_str("forgive"), py_str("forgiving"), py_str("what do i say"), py_str("what should i say"), py_str("how do i reply"), py_str("how do i respond"), py_str("they said sorry"), py_str("he said sorry"), py_str("she said sorry"), py_str("apologised to me"), py_str("apologized to me"), py_str("apologises to me"), py_str("apologizes to me"), py_str("said sorry to me"), py_str("says sorry to me")]) }
    fn __data55() -> Py { py_tuple(vec![py_str("sorry"), py_str("apology"), py_str("apologise"), py_str("apologize"), py_str("say"), py_str("saying"), py_str("what"), py_str("how"), py_str("reply"), py_str("respond"), py_str("accept"), py_str("forgive"), py_str("back"), py_str("after"), py_str("when")]) }
    fn __data56() -> Py { py_tuple(vec![py_str("i said sorry"), py_str("i already said sorry"), py_str("i apologise"), py_str("i apologize"), py_str("i have apologised"), py_str("how do i say sorry"), py_str("how to say sorry"), py_str("should i apologise"), py_str("should i say sorry"), py_str("wont apologise"), py_str("wont say sorry"), py_str("never apologises"), py_str("never says sorry"), py_str("owes me an apology"), py_str("owe me an apology"), py_str("refuses to apologise"), py_str("refuses to say sorry"), py_str("stop saying sorry"), py_str("sorry for your loss"), py_str("condolence"), py_str("died"), py_str("funeral"), py_str("passed away"), py_str("grief"), py_str("how do i write"), py_str("how to write"), py_str("write a text"), py_str("write a message"), py_str("text to say"), py_str("draft"), py_str("drafting"), py_str("wording"), py_str("how do i say"), py_str("how to say"), py_str("should i send")]) }
    fn __data57() -> Py { py_tuple(vec![py_str("wont apologise"), py_str("wont say sorry"), py_str("wont apologise to me"), py_str("never apologises"), py_str("never apologize"), py_str("never apologise"), py_str("never says sorry"), py_str("never say sorry"), py_str("doesnt apologise"), py_str("doesnt say sorry"), py_str("not apologising"), py_str("refuses to apologise"), py_str("refuses to say sorry"), py_str("owes me an apology"), py_str("owe me an apology"), py_str("owes me a sorry"), py_str("apologise to me"), py_str("apologize to me"), py_str("should apologise to me"), py_str("should say sorry to me"), py_str("should have apologised"), py_str("get him to apologise"), py_str("get her to apologise"), py_str("get them to apologise"), py_str("wont admit he"), py_str("wont admit she"), py_str("never admits"), py_str("never admits he is wrong")]) }
    fn __data58() -> Py { py_tuple(vec![py_str("my husband"), py_str("my wife"), py_str("my partner"), py_str("my mum"), py_str("my dad"), py_str("my friend"), py_str("my boss"), py_str("my sister"), py_str("my brother"), py_str("my boyfriend"), py_str("my girlfriend"), py_str("my son"), py_str("my daughter"), py_str("my mother"), py_str("my father"), py_str("he"), py_str("she"), py_str("they"), py_str("him"), py_str("her"), py_str("them"), py_str("his"), py_str("himself"), py_str("herself"), py_str("how"), py_str("what"), py_str("should"), py_str("can"), py_str("why"), py_str("never")]) }
    fn __data59() -> Py { py_tuple(vec![py_str("never"), py_str("wont"), py_str("doesnt"), py_str("owes"), py_str("owe"), py_str("owed"), py_str("should"), py_str("refuses"), py_str("to me"), py_str("how"), py_str("what"), py_str("why"), py_str("can")]) }
    fn __data60() -> Py { py_tuple(vec![py_str("died"), py_str("death"), py_str("funeral"), py_str("passed away"), py_str("sorry i"), py_str("im sorry"), py_str("my fault"), py_str("stop saying sorry"), py_str("all the time"), py_str("too much"), py_str("so much"), py_str("i apologise"), py_str("i apologize"), py_str("i said sorry"), py_str("i should say sorry"), py_str("what do i say"), py_str("what should i say"), py_str("what do i write"), py_str("reply"), py_str("respond"), py_str("accept"), py_str("said sorry to me"), py_str("says sorry to me"), py_str("apologises to me"), py_str("apologizes to me"), py_str("apologised to me"), py_str("apologized to me"), py_str("when someone apologises"), py_str("when someone apologized"), py_str("when someone says sorry")]) }
    fn __data61() -> Py { py_tuple(vec![py_str("with a cold"), py_str("a cold"), py_str("with flu"), py_str("a cough"), py_str("temperature"), py_str("chest infection"), py_str("thank you"), py_str("thankyou"), py_str("thank-you"), py_str("thanks"), py_str("vomiting"), py_str("sickness bug"), py_str("chickenpox"), py_str("off school"), py_str("home from school"), py_str("keep my child home"), py_str("well enough"), py_str("should i keep"), py_str("keep him home"), py_str("keep her home"), py_str("keep them home"), py_str("when can he"), py_str("when can she"), py_str("how long off"), py_str("bullied"), py_str("bullying"), py_str("bullies"), py_str("picked on"), py_str("being picked on"), py_str("my teacher"), py_str("my lecturer"), py_str("my tutor"), py_str("my professor"), py_str("my form tutor"), py_str("choose a school"), py_str("choosing a school"), py_str("choose the school"), py_str("which school"), py_str("what school"), py_str("schools to look at"), py_str("look at schools"), py_str("visit schools"), py_str("school visits"), py_str("school open day"), py_str("compare schools"), py_str("pick a school"), py_str("picking a school"), py_str("decide on a school"), py_str("school for my child"), py_str("school for my son"), py_str("school for my daughter")]) }
    fn __data62() -> Py { py_tuple(vec![py_str("report my child absent"), py_str("report an absence"), py_str("report the absence"), py_str("report my child off"), py_str("absent from school"), py_str("absence from school"), py_str("school absence"), py_str("school absences"), py_str("note for school"), py_str("note to school"), py_str("letter to school"), py_str("letter for school"), py_str("email the school"), py_str("email school about"), py_str("write to the school"), py_str("write to school about"), py_str("sick note"), py_str("school note"), py_str("note about my child"), py_str("letter about my child"), py_str("off school note")]) }
    fn __data63() -> Py { py_tuple(vec![py_str("uniform"), py_str("uniforms"), py_str("blazer"), py_str("pinafore"), py_str("pe kit"), py_str("shoes"), py_str("socks"), py_str("stain"), py_str("stains"), py_str("school run"), py_str("dropping off"), py_str("drop off"), py_str("choose a school"), py_str("choosing a school"), py_str("which school"), py_str("school place"), py_str("admissions"), py_str("ofsted"), py_str("catchment")]) }
    fn __data64() -> Py { py_tuple(vec![py_str("time off"), py_str("holiday"), py_str("holidays"), py_str("day off"), py_str("days off"), py_str("week off"), py_str("weeks off"), py_str("week off work"), py_str("leave early"), py_str("leaving early"), py_str("finish early"), py_str("half day"), py_str("an hour early"), py_str("early on friday"), py_str("dentist"), py_str("doctor"), py_str("appointment"), py_str("friday off"), py_str("monday off"), py_str("tuesday off"), py_str("wednesday off"), py_str("thursday off"), py_str("tomorrow off")]) }
    fn __data65() -> Py { py_tuple(vec![py_str("boss"), py_str("request"), py_str("email"), py_str("book"), py_str("work"), py_str("ask"), py_str("asking"), py_str("leave"), py_str("leaving"), py_str("take"), py_str("can i"), py_str("could i"), py_str("off")]) }
    fn __data66() -> Py { py_tuple(vec![py_str("sick"), py_str("ill"), py_str("unwell"), py_str("poorly"), py_str("not well"), py_str("temperature"), py_str("fever"), py_str("flu"), py_str("migraine"), py_str("headache"), py_str("vomiting"), py_str("throwing up"), py_str("with the kids"), py_str("with my kids"), py_str("with my children"), py_str("with the children"), py_str("kids off school"), py_str("school holidays"), py_str("half term"), py_str("another job"), py_str("new job"), py_str("second job"), py_str("quit"), py_str("quitting"), py_str("leave my job"), py_str("lose weight"), py_str("lose a stone"), py_str("losing weight"), py_str("diet"), py_str("get fit"), py_str("gym"), py_str("entitled"), py_str("entitlement"), py_str("statutory"), py_str("pro rata"), py_str("pro-rata"), py_str("minimum"), py_str("how many days holiday"), py_str("how much holiday"), py_str("how many holidays"), py_str("putting"), py_str("postponing"), py_str("procrastinating"), py_str("calls me"), py_str("calling me"), py_str("called me"), py_str("rings me"), py_str("rang me"), py_str("ring me"), py_str("ringing me"), py_str("texts me"), py_str("texted me"), py_str("texting me"), py_str("messages me"), py_str("phoned me"), py_str("phones me"), py_str("keeps calling"), py_str("keeps ringing"), py_str("keeps texting"), py_str("contacts me"), py_str("expects me at work"), py_str("pay off"), py_str("paying off"), py_str("credit card"), py_str("card debt"), py_str("card payment"), py_str("debts"), py_str("debt"), py_str("savings"), py_str("save up"), py_str("saving up"), py_str("or save"), py_str("overpay"), py_str("mortgage"), py_str("interest"), py_str("where can i go"), py_str("where should i go"), py_str("where could i go"), py_str("where to go"), py_str("where would you go"), py_str("where should we go"), py_str("where can we go"), py_str("where could we go"), py_str("where would we go"), py_str("which country"), py_str("somewhere warm"), py_str("what should i do"), py_str("what can i do"), py_str("what to do"), py_str("any ideas"), py_str("any tips"), py_str("no idea"), py_str("nothing planned"), py_str("nothing to do"), py_str("keep busy"), py_str("bored"), py_str("cook"), py_str("cooking"), py_str("bake"), py_str("baking"), py_str("meal"), py_str("meals"), py_str("dinner"), py_str("dinners"), py_str("lunch"), py_str("lunches"), py_str("recipe"), py_str("recipes"), py_str("save"), py_str("saving"), py_str("savings"), py_str("save up"), py_str("money"), py_str("budget"), py_str("cost"), py_str("afford"), py_str("quid"), py_str("pounds"), py_str("broke"), py_str("skint"), py_str("hard up"), py_str("no money"), py_str("tight month"), py_str("on a budget"), py_str("plan a trip"), py_str("planning a trip"), py_str("book a trip"), py_str("a trip"), py_str("trip with"), py_str("going away"), py_str("go away for"), py_str("get away for"), py_str("a break away"), py_str("book a holiday"), py_str("plan a holiday"), py_str("planning a holiday")]) }
    fn __data67() -> Py { py_tuple(vec![py_str("chase"), py_str("chasing"), py_str("chased"), py_str("follow up"), py_str("followed up"), py_str("follow-up"), py_str("following up"), py_str("follows up"), py_str("chase up"), py_str("chasing up"), py_str("email"), py_str("emailed"), py_str("emailing"), py_str("note"), py_str("reply"), py_str("replies"), py_str("write"), py_str("writing"), py_str("message"), py_str("nudge")]) }
    fn __data68() -> Py { py_tuple(vec![py_str("interview"), py_str("interviews"), py_str("application"), py_str("applications"), py_str("applied"), py_str("recruiter"), py_str("hiring manager"), py_str("panel"), py_str("job offer"), py_str("about a job"), py_str("for a job"), py_str("the job"), py_str("vacancy"), py_str("role"), py_str("following up"), py_str("follows up"), py_str("follow up"), py_str("chase up"), py_str("chasing up"), py_str("chasing")]) }
    fn __data69() -> Py { py_tuple(vec![py_str("never replies"), py_str("never reply"), py_str("never answers"), py_str("never answer"), py_str("never emails"), py_str("letter of application"), py_str("wont reply"), py_str("won't reply"), py_str("wont answer"), py_str("doesnt reply"), py_str("doesn't reply"), py_str("doesnt answer"), py_str("doesn't answer"), py_str("ignores my"), py_str("ignored my"), py_str("no reply"), py_str("no response"), py_str("nothing back"), py_str("my manager"), py_str("my boss"), py_str("my employer"), py_str("chased twice"), py_str("asked twice"), py_str("cover letter"), py_str("covering letter"), py_str("results"), py_str("blood test"), py_str("bloods"), py_str("scan"), py_str("x-ray"), py_str("smear"), py_str("biopsy"), py_str("gp"), py_str("doctor"), py_str("hospital"), py_str("surgery"), py_str("nurse"), py_str("test result"), py_str("appointment"), py_str("after applying"), py_str("after i applied"), py_str("after you apply"), py_str("after my application"), py_str("after sending my cv"), py_str("after sending my application"), py_str("after sending the application"), py_str("after sending it"), py_str("after the closing date"), py_str("after the closing date should")]) }
    fn __data70() -> Py { py_tuple(vec![py_str("cooking"), py_str("cooks"), py_str("baking"), py_str("bakes"), py_str("food"), py_str("singing"), py_str("haircut"), py_str("outfit"), py_str("writing"), py_str("drawing"), py_str("performance"), py_str("cheat"), py_str("cheats"), py_str("cheated"), py_str("cheating"), py_str("affair"), py_str("affairs"), py_str("like her"), py_str("like him"), py_str("like them"), py_str("fancy"), py_str("feelings"), py_str("a crush"), py_str("in love"), py_str("depression"), py_str("depressed"), py_str("anxiety"), py_str("anxious"), py_str("mental health"), py_str("diagnosis"), py_str("coming out"), py_str("gay"), py_str("lesbian"), py_str("trans"), py_str("cant come"), py_str("cant make it"), py_str("not coming"), py_str("cant be there"), py_str("cant go"), py_str("wont be able to come"), py_str("have to cancel"), py_str("had to cancel"), py_str("cannot make it"), py_str("wedding"), py_str("christening"), py_str("funeral"), py_str("engagement"), py_str("hen do"), py_str("stag do")]) }
    fn __data71() -> Py { py_tuple(vec![py_str("wedding"), py_str("birthday"), py_str("christening"), py_str("party"), py_str("dinner"), py_str("christmas"), py_str("lunch"), py_str("visit"), py_str("meal"), py_str("funeral"), py_str("barbecue"), py_str("bbq"), py_str("engagement"), py_str("hen do"), py_str("stag do")]) }
    fn __data72() -> Py { py_tuple(vec![py_str("how do i tell"), py_str("how should i tell"), py_str("how to tell"), py_str("how do i break"), py_str("how do i ask"), py_str("how should i ask"), py_str("how can i ask"), py_str("how to ask"), py_str("break the news"), py_str("how do i say"), py_str("how do i bring it up"), py_str("how do i bring up"), py_str("bring it up with"), py_str("how do i raise it"), py_str("how do i talk to my"), py_str("talk to my"), py_str("how do i tell my"), py_str("telling my"), py_str("how do i announce"), py_str("talks over me"), py_str("talking over me"), py_str("talk over me"), py_str("talks over my"), py_str("keeps talking over"), py_str("interrupts me"), py_str("interrupting me"), py_str("interrupts my"), py_str("keeps interrupting"), py_str("cuts me off"), py_str("keeps cutting me off"), py_str("butts in"), py_str("keeps butting in"), py_str("always butts in"), py_str("butting in on"), py_str("finishes my sentences"), py_str("how do i give"), py_str("how to give"), py_str("how do i break it"), py_str("how to break it"), py_str("words to say"), py_str("words to use"), py_str("words to tell"), py_str("give me the words"), py_str("help me word"), py_str("put it in words"), py_str("put this in words"), py_str("find the words"), py_str("what words"), py_str("which words")]) }
    fn __data73() -> Py { py_tuple(vec![py_str("parents"), py_str("parent"), py_str("mum"), py_str("dad"), py_str("mother"), py_str("father"), py_str("partner"), py_str("husband"), py_str("wife"), py_str("housemate"), py_str("housemates"), py_str("flatmate"), py_str("flatmates"), py_str("roommate"), py_str("family"), py_str("my friend"), py_str("my friends"), py_str("brother"), py_str("sister"), py_str("son"), py_str("daughter"), py_str("kids"), py_str("children"), py_str("my boss"), py_str("my team"), py_str("team"), py_str("staff"), py_str("colleagues"), py_str("everyone"), py_str("someone"), py_str("somebody"), py_str("person"), py_str("people"), py_str("friend"), py_str("friends"), py_str("neighbour"), py_str("neighbours"), py_str("neighbor"), py_str("neighbors"), py_str("my neighbour"), py_str("housemates"), py_str("flatmates"), py_str("roommates"), py_str("workmate"), py_str("workmates"), py_str("coworker"), py_str("co-worker"), py_str("boyfriend"), py_str("girlfriend"), py_str("fiance"), py_str("fiancee"), py_str("my ex"), py_str("ex partner"), py_str("nan"), py_str("nana"), py_str("grandma"), py_str("grandad"), py_str("granny"), py_str("grandparent"), py_str("in laws"), py_str("in-law"), py_str("stepson"), py_str("stepdaughter"), py_str("stepchildren")]) }
    fn __data74() -> Py { py_tuple(vec![py_str("tell"), py_str("say"), py_str("ask"), py_str("asking"), py_str("asks"), py_str("bring"), py_str("raise"), py_str("talk"), py_str("breaking"), py_str("announce"), py_str("telling"), py_str("give"), py_str("giving"), py_str("talks over me"), py_str("talking over me"), py_str("talk over me"), py_str("talks over my"), py_str("keeps talking over"), py_str("interrupts me"), py_str("interrupting me"), py_str("interrupts my"), py_str("keeps interrupting"), py_str("cuts me off"), py_str("keeps cutting me off"), py_str("butts in"), py_str("keeps butting in"), py_str("always butts in"), py_str("butting in on"), py_str("finishes my sentences")]) }
    fn __data75() -> Py { py_tuple(vec![py_str("cant come"), py_str("cant make it"), py_str("not coming"), py_str("cannot make it"), py_str("have to cancel"), py_str("wedding"), py_str("funeral"), py_str("hen do"), py_str("stag do"), py_str("resign"), py_str("resigning"), py_str("leaving my job"), py_str("quit"), py_str("notice period"), py_str("santa"), py_str("father christmas"), py_str("died"), py_str("dying"), py_str("death"), py_str("dead"), py_str("passed away"), py_str("put down"), py_str("im leaving"), py_str("i'm leaving"), py_str("leaving my job"), py_str("leaving the job"), py_str("leaving work"), py_str("leaving the company"), py_str("resigning"), py_str("resign"), py_str("my notice"), py_str("notice period"), py_str("divorce"), py_str("divorced"), py_str("divorcing"), py_str("divorce papers"), py_str("getting divorced"), py_str("separating"), py_str("separated"), py_str("splitting up"), py_str("diagnosis"), py_str("diagnosed"), py_str("depression"), py_str("depressed"), py_str("anxiety"), py_str("mental health"), py_str("coming out"), py_str("gay"), py_str("lesbian"), py_str("trans"), py_str("cheated"), py_str("cheating"), py_str("affair"), py_str("betrayed"), py_str("say no"), py_str("saying no"), py_str("no to my"), py_str("how to say no"), py_str("lend"), py_str("lending"), py_str("loan"), py_str("borrow"), py_str("borrowing"), py_str("lend them"), py_str("owes me"), py_str("owe me"), py_str("owed me"), py_str("owes me money"), py_str("pay me back"), py_str("pays me back"), py_str("paid me back"), py_str("get it back"), py_str("money back"), py_str("pay rise"), py_str("payrise"), py_str("raise"), py_str("promotion"), py_str("pay review"), py_str("more money"), py_str("time off"), py_str("day off"), py_str("days off"), py_str("annual leave"), py_str("holiday"), py_str("holidays"), py_str("im busy"), py_str("i'm busy"), py_str("dont want to go"), py_str("don't want to go"), py_str("doesnt want to go"), py_str("cant go"), py_str("can't go"), py_str("turned down"), py_str("turn down"), py_str("not invited"), py_str("uninvited"), py_str("my boss"), py_str("boss"), py_str("manager"), py_str("colleagues"), py_str("colleague"), py_str("in meetings"), py_str("in a meeting"), py_str("at work"), py_str("standup"), py_str("stand-up"), py_str("line manager"), py_str("work from home"), py_str("working from home"), py_str("wfh"), py_str("my kids interrupt"), py_str("the kids interrupt"), py_str("children interrupt"), py_str("kids keep interrupting"), py_str("tell off"), py_str("telling off"), py_str("told off"), py_str("tell him off"), py_str("tell her off"), py_str("tell them off"), py_str("tell my kid off"), py_str("tell my son off"), py_str("tell my daughter off"), py_str("tick off")]) }
    fn __data76() -> Py { py_tuple(vec![py_str("real"), py_str("pretend"), py_str("truth"), py_str("isnt real"), py_str("doesnt exist"), py_str("believe"), py_str("believes"), py_str("belief"), py_str("tell"), py_str("telling"), py_str("how old"), py_str("age"), py_str("found out"), py_str("teased")]) }
    fn __data77() -> Py { py_tuple(vec![py_str("tell"), py_str("telling"), py_str("told"), py_str("should i tell"), py_str("do i tell"), py_str("break it"), py_str("break the news"), py_str("worked out"), py_str("work out"), py_str("found out"), py_str("stop believing"), py_str("keep believing"), py_str("now she knows"), py_str("now he knows"), py_str("the truth about santa")]) }
    fn __data78() -> Py { py_tuple(vec![py_str("died"), py_str("death"), py_str("dying"), py_str("passed away"), py_str("divorce"), py_str("separating"), py_str("splitting up"), py_str("cancer"), py_str("ill"), py_str("illness"), py_str("diagnosis"), py_str("job"), py_str("redundant"), py_str("sacked"), py_str("fired"), py_str("santa"), py_str("pregnant"), py_str("baby")]) }
    fn __data79() -> Py { py_tuple(vec![py_str("say"), py_str("said"), py_str("saying"), py_str("text"), py_str("message"), py_str("write"), py_str("words"), py_str("reply"), py_str("respond"), py_str("react"), py_str("answer"), py_str("congratulate"), py_str("congratulations"), py_str("congrats"), py_str("tell her"), py_str("tell him"), py_str("tell them"), py_str("what do i say back")]) }
    fn __data80() -> Py { py_tuple(vec![py_str("if i am"), py_str("if im"), py_str("if i'm"), py_str("asks if"), py_str("asked if"), py_str("asking if"), py_str("whether i am"), py_str("whether im"), py_str("do you think i am"), py_str("am i pregnant"), py_str("tell my boss"), py_str("tell my employer"), py_str("tell my manager"), py_str("maternity"), py_str("paternity"), py_str("am i"), py_str("could i be"), py_str("test"), py_str("trying"), py_str("conceive"), py_str("ivf"), py_str("bleeding"), py_str("cramps"), py_str("miscarriage"), py_str("losing the baby"), py_str("symptoms"), py_str("due date"), py_str("weeks"), py_str("buy"), py_str("buying"), py_str("present"), py_str("presents"), py_str("gift"), py_str("gifts"), py_str("visit"), py_str("visiting"), py_str("take round"), py_str("payslip"), py_str("payslips"), py_str("pay slip"), py_str("wages"), py_str("wage"), py_str("salary"), py_str("salaries"), py_str("paid"), py_str("underpaid"), py_str("overpaid"), py_str("tax"), py_str("short this month")]) }
    fn __data81() -> Py { py_tuple(vec![py_str("divorce"), py_str("divorcing"), py_str("divorced"), py_str("separating"), py_str("separated"), py_str("separation"), py_str("splitting up"), py_str("split up"), py_str("breaking up"), py_str("broke up"), py_str("marriage is over"), py_str("leaving her husband"), py_str("leaving his wife"), py_str("left her husband"), py_str("left his wife"), py_str("walked out")]) }
    fn __data82() -> Py { py_tuple(vec![py_str("lawyer"), py_str("solicitor"), py_str("legal"), py_str("custody"), py_str("child maintenance"), py_str("csa"), py_str("house"), py_str("mortgage"), py_str("deposit"), py_str("pension"), py_str("split the money"), py_str("who gets"), py_str("petition"), py_str("fees"), py_str("i want to"), py_str("how do i tell"), py_str("tell my husband"), py_str("tell my wife"), py_str("i'm leaving"), py_str("im leaving"), py_str("i cheated"), py_str("affair")]) }
    fn __data83() -> Py { py_tuple(vec![py_str("diagnosed"), py_str("diagnosis"), py_str("cancer"), py_str("tumour"), py_str("tumor"), py_str("chemo"), py_str("chemotherapy"), py_str("radiotherapy"), py_str("ill"), py_str("illness"), py_str("unwell"), py_str("poorly"), py_str("condition"), py_str("multiple sclerosis"), py_str("parkinsons"), py_str("crohns"), py_str("epilepsy"), py_str("diabetes"), py_str("lupus"), py_str("heart condition"), py_str("has hiv")]) }
    fn __data84() -> Py { py_tuple(vec![py_str("symptoms"), py_str("treatment"), py_str("cure"), py_str("diet"), py_str("do i have"), py_str("do they have"), py_str("tell work"), py_str("tell my boss"), py_str("survival"), py_str("prognosis"), py_str("stages"), py_str("is it"), py_str("am i"), py_str("are these"), py_str("could it be"), py_str("died"), py_str("dying"), py_str("passed away"), py_str("funeral"), py_str("terminal"), py_str("buy"), py_str("buying"), py_str("present"), py_str("presents"), py_str("gift"), py_str("gifts"), py_str("visit"), py_str("visiting"), py_str("take round")]) }
    fn __data85() -> Py { py_tuple(vec![py_str("lost his job"), py_str("lost her job"), py_str("lost their job"), py_str("made redundant"), py_str("being made redundant"), py_str("redundant"), py_str("redundancy"), py_str("laid off"), py_str("losing his job"), py_str("losing her job"), py_str("losing their job"), py_str("been sacked"), py_str("got sacked"), py_str("been fired"), py_str("got fired"), py_str("let go from work"), py_str("lost the job")]) }
    fn __data86() -> Py { py_tuple(vec![py_str("i lost my job"), py_str("i've lost my job"), py_str("i got sacked"), py_str("i was sacked"), py_str("i got fired"), py_str("i was made redundant"), py_str("i've been made redundant"), py_str("benefits"), py_str("universal credit"), py_str("sign on"), py_str("mortgage"), py_str("rent"), py_str("interview"), py_str("notice pay"), py_str("redundancy pay"), py_str("tribunal"), py_str("sack him"), py_str("sack her"), py_str("fire him"), py_str("fire her"), py_str("dismiss"), py_str("appraisal")]) }
    fn __data87() -> Py { py_tuple(vec![py_str("resign"), py_str("resignation"), py_str("my notice"), py_str("notice in"), py_str("quitting"), py_str("quit"), py_str("leaving my job"), py_str("leaving the job"), py_str("leave my job"), py_str("leaving work for good"), py_str("im leaving"), py_str("i'm leaving"), py_str("hand in my notice"), py_str("working my notice"), py_str("leave a job"), py_str("leaving a job"), py_str("leaving email"), py_str("leaving message"), py_str("farewell email"), py_str("farewell message"), py_str("goodbye email"), py_str("goodbye message")]) }
    fn __data88() -> Py { py_tuple(vec![py_str("landlord"), py_str("landlady"), py_str("flat"), py_str("tenancy"), py_str("rent"), py_str("lease"), py_str("house"), py_str("last job"), py_str("previous job"), py_str("old job"), py_str("current job"), py_str("interview"), py_str("cv"), py_str("why i left"), py_str("why i'm leaving"), py_str("why i quit"), py_str("why did i leave"), py_str("reference"), py_str("references"), py_str("left out"), py_str("left over"), py_str("left behind"), py_str("left the"), py_str("left my keys"), py_str("left my phone"), py_str("left my wallet"), py_str("leaving early"), py_str("leave early"), py_str("half day"), py_str("dentist"), py_str("pick up"), py_str("collect"), py_str("appointment"), py_str("sick"), py_str("early on"), py_str("hour early"), py_str("time off"), py_str("day off"), py_str("partner"), py_str("husband"), py_str("wife"), py_str("boyfriend"), py_str("girlfriend"), py_str("fiance"), py_str("fiancee"), py_str("friend"), py_str("brother"), py_str("sister"), py_str("mum"), py_str("dad"), py_str("son"), py_str("daughter"), py_str("colleague"), py_str("should i quit"), py_str("should i resign"), py_str("if i should quit"), py_str("if i should leave"), py_str("whether to quit"), py_str("whether i should quit"), py_str("whether i should leave"), py_str("know if i should"), py_str("should i leave"), py_str("should i stay"), py_str("quit or stay"), py_str("worth quitting"), py_str("worth resigning"), py_str("worth leaving"), py_str("he wants to"), py_str("she wants to"), py_str("he is going to"), py_str("she is going to"), py_str("interview"), py_str("interviews"), py_str("interviewer"), py_str("why i left"), py_str("why did i leave"), py_str("ask why i left"), py_str("cv"), py_str("application"), py_str("applying"), py_str("no notice"), py_str("without notice"), py_str("short notice"), py_str("at short notice"), py_str("on notice"), py_str("rota"), py_str("rotas"), py_str("shift"), py_str("shifts"), py_str("hours changed"), py_str("changed my hours"), py_str("holiday"), py_str("holidays"), py_str("annual leave"), py_str("entitlement"), py_str("entitled"), py_str("statutory"), py_str("pro rata"), py_str("pro-rata"), py_str("accrued"), py_str("accrue"), py_str("how much holiday"), py_str("how many days")]) }
    fn __data89() -> Py { py_tuple(vec![py_str("voicemail"), py_str("answerphone"), py_str("answer machine"), py_str("answering machine"), py_str("voice mail"), py_str("leave a message on"), py_str("message on his"), py_str("message on her"), py_str("leave a voicemail"), py_str("him a message"), py_str("her a message"), py_str("them a message"), py_str("to leave a message"), py_str("who should i leave"), py_str("can i leave a message")]) }
    fn __data90() -> Py { py_tuple(vec![py_str("cancels"), py_str("cancelling"), py_str("canceling"), py_str("canceled"), py_str("cancelled"), py_str("flakes"), py_str("flaking"), py_str("bails"), py_str("bail"), py_str("bailing"), py_str("bailed"), py_str("stands me up"), py_str("stood me up")]) }
    fn __data91() -> Py { py_tuple(vec![py_str("always"), py_str("keeps"), py_str("last minute"), py_str("flaky"), py_str("again"), py_str("all the time"), py_str("every time"), py_str("three times"), py_str("twice"), py_str("four times"), py_str("on me"), py_str("plans"), py_str("stood me up")]) }
    fn __data92() -> Py { py_tuple(vec![py_str("friend"), py_str("mate"), py_str("she"), py_str("he"), py_str("they"), py_str("her"), py_str("him"), py_str("plans"), py_str("dinner"), py_str("pub"), py_str("lunch"), py_str("drink"), py_str("meet"), py_str("date"), py_str("birthday"), py_str("night out"), py_str("manager"), py_str("boss"), py_str("colleague"), py_str("workmate"), py_str("supervisor"), py_str("meeting"), py_str("meetings"), py_str("one to one"), py_str("one to ones"), py_str("1:1"), py_str("catch up"), py_str("catch-up"), py_str("our")]) }
    fn __data93() -> Py { py_tuple(vec![py_str("shift"), py_str("shifts"), py_str("rota"), py_str("day off"), py_str("days off"), py_str("annual leave"), py_str("employer"), py_str("overtime"), py_str("called in"), py_str("subscription"), py_str("subscriptions"), py_str("charged"), py_str("bill"), py_str("bills"), py_str("direct debit"), py_str("refund"), py_str("contract"), py_str("membership"), py_str("gym"), py_str("payment"), py_str("order"), py_str("booking"), py_str("flight"), py_str("on me again"), py_str("on me twice"), py_str("on me three times"), py_str("on me every time")]) }
    fn __data94() -> Py { py_tuple(vec![py_str("upset"), py_str("struggling"), py_str("hard time"), py_str("rough time"), py_str("going through"), py_str("devastated"), py_str("not coping"), py_str("whos upset"), py_str("is upset"), py_str("so upset"), py_str("really upset"), py_str("upset about"), py_str("upset over"), py_str("been dumped"), py_str("is struggling"), py_str("having a hard time"), py_str("having a rough time"), py_str("mental health")]) }
    fn __data95() -> Py { py_tuple(vec![py_str("my friend"), py_str("my mate"), py_str("my friend is"), py_str("my friend whos"), py_str("my friend has"), py_str("my friend got"), py_str("my friends"), py_str("my mate is"), py_str("my mate whos"), py_str("my mate has"), py_str("my mate got"), py_str("a friend of mine"), py_str("to my friend"), py_str("to my mate"), py_str("to her"), py_str("to him"), py_str("to them"), py_str("for my friend"), py_str("for my mate"), py_str("someone i know"), py_str("someone whos"), py_str("someone who is")]) }
    fn __data96() -> Py { py_tuple(vec![py_str("died"), py_str("dies"), py_str("dying"), py_str("death"), py_str("dead"), py_str("passed away"), py_str("passed on"), py_str("funeral"), py_str("grief"), py_str("grieving"), py_str("bereaved"), py_str("bereavement"), py_str("miscarriage"), py_str("stillborn"), py_str("lost her mum"), py_str("lost his mum"), py_str("lost her dad"), py_str("lost his dad"), py_str("lost their mum"), py_str("lost their dad"), py_str("lost my mum"), py_str("lost my dad"), py_str("card"), py_str("cards"), py_str("doctor"), py_str("gp"), py_str("nurse"), py_str("boss"), py_str("manager"), py_str("employer"), py_str("interview"), py_str("interviews"), py_str("interviewer"), py_str("interviewers"), py_str("recruiter"), py_str("hiring manager"), py_str("job application"), py_str("cv"), py_str("resume"), py_str("with me"), py_str("at me"), py_str("off with me"), py_str("annoyed"), py_str("ignoring me"), py_str("ignores me"), py_str("not talking to me"), py_str("isnt talking to me"), py_str("insulted me"), py_str("insulted"), py_str("rude to me"), py_str("shouted at me"), py_str("laughed at me"), py_str("picked on me"), py_str("bullied me"), py_str("mean to me"), py_str("offended me"), py_str("upset me"), py_str("horrible to me"), py_str("hurt me"), py_str("seems upset"), py_str("seems off"), py_str("seems distant"), py_str("looks upset"), py_str("might be upset"), py_str("maybe upset"), py_str("could be upset"), py_str("acts off"), py_str("acting off"), py_str("seems down"), py_str("baby"), py_str("babies"), py_str("toddler"), py_str("newborn"), py_str("infant"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("depressed"), py_str("depression")]) }
    fn __data97() -> Py { py_tuple(vec![py_str("how do i comfort"), py_str("how do i console"), py_str("how do i cheer"), py_str("console"), py_str("comfort"), py_str("comforting"), py_str("cheer up"), py_str("cheering up"), py_str("be there for"), py_str("what do i say"), py_str("what should i say"), py_str("what can i say"), py_str("what to say"), py_str("shares bad news"), py_str("share bad news"), py_str("shared bad news"), py_str("shares some bad news"), py_str("shares news"), py_str("tells me bad news"), py_str("told me bad news"), py_str("tells me something awful"), py_str("told me something awful"), py_str("with bad news"), py_str("bad news about"), py_str("hard news"), py_str("when someone has bad news"), py_str("when someone had bad news"), py_str("someone has bad news"), py_str("respond to bad news"), py_str("reply to bad news"), py_str("react to bad news")]) }
    fn __data98() -> Py { py_tuple(vec![py_str("someone"), py_str("somebody"), py_str("a friend"), py_str("a mate"), py_str("my friend"), py_str("my mate"), py_str("my best friend"), py_str("my sister"), py_str("my brother"), py_str("my mum"), py_str("my mom"), py_str("my dad"), py_str("my son"), py_str("my daughter"), py_str("my wife"), py_str("my husband"), py_str("my partner"), py_str("my gran"), py_str("a loved one"), py_str("a family member"), py_str("a colleague"), py_str("people"), py_str("a person"), py_str("anyone"), py_str("bad news"), py_str("sad news"), py_str("hard news")]) }
    fn __data99() -> Py { py_tuple(vec![py_str("upset"), py_str("struggling"), py_str("hard time"), py_str("rough time"), py_str("going through"), py_str("devastated"), py_str("not coping"), py_str("whos upset"), py_str("is upset"), py_str("so upset"), py_str("really upset"), py_str("upset about"), py_str("upset over"), py_str("been dumped"), py_str("having a hard time"), py_str("having a rough time"), py_str("mental health"), py_str("died"), py_str("dies"), py_str("dying"), py_str("death"), py_str("dead"), py_str("passed away"), py_str("funeral"), py_str("grief"), py_str("grieving"), py_str("bereaved"), py_str("card"), py_str("cards"), py_str("doctor"), py_str("gp"), py_str("nurse"), py_str("boss"), py_str("manager"), py_str("baby"), py_str("babies"), py_str("toddler"), py_str("newborn"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("myself"), py_str("my own"), py_str("insulted me"), py_str("insulted"), py_str("hurt me"), py_str("rude to me"), py_str("shouted at me"), py_str("laughed at me"), py_str("picked on me"), py_str("bullied me"), py_str("mean to me"), py_str("offended me"), py_str("upset me"), py_str("horrible to me"), py_str("with me"), py_str("at me"), py_str("pregnant"), py_str("pregnancy"), py_str("expecting"), py_str("engaged"), py_str("engagement"), py_str("promotion"), py_str("promoted"), py_str("got the job"), py_str("new job"), py_str("passed"), py_str("graduated"), py_str("won")]) }
    fn __data100() -> Py { py_tuple(vec![py_str("write"), py_str("writing"), py_str("help"), py_str("how"), py_str("draft"), py_str("start"), py_str("opening"), py_str("open"), py_str("example"), py_str("examples"), py_str("template"), py_str("length"), py_str("long"), py_str("words"), py_str("minutes"), py_str("what do i say"), py_str("structure"), py_str("include"), py_str("add"), py_str("read"), py_str("reading"), py_str("say")]) }
    fn __data101() -> Py { py_tuple(vec![py_str("toast"), py_str("wedding"), py_str("best man"), py_str("maid of honour"), py_str("birthday"), py_str("anniversary"), py_str("retirement"), py_str("thank you"), py_str("card"), py_str("cards"), py_str("vows"), py_str("what time"), py_str("what day"), py_str("tuesday"), py_str("wednesday"), py_str("thursday"), py_str("friday"), py_str("saturday"), py_str("sunday"), py_str("monday"), py_str("parking"), py_str("flowers"), py_str("coffin"), py_str("casket")]) }
    fn __data102() -> Py { py_tuple(vec![py_str("write"), py_str("writing"), py_str("help"), py_str("how"), py_str("draft"), py_str("template"), py_str("examples"), py_str("length"), py_str("long"), py_str("words"), py_str("minutes"), py_str("structure"), py_str("start"), py_str("opening"), py_str("open"), py_str("what do i say"), py_str("say"), py_str("include"), py_str("nervous"), py_str("scared")]) }
    fn __data103() -> Py { py_tuple(vec![py_str("wedding"), py_str("sisters"), py_str("best man"), py_str("maid of honour"), py_str("father of the bride"), py_str("few words"), py_str("write"), py_str("birthday"), py_str("70th"), py_str("60th"), py_str("50th"), py_str("retirement"), py_str("leaving do"), py_str("anniversary")]) }
    fn __data104() -> Py { py_tuple(vec![py_str("thank you"), py_str("thankyou"), py_str("card"), py_str("cards"), py_str("invitation"), py_str("invitations"), py_str("save the date"), py_str("place card"), py_str("place cards"), py_str("order of service"), py_str("vows"), py_str("vow"), py_str("reading"), py_str("readings"), py_str("venue"), py_str("guest list"), py_str("seating"), py_str("table plan")]) }
    fn __data105() -> Py { py_tuple(vec![py_str("letter"), py_str("note"), py_str("card"), py_str("message"), py_str("text"), py_str("email"), py_str("say"), py_str("saying"), py_str("write"), py_str("wording"), py_str("words"), py_str("meal"), py_str("dinner"), py_str("lunch"), py_str("party"), py_str("weekend"), py_str("staying"), py_str("stayed"), py_str("host"), py_str("hosted"), py_str("put me up"), py_str("lift"), py_str("present"), py_str("gift"), py_str("flowers"), py_str("helped me"), py_str("help"), py_str("wedding")]) }
    fn __data106() -> Py { py_tuple(vec![py_str("how do i"), py_str("how can i"), py_str("how should i"), py_str("how do you"), py_str("what should i say"), py_str("what do i say"), py_str("what to say"), py_str("what do i write"), py_str("what should i write"), py_str("write"), py_str("writing"), py_str("wording"), py_str("word"), py_str("words"), py_str("letter"), py_str("note"), py_str("card"), py_str("message"), py_str("text"), py_str("email"), py_str("thank you for the"), py_str("thanks for the"), py_str("thank them")]) }
    fn __data107() -> Py { py_tuple(vec![py_str("italy"), py_str("italian"), py_str("france"), py_str("french"), py_str("germany"), py_str("german"), py_str("portugal"), py_str("greece"), py_str("spain by"), py_str("netherlands"), py_str("belgium"), py_str("train travel"), py_str("by train"), py_str("take the train"), py_str("get around"), py_str("travel around"), py_str("travelling around"), py_str("rail"), py_str("railway"), py_str("interrail"), py_str("railcard"), py_str("tickets"), py_str("book a train"), py_str("cheap train"), py_str("trainline"), py_str("high speed"), py_str("gym"), py_str("gyms"), py_str("workout"), py_str("work out"), py_str("weights"), py_str("i train"), py_str("exercising")]) }
    fn __data108() -> Py { py_tuple(vec![py_str("food"), py_str("pizza"), py_str("weather"), py_str("hotel"), py_str("hotels"), py_str("flight"), py_str("flights"), py_str("visa"), py_str("passport"), py_str("work"), py_str("job"), py_str("move to"), py_str("living in"), py_str("rent"), py_str("house")]) }
    fn __data109() -> Py { py_tuple(vec![py_str("write"), py_str("writing"), py_str("how do i"), py_str("how can i"), py_str("complain"), py_str("contact"), py_str("email"), py_str("what do i say"), py_str("who is"), py_str("who's"), py_str("should i"), py_str("get in touch"), py_str("reach")]) }
    fn __data29() -> Py { py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("card-writing")); __args.push(py_tuple(vec![op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("wishes"), py_str("lines"), py_str("write"), py_str("writing")]), &g_SAY()), py_tuple(vec![py_str("leaving"), py_str("leaving do"), py_str("last day"), py_str("goodbye"), py_str("farewell"), py_str("moving on"), py_str("emigrating"), py_str("retiring"), py_str("retirement")])])); __args.push(py_str("Keep it to two lines and one real detail: \"Working with you made the Monday mornings survivable, and I still owe you for the time you covered for me.\" Say what you will actually miss, then a plain good luck. If you want it funny, keep the joke to something the two of you share and never about their job or their leaving, and sign your full name for anyone who is not in your team.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&op_add(&g_SAID(), &g_ON_THE_PHONE()), &__data30())), ("ask".to_string(), op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("wishes"), py_str("write"), py_str("writing"), py_str("lines"), py_str("what do i write"), py_str("what to write"), py_str("what should i write")]), &g_SAY()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("card-birthday")); __args.push(py_tuple(vec![op_add(&__data31(), &g_SAY()), py_tuple(vec![py_str("birthday"), py_str("bday"), py_str("30th"), py_str("40th"), py_str("50th"), py_str("60th"), py_str("70th"), py_str("18th"), py_str("21st"), py_str("65th")])])); __args.push(py_func("_card_birthday", |args: &[Py]| _card_birthday(args))); __kws.extend(vec![("avoid".to_string(), op_add(&g_SAID(), &__data32())), ("bias".to_string(), Py::Int(1)), ("shape".to_string(), py_tuple(vec![py_func("_names_an_age", |args: &[Py]| _names_an_age(args))])), ("ask".to_string(), op_add(&__data33(), &g_SAY()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("card-congrats")); __args.push(py_tuple(vec![op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("wishes"), py_str("congratulations"), py_str("congrats"), py_str("write"), py_str("writing")]), &g_SAY()), py_tuple(vec![py_str("new job"), py_str("new role"), py_str("promotion"), py_str("graduation"), py_str("graduated"), py_str("passed"), py_str("results"), py_str("driving test"), py_str("first day")])])); __args.push(py_str("Name the thing they did and one thing you noticed about them doing it: \"All those late nights paid off - well done, Dr Chen.\" Skip the advice and the joke, and if the card is from a group, sign the group rather than one person's name.")); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&g_SAID(), &g_MOURNING()), &__data34())), ("bias".to_string(), Py::Int(1)), ("ask".to_string(), op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("wishes"), py_str("congratulations"), py_str("congrats"), py_str("write"), py_str("writing"), py_str("what do i write"), py_str("what to write")]), &g_SAY()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("card-new-home")); __args.push(py_tuple(vec![op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("wishes"), py_str("congratulations"), py_str("congrats"), py_str("write"), py_str("writing")]), &g_SAY()), py_tuple(vec![py_str("new home"), py_str("new house"), py_str("new flat"), py_str("moved in"), py_str("moving in"), py_str("first house")])])); __args.push(py_str("Wish them well in the place, not the paperwork: \"Congratulations - may the boiler behave and the neighbours be quiet.\" A small thing for the house instead of a bottle of wine is a good present, and if you are visiting, ask before you bring anything big.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_SAID(), &py_tuple(vec![py_str("removals"), py_str("removal"), py_str("van"), py_str("packing"), py_str("pack"), py_str("unpacking"), py_str("deposit"), py_str("mortgage"), py_str("house price"), py_str("buying a house"), py_str("moved out"), py_str("rent")]))), ("bias".to_string(), Py::Int(1)), ("ask".to_string(), op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("wishes"), py_str("congratulations"), py_str("congrats"), py_str("write"), py_str("writing"), py_str("what do i write"), py_str("what to write")]), &g_SAY()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("card-something")); __args.push(py_tuple(vec![op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("note"), py_str("message"), py_str("messages"), py_str("write"), py_str("writing")]), &g_SAY()), py_tuple(vec![py_str("what do i write"), py_str("what to write"), py_str("what should i write"), py_str("what do i say"), py_str("what to say"), py_str("blank"), py_str("stuck"), py_str("no idea")])])); __args.push(py_func("_card_plain", |args: &[Py]| _card_plain(args))); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&g_MOURNING(), &__data35())), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let reading: Py = __kwp(args, 0, "reading").unwrap_or(Py::None); Py::Bool(!py_truthy(&_names_an_age(&[{ let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]) } }]))) })), ("shape".to_string(), py_tuple(vec![py_func("_asks_for_the_words", |args: &[Py]| _asks_for_the_words(args))])), ("ask".to_string(), py_tuple(vec![py_str("card"), py_str("cards"), py_str("note"), py_str("notes"), py_str("message"), py_str("messages"), py_str("write"), py_str("writing"), py_str("blank")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("card-anniversary")); __args.push(py_tuple(vec![op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("wishes"), py_str("write"), py_str("writing")]), &g_SAY()), py_tuple(vec![py_str("anniversary"), py_str("years together"), py_str("married for"), py_str("wedding day")])])); __args.push(py_str("Name the year, then one thing you are glad about: \"Ten years, and you still make me laugh at the worst jokes.\" A line about a specific time beats anything about forever; if it is someone else's anniversary, keep it to the pair and skip the advice.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_SAID(), &py_tuple(vec![py_str("buy"), py_str("buying"), py_str("present"), py_str("presents"), py_str("gift"), py_str("gifts"), py_str("restaurant"), py_str("book a table"), py_str("meal out"), py_str("hotel")]))), ("bias".to_string(), Py::Int(1)), ("ask".to_string(), op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("wishes"), py_str("write"), py_str("writing"), py_str("what do i write"), py_str("what to write")]), &g_SAY()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("card-valentines")); __args.push(py_tuple(vec![op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("wishes"), py_str("write"), py_str("writing"), py_str("love")]), &g_SAY()), py_tuple(vec![py_str("valentine"), py_str("valentines"), py_str("valentines day"), py_str("14 february"), py_str("romantic"), py_str("romance"), py_str("girlfriend"), py_str("boyfriend")])])); __args.push(py_str("One true thing you would say out loud, then stop: \"I like who I am when I am with you.\" Long lists and borrowed lines read as effort, not feeling, so keep it to two lines and sign it properly unless it is going to a secret valentine.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_SAID(), &py_tuple(vec![py_str("buy"), py_str("buying"), py_str("present"), py_str("presents"), py_str("gift"), py_str("gifts"), py_str("flowers"), py_str("restaurant"), py_str("book a table"), py_str("meal out"), py_str("hotel")]))), ("bias".to_string(), Py::Int(1)), ("ask".to_string(), op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("wishes"), py_str("write"), py_str("writing"), py_str("love"), py_str("what do i write"), py_str("what to write")]), &g_SAY()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("card-parents-day")); __args.push(py_tuple(vec![op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("wishes"), py_str("write"), py_str("writing")]), &g_SAY()), py_tuple(vec![py_str("mothers day"), py_str("mothering sunday"), py_str("fathers day"), py_str("mums day"), py_str("dads day")])])); __args.push(py_str("One thing she or he actually did for you, named plainly: \"Thanks for every school run in the rain.\" A memory does more than \"best mum ever\", and if you are not close, a warm two lines and both your names is enough - the present is a separate job.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_SAID(), &__data36())), ("bias".to_string(), Py::Int(1)), ("ask".to_string(), op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("wishes"), py_str("write"), py_str("writing"), py_str("what do i write"), py_str("what to write")]), &g_SAY()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("card-new-baby")); __args.push(py_tuple(vec![op_add(&__data37(), &g_SAY()), __data38()])); __args.push(py_str("Name the baby if you know the name, wish the parents some sleep, and skip the jokes about their life being over: \"Congratulations - she is beautiful. Wishing all three of you a slow, sleepy first month.\" If it is a second baby, say something about the older one too; nobody else will.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_SAID(), &__data39())), ("bias".to_string(), Py::Int(1)), ("ask".to_string(), op_add(&__data40(), &g_SAY()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("card-get-well")); __args.push(py_tuple(vec![op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("write"), py_str("writing"), py_str("wishes")]), &g_SAY()), py_tuple(vec![py_str("get well"), py_str("get better"), py_str("speedy recovery"), py_str("poorly"), py_str("operation"), py_str("surgery"), py_str("in hospital"), py_str("recovering"), py_str("recovery"), py_str("illness")]), py_tuple(vec![py_str("hospital"), py_str("in hospital"), py_str("ward"), py_str("operation"), py_str("surgery")])])); __args.push(py_str("Say the one thing you want for them and keep it light: \"Thinking of you - take the time you need and we will save you a seat.\" No advice, no story about your own operation, and do not say \"let me know if you need anything\": offer something specific instead, like a lift home or dinner on Thursday.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_SAID(), &__data41())), ("bias".to_string(), Py::Int(1)), ("ask".to_string(), op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("write"), py_str("writing"), py_str("wishes"), py_str("what do i write"), py_str("what to write")]), &g_SAY()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("card-wedding")); __args.push(py_tuple(vec![op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("write"), py_str("writing"), py_str("wishes")]), &g_SAY()), py_tuple(vec![py_str("wedding"), py_str("married"), py_str("marriage"), py_str("engagement"), py_str("engaged"), py_str("the big day")])])); __args.push(py_str("Two lines, one about them as a pair: \"So glad you two found each other - the way you laugh at the same things is the whole thing.\" Put both their names on the envelope, and if you are not going, say so warmly in one line and leave it there.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_SAID(), &py_tuple(vec![py_str("speech"), py_str("toast"), py_str("venue"), py_str("seating"), py_str("table plan"), py_str("guest list"), py_str("first dance"), py_str("my husband"), py_str("my wife"), py_str("get married"), py_str("propose")]))), ("bias".to_string(), Py::Int(1)), ("ask".to_string(), op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("write"), py_str("writing"), py_str("wishes"), py_str("what do i write"), py_str("what to write")]), &g_SAY()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("card-christening")); __args.push(py_tuple(vec![op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("write"), py_str("writing"), py_str("wishes")]), &g_SAY()), py_tuple(vec![py_str("christening"), py_str("baptism"), py_str("baptised"), py_str("naming ceremony"), py_str("godparent"), py_str("godparents")])])); __args.push(py_str("Name the baby and one wish: \"Welcome to the world, Ada - may you be as loved as you were today.\" Sign both your names, and if you gave a present, the card is not the place to describe it.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_SAID(), &py_tuple(vec![py_str("wear"), py_str("wearing"), py_str("dress"), py_str("outfit"), py_str("suit"), py_str("what should i wear"), py_str("christening gown"), py_str("present"), py_str("presents"), py_str("gift"), py_str("gifts")]))), ("bias".to_string(), Py::Int(1)), ("ask".to_string(), op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("write"), py_str("writing"), py_str("wishes"), py_str("what do i write"), py_str("what to write")]), &g_SAY()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("card-christmas")); __args.push(py_tuple(vec![op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("write"), py_str("writing"), py_str("wishes"), py_str("greetings")]), &g_SAY()), py_tuple(vec![py_str("christmas"), py_str("xmas"), py_str("festive"), py_str("new year"), py_str("season's")])])); __args.push(py_str("Two lines is plenty unless it is family: \"Merry Christmas - hope the year has been kind to you and the next one is better.\" Add a specific thing you remember from the year for anyone close (\"still laughing about the tent\"), and for a neighbour or a colleague a plain wish and both your names is the whole job.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&g_SAID(), &__data42())), ("ask".to_string(), op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("note"), py_str("write"), py_str("writing"), py_str("wishes"), py_str("greetings"), py_str("what do i write"), py_str("what to write")]), &g_SAY()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("landlord-broken-boiler")); __args.push(py_tuple(vec![py_str("landlord"), py_str("broken"), py_str("repair"), py_tuple(vec![py_str("boiler"), py_str("heating"), py_str("email"), py_str("message")])])); __args.push(pyrt::xrefs::xcall("knowledge.lookup", "the_fault", &[py_str("Keep it short, dated and specific, and ask for a date: \"Hi — I reported the {fault} on [date] and nothing has been done. Please let me know when an engineer can come; I'm home after 5pm on weekdays. Thanks, [name].\" Follow it up in writing if nothing happens in 48 hours. Anything unsafe, the heating, the hot water and any appliance that came with the flat are the landlord's job, so if they ignore you ask the council's environmental health team to step in, keep the messages, and keep paying the rent: withholding it usually puts you in the wrong."), py_str("Keep it short, dated and specific, and ask for a date. Say what is wrong, when it started and when you are in, and follow it up in writing if nothing happens in 48 hours. Anything unsafe, the heating, the hot water and any appliance that came with the flat are the landlord's job, so if they ignore you ask the council's environmental health team to step in, keep the messages, and keep paying the rent: withholding it usually puts you in the wrong.")])); __kws.extend(vec![("avoid".to_string(), __data43()), ("ask".to_string(), __data44())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("neighbour-dangerous-dog")); __args.push(py_tuple(vec![py_tuple(vec![py_str("neighbours"), py_str("neighbour"), py_str("neighbors"), py_str("neighbor"), py_str("next door"), py_str("next-door"), py_str("nextdoors"), py_str("upstairs"), py_str("downstairs"), py_str("flat above")]), py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("puppy")])])); __args.push(py_str("Keep your own dog out of its way while this is going on - walk a different way and do not try to introduce them through the fence. Write down each time it happens: the date, the time, what the dog did, and whether anyone was hurt. Report it to the council's dog warden with those details, since a dog out of control in a public place or in your garden is their job, and a dog that has injured a person or another animal is a police matter. If the owner is approachable, say it plainly once: \"your dog went for mine on Tuesday by the gate - can you put it on a lead there?\" If anyone is bitten, get medical help or a vet first and report it afterwards, and keep every message you send.")); __kws.extend(vec![("ask".to_string(), __data45()), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("i got bitten"), py_str("i was bitten"), py_str("i have been bitten"), py_str("bit someone"), py_str("my dog bit"), py_str("my puppy bit"), py_str("my dog is aggressive"), py_str("my dog attacks"), py_str("my dog snapped"), py_str("what should i do if a dog bites")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("neighbour-noise")); __args.push(py_tuple(vec![__data46(), op_add(&op_add(&__data47(), &g_NOISE_WORDS()), &py_tuple(vec![py_str("keeps me awake"), py_str("wakes me up"), py_str("all night"), py_str("all day")]))])); __args.push(py_str("Talk to them once, at a calm time, and say when it happened and what you need: \"this has gone on after 11pm most nights this week, and I'm up at six - can it stop by then?\" Name the time and the nights, not the person. If it carries on, keep a short log of dates and times and report it to the council's noise team, who can act on it without you having to confront anyone.")); __kws.extend(vec![("ask".to_string(), op_add(&__data48(), &g_NOISE_WORDS())), ("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&__data49(), &g_MY_DOG_NOISE()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ask-for-refund")); __args.push(py_tuple(vec![py_tuple(vec![py_str("complain"), py_str("complaint"), py_str("refund"), py_str("refunds"), py_str("money back"), py_str("complained about")]), py_str("product"), __data50()])); __args.push(py_str("State the order, the problem and what you want, in that order: \"I ordered a lamp on 3 May (order 12345). It arrived cracked. Please send a replacement or refund the £40.\" Give a deadline of 14 days and mention your rights only if they say no.")); __kws.extend(vec![("ask".to_string(), __data51()), ("avoid".to_string(), __data52())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("decline-a-meeting"), py_tuple(vec![py_tuple(vec![py_str("meeting"), py_str("meetings"), py_str("zoom"), py_str("call with"), py_str("stand-up"), py_str("standup")]), py_tuple(vec![py_str("decline"), py_str("declining"), py_str("cancel"), py_str("cancelling"), py_str("excuse"), py_str("politely"), py_str("get out of"), py_str("skip"), py_str("reschedule")])]), py_str("Be brief and offer a substitute: \"I can't make Tuesday — can we do Thursday, or shall I send a note with my points?\" A short reason and an alternative reads as organised rather than rude.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("apology-message")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sorry"), py_str("apologise"), py_str("apologize"), py_str("apologise"), py_str("apology")]), __data53()])); __args.push(py_str("Name what you did, say why it was wrong, and don't add a but: \"I said I'd call and I didn't, and that left you waiting. I'm sorry.\" Then say what you will do differently, in one sentence.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("sorry"), py_str("apologise"), py_str("apologize"), py_str("apology")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("accept-an-apology")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sorry"), py_str("apology"), py_str("apologise"), py_str("apologize"), py_str("apologies"), py_str("apologises"), py_str("apologizes"), py_str("apologising"), py_str("apologizing"), py_str("apologized"), py_str("apologised")]), __data54()])); __args.push(py_str("Take it plainly and say so: \"Thanks for saying that - it means a lot.\" Then, if something is still needed, name one thing in a sentence (\"a text next time you are running late would help\") and leave it there. You do not have to forgive on the spot; accepting the apology is enough for today.")); __kws.extend(vec![("ask".to_string(), __data55()), ("avoid".to_string(), __data56()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("apology-somebody-owes")); __args.push(py_tuple(vec![__data57(), __data58()])); __args.push(py_str("You cannot make someone sorry, so ask for the repair rather than the words: say what you needed and what you will do now - \"I needed you to say it mattered, and I am not going to keep bringing it up.\" Ask once, plainly, and let them decide; if it never comes, the useful question is what you want next, not how to get it out of them.")); __kws.extend(vec![("ask".to_string(), __data59()), ("bias".to_string(), Py::Int(3)), ("avoid".to_string(), __data60())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("email-to-teacher")); __args.push(py_tuple(vec![py_str("teacher"), py_str("childs"), py_str("school"), py_tuple(vec![py_str("email"), py_str("write"), py_str("message")])])); __args.push(py_str("Put the child's name and class in the subject, say what you have noticed and what you would like: \"Sam (5B) has been anxious about maths homework — could we talk for ten minutes this week?\" Keep it to a few lines and ask for a specific next step.")); __kws.extend(vec![("avoid".to_string(), __data61()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("note-to-school-absence")); __args.push(py_tuple(vec![__data62(), py_tuple(vec![py_str("report"), py_str("note"), py_str("letter"), py_str("absent"), py_str("absence"), py_str("absences"), py_str("written")])])); __args.push(py_str("Say the child's name and class, the dates they were off, and the reason in one line: \"Sam, 5B, off Thursday and Friday with a sickness bug - back on Monday.\" Ask them to mark it authorised, and offer a doctor's note or appointment card if they want one. Email the school office with the dates in the subject line; most schools want a call on the first morning as well as the note.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("ask".to_string(), py_tuple(vec![py_str("note"), py_str("letter"), py_str("email"), py_str("write"), py_str("written"), py_str("absent"), py_str("absence"), py_str("absences"), py_str("sick note"), py_str("report")])), ("avoid".to_string(), __data63())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("asking-for-time-off")); __args.push(py_tuple(vec![__data64(), __data65()])); __args.push(py_func("_time_off", |args: &[Py]| _time_off(args))); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("time off"), py_str("day off"), py_str("days off"), py_str("boss"), py_str("work"), py_str("email"), py_str("early"), py_str("half day"), py_str("off")])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), __data66()), ("guard".to_string(), py_func("_asks_for_the_time", |args: &[Py]| _asks_for_the_time(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("follow-up-email")); __args.push(py_tuple(vec![__data67(), __data68()])); __args.push(py_func("_follow_up", |args: &[Py]| _follow_up(args))); __kws.extend(vec![("avoid".to_string(), op_add(&__data69(), &g_NOT_A_JOB()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("raise-a-problem-with-friend")); __args.push(py_tuple(vec![py_str("friend"), py_str("upset"), py_str("hurt"), py_tuple(vec![py_str("problem"), py_str("bring"), py_str("tell")])])); __args.push(py_func("_raise_problem", |args: &[Py]| _raise_problem(args))); __kws.extend(vec![("avoid".to_string(), __data70()), ("guard".to_string(), py_func("_not_a_hurt_body", |args: &[Py]| _not_a_hurt_body(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tell-someone-you-cant-come")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cant come"), py_str("cant make it"), py_str("not coming"), py_str("cant be there"), py_str("wont be able to come"), py_str("cant go"), py_str("have to cancel"), py_str("had to cancel"), py_str("pull out"), py_str("cannot make it")]), __data71()])); __args.push(py_tuple(vec![py_str("Say it early and plainly, give one reason and offer one thing instead: \"I'm so sorry, I can't make Saturday after all - I'm not well enough to travel. Can I take you both out once the dust settles?\" Short is kinder than detailed; a long excuse sounds like a choice, and telling them before they have paid for your plate is the whole kindness."), py_str("Say it as soon as you know, name the reason in one line and say what you can still do: \"I can't be there on the day, but I'd love to come and see you the week after.\" Do not leave it until the last week, and do not offer a string of excuses - one reason and one alternative reads as true, several read as invented.")])); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("cant come"), py_str("cant make it"), py_str("not coming"), py_str("cant be there"), py_str("wont be able to come"), py_str("cant go"), py_str("have to cancel"), py_str("had to cancel"), py_str("pull out"), py_str("cannot make it")])), ("avoid".to_string(), py_tuple(vec![py_str("money"), py_str("lend"), py_str("lent"), py_str("debt"), py_str("borrow"), py_str("job"), py_str("resign"), py_str("quit")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-to-tell-them")); __args.push(py_tuple(vec![__data72(), __data73()])); __args.push(py_func("_saying_it", |args: &[Py]| _saying_it(args))); __kws.extend(vec![("ask".to_string(), __data74()), ("avoid".to_string(), __data75())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tell-kids-santa")); __args.push(py_tuple(vec![py_tuple(vec![py_str("santa"), py_str("father christmas")]), __data76()])); __args.push(py_tuple(vec![py_str("Answer what they asked and keep the game: \"The presents come from us - the Santa story is a game grown-ups play to make Christmas feel magic, and now you are in on it.\" Ask them to keep it going for a younger cousin or brother, which turns the news into a job. Most children work it out at six or seven and want the ritual more than the belief."), py_str("Keep it warm and short: \"The Santa story is a game we all play - I put the presents out, and now you know the secret.\" Then give them a part to play for a younger child, because being trusted with the secret is what makes it feel like growing up rather than being lied to.")])); __kws.extend(vec![("ask".to_string(), __data77())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("break-hard-news-to-children")); __args.push(py_tuple(vec![py_tuple(vec![py_str("break the news"), py_str("bad news"), py_str("hard news"), py_str("difficult news"), py_str("something awful"), py_str("something terrible")]), py_tuple(vec![py_str("children"), py_str("kids"), py_str("child"), py_str("son"), py_str("daughter"), py_str("family"), py_str("toddler")])])); __args.push(py_str("Pick a calm moment with time after it, sit at their level and give the one fact in a short sentence, then say what stays the same: who picks them up, where they sleep, that you are not going anywhere. Then stop and let them ask; the real questions come later, in the bath or the car, and answering them plainly is the rest of the job.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("break the news"), py_str("bad news"), py_str("hard news"), py_str("difficult news"), py_str("something awful"), py_str("something terrible")])), ("avoid".to_string(), __data78()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("news-pregnancy")); __args.push(py_tuple(vec![op_add(&__data79(), &g_SAY()), py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("expecting"), py_str("expecting a baby"), py_str("baby on the way"), py_str("having a baby")])])); __args.push(py_str("Say congratulations first and mean it, then ask how she is: \"That is such good news - how are you feeling?\" Let her set the tone, because the first months are often sick and tired rather than shiny, and ask who already knows before you tell anyone else. Skip your own birth stories and do not ask whether it was planned.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_SAID(), &__data80())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("news-splitting-up")); __args.push(py_tuple(vec![op_add(&py_tuple(vec![py_str("say"), py_str("said"), py_str("saying"), py_str("text"), py_str("message"), py_str("write"), py_str("words"), py_str("reply"), py_str("respond"), py_str("react"), py_str("answer")]), &g_SAY()), __data81()])); __args.push(py_str("Do not take a side and do not ask what happened: \"I'm sorry - that's a lot. How are you holding up?\" Then listen, and offer one thing you can actually do, like a bed for the night or dinner on Thursday. Say nothing about their ex that you might have to take back later, and keep checking in over the next few months, when the visits stop.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_SAID(), &__data82())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("news-diagnosis")); __args.push(py_tuple(vec![op_add(&py_tuple(vec![py_str("say"), py_str("said"), py_str("saying"), py_str("text"), py_str("message"), py_str("write"), py_str("words"), py_str("reply"), py_str("respond"), py_str("answer"), py_str("react")]), &g_SAY()), __data83()])); __args.push(py_str("Say the plain thing and stop there: \"I'm sorry, that's a shock. How are you doing?\" Let them name the illness and the treatment in their own time, and take your steer from how much they want to talk about it. No cures, no \"my aunt had that and she's fine\", no advice - and offer one specific, repeatable help, like lifts to appointments, then do it.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_SAID(), &__data84())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("news-lost-job")); __args.push(py_tuple(vec![op_add(&py_tuple(vec![py_str("say"), py_str("said"), py_str("saying"), py_str("text"), py_str("message"), py_str("write"), py_str("words"), py_str("reply"), py_str("respond"), py_str("answer"), py_str("react")]), &g_SAY()), __data85()])); __args.push(py_str("Do not rush to fix it: \"That's rough - I'm sorry. How are you doing?\" Then offer the practical thing they will need in a week or two, like a reference, a look over their CV or a name to ring, rather than a plan on the first day. Skip anything that sounds like a lesson - everything happens for a reason, it worked out for the best - and do not ask what they did wrong.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_SAID(), &__data86())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("quit-a-job-message")); __args.push(py_tuple(vec![__data87(), py_tuple(vec![py_str("letter"), py_str("email"), py_str("message"), py_str("boss"), py_str("manager"), py_str("hand"), py_str("job"), py_str("tell"), py_str("write"), py_str("resign")])])); __args.push(py_str("Keep it to four lines: that you are resigning, the date your notice starts, your last working day, and thanks. Do it in a short meeting first, then send the letter the same day, and check your contract for the notice period.")); __kws.extend(vec![("guard".to_string(), py_func("_the_notice_is_being_written", |args: &[Py]| _the_notice_is_being_written(args))), ("avoid".to_string(), op_add(&__data88(), &g_ON_THE_PHONE()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("voicemail"), py_tuple(vec![__data89(), py_tuple(vec![py_str("leave"), py_str("leaving"), py_str("record"), py_str("say"), py_str("how"), py_str("good"), py_str("best"), py_str("clear"), py_str("tip"), py_str("tips"), py_str("should i"), py_str("when")])]), py_str("Keep it under 20 seconds: your name, your number said slowly and twice, why you rang, and when you can be reached. \"Hi, it's Sam on 07700 900123 - 07700 900123, about the boiler. Any time after six is fine.\" The number is the part people replay, so it is the part to slow down for.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("asking-for-a-favour")); __args.push(py_tuple(vec![py_str("favour"), py_str("favor"), py_str("big"), py_str("friend"), py_str("borrow")])); __args.push(py_str("Make it small, specific and easy to refuse: \"Could you water my plants on Tuesday and Friday? No problem if not.\" Saying what it involves and that they can say no makes people much more likely to say yes.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("favour"), py_str("favor")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("message-cancelling-dinner")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cancel"), py_str("cancelling"), py_str("canceling"), py_str("cancelled"), py_str("bail"), py_str("bailing")]), py_tuple(vec![py_str("dinner"), py_str("plans"), py_str("tonight"), py_str("message"), py_str("text"), py_str("politely"), py_str("friend")])])); __args.push(py_str("Say it early, be honest and offer a concrete alternative: \"So sorry, I'm going to have to cancel tonight — I'm wiped out. Can we do Thursday instead? My treat.\" Don't invent an illness; a plain reason is easier to forgive.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("always cancels"), py_str("keeps cancelling"), py_str("keeps canceling"), py_str("last minute"), py_str("cancels on me")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("friend-keeps-cancelling")); __args.push(py_tuple(vec![__data90(), __data91()])); __args.push(py_str("Say it once, about the pattern rather than the last time: \"We've cancelled three times now and honestly it's starting to feel like I'm an afterthought. Can we pick a night we'll both keep?\" Naming the pattern and asking for one firm date keeps it about the diary, not about her character.")); __kws.extend(vec![("ask".to_string(), __data92()), ("avoid".to_string(), __data93()), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let reading: Py = __kwp(args, 0, "reading").unwrap_or(Py::None); Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.repair", "owns_the_deed", &[reading.clone()]))) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("comfort-someone")); __args.push(py_tuple(vec![g_COMFORT_SAY(), op_add(&g_COMFORT_SITUATIONS(), &__data94()), __data95()])); __args.push(py_func("_comfort", |args: &[Py]| _comfort(args))); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), __data96()), ("ask".to_string(), g_COMFORT_SAY())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("comfort-plainly")); __args.push(py_tuple(vec![__data97(), __data98()])); __args.push(g_COMFORT_PLAIN()); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&g_COMFORT_SITUATIONS(), &__data99()), &g_A_BEHAVIOUR_TOWARD_THE_ASKER()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("affect-vs-effect"), py_tuple(vec![py_str("affect"), py_str("effect"), py_str("difference")]), py_str("Affect is usually the verb and effect the noun: the rain affected the game, the rain had an effect on the game. The exception worth knowing is that you can \"effect a change\", meaning to bring it about.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("fewer-vs-less"), py_tuple(vec![py_str("fewer"), py_str("less"), py_str("difference")]), py_str("\"Fewer\" for things you can count, \"less\" for things you can't: fewer people, fewer cups, but less water and less time. The one everyday exception is money and time in speech — \"less than five pounds\" is fine.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("how-to-pronounce"), py_tuple(vec![py_str("pronounce"), py_str("pronunciation"), py_str("word")]), py_str("Tell me the word and I'll spell out the sounds syllable by syllable in plain English, with the stressed part in capitals. If it is a name, I'll say it the way the person would.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("word-count-essay")); __args.push(py_tuple(vec![py_tuple(vec![py_str("count"), py_str("length"), py_str("word"), py_str("how long"), py_str("how many words"), py_str("word limit"), py_str("long")]), py_tuple(vec![py_str("essay"), py_str("essays"), py_str("dissertation"), py_str("dissertations"), py_str("coursework"), py_str("report")])])); __args.push(py_str("A short essay is usually 1,000–1,500 words, a university one 2,000–3,000, and coursework will state it. Aim for 10% under the limit; going over is the mistake markers mind.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("plan"), py_str("planning"), py_str("planned"), py_str("spend planning"), py_str("outline")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("write-a-eulogy")); __args.push(py_tuple(vec![py_tuple(vec![py_str("eulogy"), py_str("eulogies"), py_str("eulogy for my mum"), py_str("eulogy for my dad"), py_str("eulogy for my nan"), py_str("reading at the funeral"), py_str("speech at the funeral"), py_str("speak at the funeral")]), __data100()])); __args.push(py_func("_eulogy", |args: &[Py]| _eulogy(args))); __kws.extend(vec![("avoid".to_string(), __data101()), ("ask".to_string(), __data102()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("write-a-toast")); __args.push(py_tuple(vec![py_str("speech"), py_str("toast"), __data103()])); __args.push(py_str("Three minutes, about 450 words, is the length to aim for: a line thanking the people who got everyone there, one story with a real detail in it that shows what the person is like, then what they mean to you and a toast. Open with a fact everyone in the room recognises, then turn it: \"My sister has spent her life fixing animals, so it's a relief she's finally found one she can train.\" Keep that opening 30 seconds short, and put the sincere bit in the middle where people aren't braced for it.")); __kws.extend(vec![("avoid".to_string(), __data104())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thank-you-card-note")); __args.push(py_tuple(vec![py_tuple(vec![py_str("card"), py_str("note"), py_str("letter"), py_str("message"), py_str("text")]), py_tuple(vec![py_str("thank you"), py_str("thankyou"), py_str("thank-you"), py_str("thanks"), py_str("grateful"), py_str("appreciate"), py_str("appreciated")])])); __args.push(g_THANKS_ANY()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("birthday card"), py_str("christmas card"), py_str("wedding card"), py_str("christening card"), py_str("wedding message"), py_str("christening message"), py_str("card game"), py_str("cardiff")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thanks-for-hospitality")); __args.push(py_tuple(vec![py_tuple(vec![py_str("thank"), py_str("thanks"), py_str("thank you"), py_str("thankyou"), py_str("thank-you"), py_str("grateful")]), __data105()])); __args.push(py_func("thanks_note", |args: &[Py]| thanks_note(args))); __kws.extend(vec![("ask".to_string(), __data106()), ("avoid".to_string(), py_tuple(vec![py_str("interview"), py_str("interviews"), py_str("interviewer"), py_str("interviewing"), py_str("job interview"), py_str("panel"), py_str("recruiter")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("learn-spanish-20-minutes"), py_tuple(vec![py_str("spanish"), py_tuple(vec![py_str("learn"), py_str("learning"), py_str("language"), py_str("months"), py_str("six"), py_str("minutes")])]), py_str("Spend the 20 minutes on speaking aloud and short listening, not on apps that only make you tap words: 10 minutes of a course with audio, 5 of vocabulary you will actually use, 5 of repeating phrases. Twenty minutes every day beats two hours on Sunday, and in six months you will manage the basics of a trip.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("spanish-train-station")); __args.push(py_tuple(vec![py_str("station"), py_str("train"), py_str("phrases"), py_tuple(vec![py_str("travel"), py_str("spanish")])])); __args.push(py_str("¿Dónde está la estación de tren? — \"Where is the train station?\" Useful next to it: ¿A qué hora sale el próximo tren? for \"what time is the next train?\".")); __kws.extend(vec![("avoid".to_string(), __data107())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("trains-in-italy")); __args.push(py_tuple(vec![py_tuple(vec![py_str("italy"), py_str("italian"), py_str("trenitalia"), py_str("italo"), py_str("rome"), py_str("florence"), py_str("venice"), py_str("milan"), py_str("naples")]), py_tuple(vec![py_str("train"), py_str("trains"), py_str("rail"), py_str("by train"), py_str("get around"), py_str("travel around"), py_str("tickets"), py_str("book"), py_str("high speed"), py_str("frecciarossa")])])); __args.push(py_str("Book on Trenitalia, the state operator, or Italo for the other fast line: both sell online and at the machines in the station, and a ticket bought a week or two ahead is much cheaper than one bought on the day. The fast Frecciarossa trains do Rome to Florence in about 1 hour 30, Florence to Venice in about 2 hours, and Rome to Naples in about 1 hour 10, so Milan, Florence and Rome makes a comfortable week. Regional trains are cheap and need no reservation but are slow, and a regional ticket bought at a machine has to be stamped in the small machine on the platform before you get on.")); __kws.extend(vec![("avoid".to_string(), __data108()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("spelling-tricky-word")); __args.push(py_tuple(vec![py_tuple(vec![py_str("spell"), py_str("spelling"), py_str("spelt"), py_str("spelled"), py_str("correct"), py_str("correctly"), py_str("written")]), g__SPELLING_WORDS()])); __args.push(py_func("_spelling", |args: &[Py]| _spelling(args))); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("spell"), py_str("spelling"), py_str("spelt"), py_str("spelled"), py_str("correct"), py_str("correctly"), py_str("written"), py_str("how do you"), py_str("which is"), py_str("is it")])), ("slips".to_string(), g__SPELLING_SLIPS())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("improve-spelling")); __args.push(py_tuple(vec![py_tuple(vec![py_str("spelling"), py_str("spell"), py_str("spelt"), py_str("spelled")]), py_tuple(vec![py_str("improve"), py_str("better"), py_str("learn"), py_str("remember"), py_str("struggle"), py_str("bad"), py_str("terrible"), py_str("awful"), py_str("hopeless"), py_str("rules"), py_str("good")])])); __args.push(py_str("Work from your own mistakes, not a list of hard words: take the twenty words you actually get wrong in texts and emails, and use look-say-cover-write-check on five a day - look at it, say it, cover it, write it, check it. Break long words into syllables as you say them (gov-ern-ment, sep-a-rate), and read your own writing out loud, because most spelling slips are words you can hear. Spellcheck is fine as a check, but read the suggestion before you accept it, or the same mistake survives.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("improve"), py_str("better"), py_str("learn"), py_str("remember"), py_str("struggle"), py_str("bad"), py_str("terrible"), py_str("awful"), py_str("hopeless"), py_str("how do i"), py_str("how can i"), py_str("help")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("spelling-pick-one")); __args.push(py_tuple(vec![py_tuple(vec![py_str("which"), py_str("correct"), py_str("or")]), g__SPELLING_SLIPS()])); __args.push(py_func("_spelling", |args: &[Py]| _spelling(args))); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("which"), py_str("correct"), py_str("is it"), py_str("or")])), ("slips".to_string(), g__SPELLING_SLIPS())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("letter-to-mp")); __args.push(py_tuple(vec![py_tuple(vec![py_str("letter to my mp"), py_str("write to my mp"), py_str("my mp"), py_str("to my mp"), py_str("contact my mp"), py_str("email my mp"), py_str("member of parliament"), py_str("mps"), py_str("councillor"), py_str("councillors"), py_str("mp's")]), __data109()])); __args.push(py_str("One page, one issue, one ask. Say who you are and where you live first, with the full postcode, because an MP can only act for their own constituents; then the problem in three sentences with the dates, then exactly what you want them to do - change a rule, chase a department, or write to a minister for you. Attach copies, never originals, and ask for a reply within four weeks. Address it to the House of Commons, London SW1A 0AA, or find the name and the office email on theyworkforyou.com. A short factual letter gets an answer from a caseworker; a long angry one gets a standard reply.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("council tax"), py_str("mp3"), py_str("music"), py_str("player"), py_str("download"), py_str("app"), py_str("video"), py_str("game"), py_str("film"), py_str("book"), py_str("series")])), ("ask".to_string(), py_tuple(vec![py_str("write"), py_str("letter"), py_str("how do i"), py_str("complain"), py_str("contact"), py_str("who is"), py_str("what do i say")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.ENTRIES"); }
    s_ENTRIES(__data29());
    fn __data110() -> Py { py_tuple(vec![py_str("class"), py_str("classes"), py_str("classroom"), py_str("classrooms"), py_str("lesson"), py_str("lessons"), py_str("lecture"), py_str("lectures"), py_str("school"), py_str("schools"), py_str("assembly"), py_str("teacher"), py_str("teachers"), py_str("tutor"), py_str("seminar"), py_str("in class"), py_str("at school"), py_str("during class"), py_str("exam"), py_str("exams"), py_str("playground"), py_str("meeting"), py_str("meetings"), py_str("board meeting"), py_str("team meeting"), py_str("one to one")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.TALKING_IS_NOT_A_CHAT"); }
    s_TALKING_IS_NOT_A_CHAT(__data110());
    fn __data111() -> Py { py_tuple(vec![py_str("how do i tell"), py_str("how do i say"), py_str("how do i ask"), py_str("how to tell"), py_str("how to say"), py_str("how to ask"), py_str("what do i say"), py_str("what should i say"), py_str("how can i tell"), py_str("how can i say"), py_str("how can i ask"), py_str("ways to say"), py_str("way to say"), py_str("word for"), py_str("how do you tell"), py_str("how do you say"), py_str("how would i say"), py_str("tell someone"), py_str("telling someone"), py_str("say to someone"), py_str("saying to someone"), py_str("tell a friend"), py_str("tell my friend"), py_str("tell people"), py_str("break it to"), py_str("bring it up"), py_str("polite way"), py_str("politely")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.ASKING"); }
    s_ASKING(__data111());
    fn __data112() -> Py { py_tuple(vec![py_str("cant afford"), py_str("afford it"), py_str("stand up for myself"), py_str("second date"), py_str("goodbye"), py_str("leaving do"), py_str("leaving present"), py_str("thank you"), py_str("condolence"), py_str("sympathy"), py_str("eulogy"), py_str("toast"), py_str("wedding speech"), py_str("reference"), py_str("resign"), py_str("notice"), py_str("complaint"), py_str("refund")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording._NOT_A_SIMPLE_NO"); }
    s__NOT_A_SIMPLE_NO(__data112());
    fn __data113() -> Py { py_tuple(vec![py_str("ask"), py_str("asking"), py_str("sound"), py_str("sounding"), py_str("weak"), py_str("stupid"), py_str("embarrassed"), py_str("ashamed"), py_str("awkward"), py_str("burden"), py_str("bothering"), py_str("admit"), py_str("need")]) }
    fn __data114() -> Py { py_tuple(vec![py_str("how do i ask for a favour"), py_str("favours"), py_str("favor"), py_str("reference"), py_str("landlord"), py_str("boss for"), py_str("pay rise"), py_str("raise"), py_str("extension"), py_str("deadline"), py_str("lend"), py_str("borrow"), py_str("money"), py_str("lift"), py_str("ride"), py_str("cv"), py_str("resume"), py_str("covering letter"), py_str("cover letter"), py_str("personal statement"), py_str("application")]) }
    fn __data115() -> Py { py_tuple(vec![py_str("flat"), py_str("bedroom"), py_str("kitchen"), py_str("cupboard"), py_str("wardrobe"), py_str("shed"), py_str("office"), py_str("desk"), py_str("storage"), py_str("box room"), py_str("car"), py_str("van"), py_str("parking"), py_str("garden"), py_str("garage"), py_str("paint"), py_str("painting"), py_str("painted"), py_str("repaint"), py_str("decorate"), py_str("decorating"), py_str("decoration"), py_str("wallpaper"), py_str("plaster"), py_str("plastering"), py_str("carpet"), py_str("flooring"), py_str("tiles"), py_str("curtains"), py_str("blinds"), py_str("furniture"), py_str("sofa"), py_str("shelves"), py_str("rug"), py_str("lamp"), py_str("lights"), py_str("tidy"), py_str("tidying"), py_str("clear out"), py_str("clearing"), py_str("damp"), py_str("sprain"), py_str("sprained"), py_str("strain"), py_str("fracture"), py_str("fractured"), py_str("broken"), py_str("bone"), py_str("bones"), py_str("wrist"), py_str("ankle"), py_str("knee"), py_str("elbow"), py_str("shoulder"), py_str("rib"), py_str("ribs"), py_str("x-ray"), py_str("xray"), py_str("swelling"), py_str("swollen"), py_str("injury"), py_str("injured"), py_str("numb")]) }
    fn __data116() -> Py { py_tuple(vec![py_str("ask"), py_str("asking"), py_str("tell"), py_str("telling"), py_str("say"), py_str("saying"), py_str("how"), py_str("neighbour"), py_str("neighbor"), py_str("flatmate"), py_str("roommate"), py_str("housemate"), py_str("someone")]) }
    fn __data117() -> Py { py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("barking"), py_str("party"), py_str("parties"), py_str("music"), py_str("builders"), py_str("drilling"), py_str("landlord"), py_str("council"), py_str("police"), py_str("baby"), py_str("toddler"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("son"), py_str("daughter"), py_str("boss"), py_str("manager"), py_str("quiet word"), py_str("appraisal"), py_str("hr"), py_str("who is making"), py_str("what is that noise"), py_str("banging"), py_str("alarm"), py_str("colleague"), py_str("colleagues"), py_str("coworker"), py_str("coworkers"), py_str("co-worker"), py_str("co-workers"), py_str("workmate"), py_str("workmates"), py_str("teammate"), py_str("teammates"), py_str("team mate"), py_str("team mates"), py_str("at work"), py_str("at my desk"), py_str("open plan"), py_str("in the office"), py_str("the office")]) }
    fn __data118() -> Py { py_tuple(vec![py_str("party"), py_str("parties"), py_str("a do"), py_str("gathering"), py_str("event"), py_str("wedding"), py_str("drinks"), py_str("dinner"), py_str("barbecue"), py_str("bbq"), py_str("celebration"), py_str("night out"), py_str("house party")]) }
    fn __data119() -> Py { py_tuple(vec![py_str("leave"), py_str("leaving"), py_str("left"), py_str("go home"), py_str("going home"), py_str("slip away"), py_str("slip out"), py_str("leave early"), py_str("get away"), py_str("escape"), py_str("politely"), py_str("without being rude"), py_str("say goodbye"), py_str("say goodbyes"), py_str("goodbye"), py_str("goodbyes"), py_str("say bye"), py_str("head off"), py_str("excuse myself"), py_str("excusing myself"), py_str("make my excuses"), py_str("made my excuses"), py_str("my excuses")]) }
    fn __data120() -> Py { py_tuple(vec![py_str("leave early"), py_str("leave a party"), py_str("politely leave"), py_str("slip away"), py_str("leave without"), py_str("get out of a party"), py_str("say goodbye"), py_str("goodbye"), py_str("goodbyes"), py_str("excuse myself"), py_str("excusing myself"), py_str("make my excuses"), py_str("made my excuses"), py_str("my excuses")]) }
    fn __data121() -> Py { py_tuple(vec![py_str("leaving do"), py_str("leaving present"), py_str("leaving job"), py_str("leaving my job"), py_str("resign"), py_str("notice"), py_str("school"), py_str("hotel"), py_str("airbnb"), py_str("car"), py_str("parking"), py_str("before it ends"), py_str("keys"), py_str("flat"), py_str("tenancy"), py_str("landlord"), py_str("on the phone"), py_str("phone call"), py_str("the phone"), py_str("call"), py_str("calls"), py_str("voicemail")]) }
    fn __data122() -> Py { py_tuple(vec![py_str("meeting"), py_str("presentation"), py_str("interview"), py_str("argument"), py_str("row"), py_str("relationship"), py_str("break up"), py_str("landlord"), py_str("contract"), py_str("phone contract"), py_str("paid"), py_str("bill"), py_str("stop calling"), py_str("stop texting"), py_str("stop messaging"), py_str("keeps calling"), py_str("block"), py_str("blocked"), py_str("blocking"), py_str("unwanted calls"), py_str("spam"), py_str("cold callers")]) }
    fn __data123() -> Py { py_tuple(vec![py_str("someone"), py_str("friend"), py_str("grieving"), py_str("bereaved"), py_str("loss"), py_str("lost"), py_str("died"), py_str("death"), py_str("dying"), py_str("crying"), py_str("upset"), py_str("hurt"), py_str("bad news"), py_str("how"), py_str("what")]) }
    fn __data124() -> Py { py_tuple(vec![py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("dog"), py_str("cat"), py_str("pet"), py_str("bereavement leave"), py_str("funeral cost"), py_str("funeral flowers"), py_str("eulogy"), py_str("obituary"), py_str("death certificate"), py_str("probate"), py_str("will"), py_str("inheritance"), py_str("what to wear")]) }
    fn __data125() -> Py { py_tuple(vec![py_str("apologise"), py_str("apologize"), py_str("apology"), py_str("sorry"), py_str("how"), py_str("say"), py_str("write"), py_str("respond"), py_str("reply"), py_str("answer"), py_str("bad"), py_str("unhappy"), py_str("angry"), py_str("upset")]) }
    fn __data126() -> Py { py_tuple(vec![py_str("refund"), py_str("guarantee"), py_str("warranty"), py_str("legal"), py_str("sue"), py_str("ombudsman"), py_str("trading standards"), py_str("tell a customer no"), py_str("tell customers no"), py_str("say no"), py_str("saying no"), py_str("turn down"), py_str("refuse"), py_str("decline"), py_str("price"), py_str("quote"), py_str("delay"), py_str("late"), py_str("waiting"), py_str("small claims"), py_str("compensation"), py_str("my landlord"), py_str("my boss"), py_str("my colleague"), py_str("my friend"), py_str("my partner"), py_str("my wife"), py_str("my husband"), py_str("my mum"), py_str("my dad")]) }
    let __aug127 = op_add(&g_ENTRIES(), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ask-for-help")); __args.push(py_tuple(vec![py_tuple(vec![py_str("help"), py_str("hand"), py_str("support"), py_str("a favour"), py_str("favour"), py_str("assistance")]), __data113()])); __args.push(py_str("Name the job, the bit you are stuck on, and the smallest thing that would unstick it, with a time on it: \"I'm stuck on the last two pages of this report - could you read them before Friday?\" A specific, finished ask is easy to say yes to, and it reads as competence rather than weakness; \"can you help me with this\" gives them nothing to hold on to. If it still feels like a lot, say why you are asking them: they were good at this, or they have done it before.")); __kws.extend(vec![("ask".to_string(), op_add(&g_ASKING(), &py_tuple(vec![py_str("sound weak"), py_str("sounding weak"), py_str("without sounding"), py_str("dont want to sound"), py_str("feel bad asking"), py_str("hate asking"), py_str("hate to ask"), py_str("hard to ask")]))), ("avoid".to_string(), __data114())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ask-for-space")); __args.push(py_tuple(vec![py_tuple(vec![py_str("space"), py_str("some room"), py_str("room to breathe"), py_str("room to think"), py_str("room to myself"), py_str("time alone"), py_str("time to myself"), py_str("time on my own"), py_str("breathing room"), py_str("a break")]), py_tuple(vec![py_str("need"), py_str("needs"), py_str("want"), py_str("tell"), py_str("say"), py_str("ask"), py_str("asking"), py_str("how"), py_str("someone"), py_str("i need")])])); __args.push(py_str("Say it as a need for a few days rather than a verdict on them, and put a date on it so it cannot sound like leaving: \"I've got a lot on and I need a couple of quiet days - I'll ring you Thursday.\" Do not explain it as their fault and do not apologise for it three times; a short clear ask with a return date is the kindest version, and it gives them something to wait for instead of something to worry about.")); __kws.extend(vec![("ask".to_string(), op_add(&g_ASKING(), &py_tuple(vec![py_str("i need space"), py_str("need some space"), py_str("need a bit of space"), py_str("time apart")]))), ("avoid".to_string(), __data115())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ask-for-quiet")); __args.push(py_tuple(vec![py_tuple(vec![py_str("quiet"), py_str("quieter"), py_str("quietly"), py_str("noise"), py_str("noisy"), py_str("loud"), py_str("turn it down"), py_str("keep it down"), py_str("lower the volume")]), __data116()])); __args.push(py_str("Ask once, in private, at a calm time, and make it about the thing and not the person: \"The telly carries through the wall after eleven - could you turn it down a notch?\" Give when it happens, how it lands and what would fix it. Do not knock at midnight while it is loud - the ask that lands is the one made in daylight, and the first ask should sound like a favour between neighbours, because it is.")); __kws.extend(vec![("ask".to_string(), op_add(&g_ASKING(), &py_tuple(vec![py_str("be quiet"), py_str("be quieter"), py_str("keep it down"), py_str("turn it down"), py_str("less noise")]))), ("avoid".to_string(), __data117())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("leave-a-party")); __args.push(py_tuple(vec![__data118(), __data119()])); __args.push(py_str("Thank the host, mean it, and go: \"It's been lovely - I'm going to head off, thank you so much.\" Say your goodbyes to the host and the one or two people you actually spent the evening with, not the whole room, and have your reason ready in one line (\"early start tomorrow\") because the second question is always why. Leaving at the natural lull is easier than the big goodbye, and nobody minds nearly as much as the leaving feels like it will.")); __kws.extend(vec![("ask".to_string(), op_add(&g_ASKING(), &__data120())), ("avoid".to_string(), __data121())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("end-a-conversation")); __args.push(py_tuple(vec![py_tuple(vec![py_str("conversation"), py_str("chat"), py_str("talking"), py_str("call"), py_str("phone call"), py_str("small talk")]), py_tuple(vec![py_str("end"), py_str("ending"), py_str("finish"), py_str("finishing"), py_str("stop"), py_str("get out"), py_str("escape"), py_str("leave"), py_str("close"), py_str("politely"), py_str("how")])])); __args.push(py_str("Close it with a reason and a warm line, then move: \"I'll let you go - it's been really good to catch up.\" Turning it round to them is the trick, because \"I'll let you get back to it\" ends the call without anyone being brushed off. In person, say one thing you want to come back to (\"I want to hear how the move went - let's get a coffee next week\"), then go; a promise to continue is what makes leaving feel friendly rather than rude.")); __kws.extend(vec![("ask".to_string(), op_add(&g_ASKING(), &py_tuple(vec![py_str("end a conversation"), py_str("end the conversation"), py_str("stop talking"), py_str("get out of a conversation"), py_str("politely end"), py_str("politely get away"), py_str("end a chat"), py_str("end the chat"), py_str("how do i end"), py_str("how to end"), py_str("how do i get out")]))), ("avoid".to_string(), op_add(&__data122(), &g_TALKING_IS_NOT_A_CHAT())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("console-someone")); __args.push(py_tuple(vec![py_tuple(vec![py_str("console"), py_str("comfort"), py_str("sympathy"), py_str("condolence"), py_str("condolences"), py_str("cheer up"), py_str("make them feel better"), py_str("say to someone")]), __data123()])); __args.push(py_str("Say the plain thing and then be quiet: \"I'm so sorry - I don't have words, but I'm here.\" Name the person who died if they have named them, say their name back, and offer one concrete thing instead of \"let me know if you need anything\": a lift, a meal on Thursday, sitting with them. Do not explain, compare it to your own loss, or look for the bright side; the comfort is in staying, and in being the one who is still there in three weeks.")); __kws.extend(vec![("ask".to_string(), op_add(&g_ASKING(), &py_tuple(vec![py_str("console someone"), py_str("comfort someone"), py_str("what do i say to"), py_str("what to say when someone"), py_str("cheer someone up")]))), ("avoid".to_string(), __data124())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("apologise-to-a-customer")); __args.push(py_tuple(vec![py_tuple(vec![py_str("customer"), py_str("client"), py_str("customer service"), py_str("complaint"), py_str("complaining"), py_str("review"), py_str("reviewer"), py_str("client")]), __data125()])); __args.push(py_str("Name what happened, own it once, say what you are doing and by when, and leave your process out of it: \"I'm sorry - we sent the wrong size twice, and that's on us. The right one goes out today on next-day delivery.\" One apology is enough; explaining the system reads as an excuse, and asking them to do the work reads as a second failure. If it is a public review, answer in two sentences and move the detail to email.")); __kws.extend(vec![("ask".to_string(), op_add(&g_ASKING(), &py_tuple(vec![py_str("apologise to a customer"), py_str("apologize to a customer"), py_str("sorry to a customer"), py_str("reply to a bad review"), py_str("respond to a complaint"), py_str("answer a complaint"), py_str("apologise to a client")]))), ("avoid".to_string(), __data126())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.ENTRIES"); }
    s_ENTRIES(__aug127);
    fn __data128() -> Py { py_tuple(vec![py_str("present i dont like"), py_str("present i don't like"), py_str("gift i dont like"), py_str("gift i don't like"), py_str("present i hate"), py_str("gift i hate"), py_str("already have it"), py_str("already got one"), py_str("wrong size"), py_str("ugly present"), py_str("ugly gift"), py_str("bad present"), py_str("bad gift"), py_str("didnt like the present"), py_str("didn't like the present"), py_str("didnt like the gift"), py_str("didn't like the gift"), py_str("someone gives me"), py_str("gives me a gift"), py_str("gave me a present"), py_str("gave me a gift"), py_str("thank you for a present"), py_str("thank you for the present"), py_str("thank you note"), py_str("thank you card"), py_str("thank you letter"), py_str("thank someone for a gift"), py_str("what do i say when someone gives me")]) }
    fn __data129() -> Py { py_tuple(vec![py_str("present"), py_str("gift"), py_str("presents"), py_str("gifts"), py_str("thank"), py_str("thanks"), py_str("say"), py_str("write"), py_str("reply"), py_str("respond"), py_str("polite"), py_str("politely"), py_str("card"), py_str("note"), py_str("letter"), py_str("how"), py_str("what")]) }
    fn __data130() -> Py { py_tuple(vec![py_str("say"), py_str("said"), py_str("saying"), py_str("write"), py_str("reply"), py_str("respond"), py_str("thank"), py_str("thanks"), py_str("what"), py_str("how"), py_str("words"), py_str("message"), py_str("text"), py_str("card"), py_str("note"), py_str("letter")]) }
    fn __data131() -> Py { py_tuple(vec![py_str("are you pregnant"), py_str("if i am pregnant"), py_str("if im pregnant"), py_str("if i'm pregnant"), py_str("asks if i am pregnant"), py_str("asked if i am pregnant"), py_str("asking if i am pregnant"), py_str("asks if im pregnant"), py_str("asked if im pregnant"), py_str("do you think i am pregnant"), py_str("when are you due"), py_str("when is the baby due"), py_str("when is your baby due"), py_str("any news"), py_str("is there a baby"), py_str("are you expecting"), py_str("asks if i am expecting"), py_str("asked if i am expecting"), py_str("asking about a baby"), py_str("asks about a baby")]) }
    fn __data132() -> Py { py_tuple(vec![py_str("say"), py_str("said"), py_str("saying"), py_str("reply"), py_str("respond"), py_str("answer"), py_str("how"), py_str("what"), py_str("words"), py_str("someone"), py_str("people"), py_str("asks"), py_str("asked"), py_str("asking"), py_str("relative"), py_str("relatives"), py_str("mum"), py_str("mother"), py_str("mother in law"), py_str("colleague"), py_str("colleagues"), py_str("aunt"), py_str("friend")]) }
    fn __data133() -> Py { py_tuple(vec![py_str("say"), py_str("said"), py_str("saying"), py_str("reply"), py_str("respond"), py_str("answer"), py_str("how"), py_str("what"), py_str("words"), py_str("someone"), py_str("people"), py_str("asks"), py_str("asked"), py_str("asking")]) }
    let __aug134 = op_add(&g_ENTRIES(), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thanks-for-a-gift")); __args.push(py_tuple(vec![__data128(), __data129()])); __args.push(py_str("Say the thanks and mean that part, in one line, and do not name the thing: \"Thank you - it was so kind of you to think of me.\" If you will use some of it, say how - \"the scarf is going straight on tomorrow\" - and if you will not, say nothing about it and thank them again later for the thought. Ask for the receipt only from someone close, and only when the thing was expensive: \"It's lovely but the size is wrong - could I swap it?\" Nobody is owed the truth about a present.")); __kws.extend(vec![("ask".to_string(), __data130())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("asked-if-pregnant")); __args.push(py_tuple(vec![__data131(), __data132()])); __args.push(py_str("One easy line closes it, and you do not owe anyone more than that: \"No - I've just had a big lunch.\" If it keeps coming, or it is someone who should know better, name it once and flatly: \"I'd rather not talk about that.\" You do not have to explain whether you are trying, whether it happened and did not last, or whether you have decided not to - and you can say exactly that: \"that's not something I talk about.\" Then change the subject to them, which is what most people want anyway.")); __kws.extend(vec![("ask".to_string(), __data133())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.ENTRIES"); }
    s_ENTRIES(__aug134);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.LEAVING_SPEECH"); }
    s_LEAVING_SPEECH(py_str("Three short parts, three minutes at most: say who you are and why you are the one talking, give one concrete thing you will miss about them - a habit, a phrase, a thing they always fixed - and end with the room raising a glass. Write it out in full sentences and then cut it in half: the best version is two clean stories and a line about what they were like to work with. Name two or three people who are in the room and thank them for something specific, and finish on a toast like 'to Priya' rather than a thank-you to everyone, which always sounds like a wedding."));
    fn __data135() -> Py { py_tuple(vec![py_str("leaving do"), py_str("leaving party"), py_str("leaving drinks"), py_str("leaving"), py_str("leaving us"), py_str("leaving work"), py_str("last day"), py_str("farewell"), py_str("goodbye"), py_str("send off"), py_str("retirement"), py_str("retiring"), py_str("colleague"), py_str("team"), py_str("boss"), py_str("work"), py_str("thank you"), py_str("thanks"), py_str("tribute")]) }
    fn __data136() -> Py { py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("sign"), py_str("resignation"), py_str("resign"), py_str("notice"), py_str("letter"), py_str("email"), py_str("wedding"), py_str("bride"), py_str("groom"), py_str("best man"), py_str("father of the bride"), py_str("funeral"), py_str("eulogy"), py_str("death"), py_str("died"), py_str("passed away"), py_str("birthday"), py_str("anniversary"), py_str("maid of honour"), py_str("speech to text"), py_str("speech delay"), py_str("speech therapy"), py_str("figure of speech")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("leaving-speech")); __args.push(py_tuple(vec![py_tuple(vec![py_str("speech"), py_str("speeches"), py_str("toast"), py_str("few words"), py_str("say a few words"), py_str("stand up and say"), py_str("say at a leaving"), py_str("say at the leaving"), py_str("say at my leaving"), py_str("say at a farewell"), py_str("say at the retirement"), py_str("words at a leaving")]), __data135()])); __args.push(g_LEAVING_SPEECH()); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("speech"), py_str("speeches"), py_str("toast"), py_str("say"), py_str("words")])), ("avoid".to_string(), __data136()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data137() -> Py { py_tuple(vec![py_str("two goes"), py_str("two cycles"), py_str("two washes"), py_str("two loads"), py_str("three goes"), py_str("three cycles"), py_str("three washes"), py_str("took two"), py_str("takes two"), py_str("went through twice"), py_str("put it through twice"), py_str("still wet"), py_str("still damp"), py_str("leaves everything damp"), py_str("leaves the clothes damp"), py_str("leaves clothes damp"), py_str("leaves them damp"), py_str("leave everything damp"), py_str("come out damp"), py_str("comes out damp"), py_str("come out wet"), py_str("comes out wet"), py_str("doesnt dry"), py_str("doesn't dry"), py_str("isnt drying"), py_str("isn't drying"), py_str("not drying"), py_str("never dries"), py_str("doesnt finish"), py_str("doesn't finish"), py_str("not finishing"), py_str("doesnt cook"), py_str("doesn't cook"), py_str("undercooked"), py_str("comes out raw"), py_str("not hot enough"), py_str("takes ages"), py_str("taking ages"), py_str("takes forever"), py_str("taking forever"), py_str("taking hours"), py_str("takes hours"), py_str("taking so long"), py_str("takes so long"), py_str("taking all day"), py_str("takes all day"), py_str("long to dry"), py_str("slow to dry"), py_str("drying slowly"), py_str("dries slowly"), py_str("hours to dry"), py_str("all night to dry")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.wording.A_JOB_LEFT_UNDONE"); }
    s_A_JOB_LEFT_UNDONE(__data137());
}
pub fn the_line_said(args: &[Py]) -> Py {
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
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The line as one lower-case string with spaces round it, or \"\" when there is no line.");
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    return if py_truthy(&said.clone()) { { let mut __s = String::new(); __s.push_str(" "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" "); py_string(__s) } } else { py_str("") };
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
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut phrases: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrases") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrases"))) };
    let mut phrase: Py = Py::None;
    let mut tokens: Py = Py::None;
    tokens = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrases.clone()) { let mut phrase: Py = __item.clone(); __out.push(if py_truthy(&Py::Bool(py_contains(&phrase.clone(), &py_str(" ")))) { Py::Bool(py_contains(&said.clone(), &phrase.clone())) } else { Py::Bool(py_contains(&tokens.clone(), &phrase.clone())) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn screens_are_a_childs(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the screens in the line belong to a child, so a parent's limit answer fits.");
    said = the_line_said(&[understood.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[])))) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ONLY_SAID_ABOUT_A_CHILD()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return _names_one_of(&[said.clone(), g__A_CHILD_NAMED()]);
    Py::None
}
pub fn screens_are_somebody_elses(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the phone in the line is not the writer's own.\n\n    A line that says \"my phone\" or \"im on\" is the writer's whatever else it names, which is what\n    keeps \"my girlfriend says im always on my phone\" off the card about her phone.\n    ");
    said = the_line_said(&[understood.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[])))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__MY_OWN_SCREENS(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&screens_are_a_childs(&[understood.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__THEIR_SCREENS(), "search", &[said.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__SOMEBODY_AT_HOME()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &op_add(&phrase.clone(), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _birthday_age(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut found: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The age the line names, as an int, or None.\n\n    A card for a child of seven answered with an example that said \"Happy 40th\": the phrasing was\n    a fixed string, so the number in it was the number in the person who wrote it. People give the\n    age as an ordinal (\"70th\", \"7th\"), or as a plain number after the word (\"turning 7\", \"turns 5\").\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    found = stdlib_call("re", "search", &[py_str("\\b(\\d{1,3})(?:st|nd|rd|th)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    found = stdlib_call("re", "search", &[py_str("(?:birthday|turning|turns|going on|hits?)\\D{0,12}(\\d{1,3})\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    found = stdlib_call("re", "search", &[py_str("\\b(\\d{2,3})\\b\\s*(?:next|this|in)\\s*(?:week|month|year|weekend|june|july|august|september|october|november|december|january|february|march|april|may|christmas|summer|spring|autumn|winter|easter)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    found = stdlib_call("re", "search", &[py_str("\\b(\\d{1,2})\\s*(?:year|yr)s?[ -]?old\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    age = call_builtin("int", &[mcall(&found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&{ let __cmp11: Py = age.clone(); if py_truthy(&op_le(&Py::Int(1), &__cmp11)) { op_le(&__cmp11, &Py::Int(120)) } else { Py::Bool(false) } }) { age.clone() } else { Py::None };
    Py::None
}
pub fn _ordinal(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut number: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "number") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: number"))) };
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![Py::Int(11), Py::Int(12), Py::Int(13)]), &op_modulo(&number.clone(), &Py::Int(100))))) {
    return op_modulo(&py_str("%dth"), &number.clone());
    }
    return op_modulo(&py_str("%d%s"), &py_tuple(vec![number.clone(), mcall(&py_dict(vec![(Py::Int(1), py_str("st")), (Py::Int(2), py_str("nd")), (Py::Int(3), py_str("rd"))]), "get", &[op_modulo(&number.clone(), &Py::Int(10)), py_str("th")])]));
    Py::None
}
pub fn _names_an_age(args: &[Py]) -> Py {
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
    let _ = py_str("The line gives the age the card is for, which is what picks its example.\n\n    One signal, the same weight as a key word, and the card still has to be in the line: a\n    birthday card row is the answer to \"what do i write in my nans card\", but only the age\n    says which card it is when the line never says \"birthday\".\n    ");
    return Py::Bool(!py_is(&_birthday_age(&[words.clone()]), &Py::None));
    Py::None
}
pub fn _card_birthday(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut example: Py = Py::None;
    let mut said: Py = Py::None;
    let mut which: Py = Py::None;
    let _ = py_str("The words for a birthday card, with the age the person gave rather than a made-up one.");
    reading = if py_truthy(&call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { reading.clone() } else { py_dict(vec![]) };
    said = { let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    age = _birthday_age(&[said.clone()]);
    which = if py_truthy(&age.clone()) { _ordinal(&[age.clone()]) } else { py_str("birthday") };
    if py_truthy(&mcall(&g__NO_FUSS(), "search", &[said.clone()])) {
    return op_modulo(&py_str("Keep it plain and quiet, because that is the card a person who hates a fuss actually wants: \"Happy %s - no fuss, I'm just glad you're in my life.\" One true thing about them, no joke and no speech, and give the present quietly rather than in front of everyone."), &which.clone());
    }
    if py_truthy(&{ let __b = age.clone(); if !py_truthy(&__b) { __b } else { op_lt(&age.clone(), &Py::Int(13)) } }) {
    example = op_modulo(&py_str("\"Happy %s - the year you finally beat me at cards.\" One true thing they did beats any joke, and for a small child the card is really for the parent: keep it short and kind."), &which.clone());
    } else {
    if py_truthy(&{ let __b = age.clone(); if !py_truthy(&__b) { __b } else { op_lt(&age.clone(), &Py::Int(18)) } }) {
    example = op_modulo(&py_str("\"Happy %s - still the best person to be stuck in a car with.\" One thing the two of you actually do together beats any joke about the number, and a teenager reads a short card: no advice, no speech, and the money is welcome."), &which.clone());
    } else {
    if py_truthy(&mcall(&g__A_COLLEAGUE(), "search", &[said.clone()])) {
    example = op_modulo(&py_str("\"Happy %s - hope you get spoiled.\" With someone you do not know well the warm plain line is the card itself: no in-joke, and put the money or gift honestly (\"Have a drink on me\")."), &which.clone());
    } else {
    if py_truthy(&mcall(&g__A_CLOSE_ONE(), "search", &[said.clone()])) {
    example = op_modulo(&py_str("\"Happy %s - I still tell people about the night you drove us home at 3am.\" They are close enough for the in-joke, so use the one only the two of you would get."), &which.clone());
    } else {
    example = op_modulo(&py_str("\"Happy %s - I still tell people about the night you drove us home at 3am.\" For a colleague or someone you do not know well, a warm plain line is enough (\"Happy birthday - hope you get spoiled\"), and put the money or gift honestly (\"Have a drink on me\")."), &which.clone());
    }
    }
    }
    }
    return op_add(&py_str("Name one thing you like about them and skip the round-number joke: "), &example.clone());
    Py::None
}
pub fn _card_plain(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The card for an occasion the line never names, and the quiet one when it asks for one.\n\n    \"what do i write in a card for my neighbour\" is the two-line note, and that is right for a\n    card with no occasion. The same line with a person who hates a fuss in it (\"a card for\n    someone who hates a big do\") is asking for the quiet card, and the whole paragraph about\n    naming one real thing and one wish is exactly what they said they did not want.\n    ");
    reading = if py_truthy(&call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { reading.clone() } else { py_dict(vec![]) };
    said = { let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&mcall(&g__NO_FUSS(), "search", &[said.clone()])) {
    return py_str("Keep it plain and quiet, because that is the card this person actually wants: \"Happy birthday - no fuss, I'm just glad you're in my life.\" One true thing about them, no joke and no speech, and give the present quietly rather than in front of everyone.");
    }
    return py_str("Name one real thing about them and one wish, then stop - two lines is a card: \"Thinking of you both - hope the year is a good one.\" If you do not know them well, warm and plain beats clever, and sign it with your full name.");
    Py::None
}
pub fn _asks_for_the_words(args: &[Py]) -> Py {
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
    let _ = py_str("A shape: the line asks what words to use, whatever modal carries the ask.");
    return call_builtin("bool", &[mcall(&g__ASKS_FOR_THE_WORDS(), "search", &[mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _raise_example(args: &[Py]) -> Py {
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
    let mut patterns: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "patterns") { v.clone() }
    else { Py::None };
    let mut pattern: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The example sentence for this line, from the subject it names - or None.");
    let __iter1 = py_iter_or_raise(&g__RAISE_EXAMPLES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    words = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_is(&patterns.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&patterns.clone(), &pattern.clone())) } }) {
    continue;
    }
    if py_truthy(&mcall(&pattern.clone(), "search", &[line.clone()])) {
    return words.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _not_a_hurt_body(args: &[Py]) -> Py {
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
    let _ = py_str("False for a line whose \"hurt\" is on a body, which the body tables answer better.");
    line = call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    return Py::Bool(py_is(&mcall(&g__SOMEONE_HURT_A_PART(), "search", &[line.clone()]), &Py::None));
    Py::None
}
pub fn _raise_problem(args: &[Py]) -> Py {
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
    let mut example: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("The same advice for every problem between friends, with an example that fits this one.");
    line = call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    example = { let __b = _raise_example(&[line.clone()]); if py_truthy(&__b) { __b } else { g__RAISE_PLAIN() } };
    return op_add(&op_add(&py_str("Say what happened, how it made you feel and what you want to change, once: "), &example.clone()), &py_str(". One topic at a time, no list of past grievances, and ask for their side."));
    Py::None
}
pub fn _saying_it(args: &[Py]) -> Py {
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
    let mut example: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("How to say one specific piece of news to the people it concerns.");
    line = call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g__TALKS_OVER(), "search", &[line.clone()])) {
    return py_str("Say it once, plainly, and not in the moment it is happening: \"Can I finish this one? You jump in and I lose the thread.\" Name what they do rather than what they are - one line, no list of past times - and give them the benefit of the doubt once, because most people who talk over you are excited or nervous rather than rude. Then keep a short version for the moment itself, a hand up and \"hang on, let me finish this bit\", since that is the only place the habit can change. If it carries on after you have said it plainly, say it plainly a second time and say how it lands; that is not nagging, it is the second half of the same sentence.");
    }
    if py_truthy(&mcall(&g__MOVING_OUT(), "search", &[line.clone()])) {
    return py_str("Give them the plan, not a request for permission: \"I've found a flat and I'm moving out on the 14th - I wanted you to hear it from me first.\" Then let them react without defending it, and come back to the practical bits - how often you will visit, what you need - once the first conversation has cooled.");
    }
    if py_truthy(&mcall(&g__MOVING_AWAY(), "search", &[line.clone()])) {
    return py_str("Tell the people you will miss before the ones who will hear it second hand, and give them a date: \"I'm moving to Bristol in March - I didn't want you to find out from someone else.\" Then say what you want to keep going, a call every couple of weeks or a visit in the summer, so the news comes with a plan rather than a goodbye.");
    }
    if py_truthy(&mcall(&g__BABY(), "search", &[line.clone()])) {
    return py_str("Tell them in person or on a call rather than in a message, and lead with the news: \"I'm pregnant - it's early, and I wanted you to know.\" Then say what you want from them now, help or time before anyone else is told; that one sentence saves the guessing.");
    }
    if py_truthy(&{ let __b = mcall(&g__RAISE_SMOKE(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__RAISE_HOUSE(), "search", &[line.clone()]) } }) {
    example = { let __b = _raise_example(&[line.clone(), g__RAISE_ASK_SUBJECTS()]); if py_truthy(&__b) { __b } else { py_str("\"the music was still going at half one - could we call it midnight next time?\"") } };
    return op_add(&op_add(&py_str("Ask for the one thing, name when it happens and say what you want instead, once and in private: "), &example.clone()), &py_str(" Not the whole list and not in the moment it is annoying you, and leave them room to say why it is hard; a habit that is asked for plainly usually bends a little, where a rule handed down does not."));
    }
    if py_truthy(&mcall(&g__MESS(), "search", &[line.clone()])) {
    return py_str("Ask for the one job rather than the whole mess, and say when: \"Could you do the washing up tonight? I did it the last four nights and it is stacking up.\" Name the thing, the reason and the deadline in one breath - a general complaint about the flat gets an apology and no change.");
    }
    if py_truthy(&{ let __b = mcall(&g__SHARED_HOUSE(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__MONEY(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__SHARE_MONEY(), "search", &[line.clone()]) } } } }) {
    return py_str("Ask once, with the amount and the date, and leave the history out of it: \"The broadband comes to 32 a month and your half is 16 - could you send it by Friday?\" A plain ask reads as fair; hinting over the washing up reads as a buildup.");
    }
    if py_truthy(&mcall(&g__PARENT_MONEY(), "search", &[line.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__MONEY(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__HOUSEHOLD_MONEY(), "search", &[line.clone()]) } }) {
    return py_str("Pick a time that is not the moment a bill lands, and put the numbers side by side rather than asking for a verdict: what comes in, what goes out, and what each of you wants to be able to afford. Open with a question - \"how do you feel about the way we split things?\" - and leave one decision for next time.");
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__SAID_IT_IS_NEWS(), "search", &[line.clone()])))) {
    return Py::None;
    }
    return py_str("Say it in one flat sentence, then stop: the news, the reason if there is one, and what you want from them. The wait for the perfect moment is usually longer and worse than the conversation itself.");
    Py::None
}
pub fn _thanks_has(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut clean: Py = Py::None;
    let mut typed: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line named one of these words or phrases.");
    typed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_set(__out) })() };
    clean = mcall(&{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[]);
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&word.clone(), &py_str(" ")))) {
    if py_truthy(&Py::Bool(py_contains(&clean.clone(), &word.clone()))) {
    return Py::Bool(true);
    }
    } else {
    if py_truthy(&Py::Bool(py_contains(&typed.clone(), &word.clone()))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn thanks_note(args: &[Py]) -> Py {
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
    let _ = py_str("A thank-you note about the thing this line names, in two or three lines.\n\n    Public because the language domain answers \"how do i write a thank you note\" with it too:\n    that entry used to carry one example, and the example was always the same afternoon with\n    some twins, so a note to a teacher or a nurse came back about a cake.\n    ");
    if py_truthy(&_thanks_has(&[understood.clone(), g_THANKS_FLOWERS()])) {
    return py_str("Say where the flowers are and what they have done for the room: \"Thank you for the flowers - they are on the kitchen table and the whole week looks better.\" Two or three lines, say when you will see them next, and stop.");
    }
    if py_truthy(&_thanks_has(&[understood.clone(), g_THANKS_GIFT()])) {
    return py_str("Name the thing and what you are doing with it, rather than thanking them for it: \"Thank you for the cafetiere - it is on my desk and I use it every morning.\" Two or three lines, say when you will see them next, and stop; a present does not need a page.");
    }
    if py_truthy(&_thanks_has(&[understood.clone(), g_THANKS_STAY()])) {
    return py_str("Pick the one thing they did that you noticed - the spare bed, the early start, the extra trip to the station - and say what it saved you: \"Thank you for having me for the weekend - the lift on Sunday saved me the whole morning.\" Then say when you will have them back, if you mean it.");
    }
    if py_truthy(&_thanks_has(&[understood.clone(), g_THANKS_MEAL()])) {
    return py_str("Name the thing they did and what it changed for you, then stop: \"Thank you for the meal on Saturday - I came home full and happy.\" One detail from the evening that only you would know beats a paragraph of adjectives, and anything you are actually going to do, say when.");
    }
    if py_truthy(&_thanks_has(&[understood.clone(), g_THANKS_LIFT()])) {
    return py_str("Name the trip and what it saved you: \"Thank you for the lift to the station on Friday - it saved me an hour and a soaking.\" Then one line about what it meant to you, and stop.");
    }
    if py_truthy(&_thanks_has(&[understood.clone(), g_THANKS_FAVOUR()])) {
    return py_str("Name the job they did and what it saved you, then stop: \"Thank you for looking after the cat while we were away - coming back to a happy animal was the whole holiday.\" One line about what it meant to you beats a paragraph, and say what you will do in return if you mean it.");
    }
    if py_truthy(&_thanks_has(&[understood.clone(), g_THANKS_PERSON()])) {
    return py_str("Say what they did and what it was worth to you: \"Thank you for the time you gave me over the summer - I still use the list you wrote.\" One sentence of detail, one of what it meant, then stop; people keep the short ones.");
    }
    return g_THANKS_ANY();
    Py::None
}
pub fn _inflections(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut forms: Py = Py::None;
    let _ = py_str("The forms of a table word people actually type: \"separately\" and \"separates\" ask the same\n    question as \"separate\", and the matcher does not make them from the base word.");
    forms = py_set(vec![word.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&word.clone(), &py_str(" "))); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    let _ = mcall(&forms.clone(), "update", &[py_set(vec![op_add(&word.clone(), &py_str("ly")), op_add(&word.clone(), &py_str("s")), op_add(&word.clone(), &py_str("es")), op_add(&word.clone(), &py_str("ing")), op_add(&word.clone(), &py_str("ed"))])]);
    if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("e")])) {
    let _ = mcall(&forms.clone(), "update", &[py_set(vec![op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ing")), op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ed")), op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ly"))])]);
    }
    }
    return forms.clone();
    Py::None
}
pub fn _spelling(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut tip: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Why the word this line asks about is spelled the way it is.");
    clean = mcall(&{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    let __iter1 = py_iter_or_raise(&mcall(&mcall(&mcall(&mcall(&clean.clone(), "replace", &[py_str("?"), py_str(" ")]), "replace", &[py_str(","), py_str(" ")]), "replace", &[py_str(":"), py_str(" ")]), "split", &[]));
    for __item in __iter1 {
    word = __item.clone();
    tip = mcall(&g__SPELLING_TIPS(), "get", &[mcall(&word.clone(), "strip", &[py_str(".")])]);
    if py_truthy(&tip.clone()) {
    return tip.clone();
    }
    }
    let __iter2 = py_iter_or_raise(&mcall(&g__SPELLING_TIPS(), "items", &[]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    phrase = __unpk3[0].clone();
    tip = __unpk3[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&phrase.clone(), &py_str(" "))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &phrase.clone())) } }) {
    return tip.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _eulogy(args: &[Py]) -> Py {
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
    let mut has: Py = Py::None;
    let _ = py_str("The structure, the length and the opening line, depending on what was asked.");
    clean = mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    has = { let clean = clean.clone(); let has = has.clone(); let memory = memory.clone(); let understood = understood.clone(); py_func("lambda", move |args: &[Py]| { let clean = clean.clone(); let has = has.clone(); let memory = memory.clone(); let understood = understood.clone(); let wanted = py_tuple(args.get(0..).unwrap_or(&[]).to_vec()); call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut shape: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &shape.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) }) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.extend(py_iter_or_raise(&g__EULOGY_LENGTH())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&has, args))(&__args) }) {
    return py_str("A eulogy runs about three to five minutes, so 500 to 750 words read slowly - time it out loud once, because reading to a room is slower than reading to yourself. Shorter and warm beats long and solemn: two minutes with one real story in it lands better than ten minutes of a life in order, and nobody has ever wished a eulogy had listed more jobs.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.extend(py_iter_or_raise(&g__EULOGY_START())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&has, args))(&__args) }) {
    return py_str("Start with a plain, specific fact about them, not a summary: \"Dad was born in Leeds and never once chose a different chippy.\" Then one story that only your family could tell, with a detail in it - what they said, what they were wearing, the thing they did with their hands. Close on what they gave you, in one sentence, and step back; the last line does not have to be poetry, it has to be theirs.");
    }
    return py_str("Write it in four short parts: who they were in a sentence, one story with a real detail in it, one thing they gave you, and a last line to the room. Aim for 500 to 750 words - three to five minutes out loud - and read it from paper rather than memory. Say the funny thing if it was funny, because a laugh at a funeral is a relief and not a mistake, and put the plainest, truest sentence you have near the end where it will be heard.");
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
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    return Py::Bool(!py_is(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None));
    Py::None
}
pub fn _follow_up_general(args: &[Py]) -> Py {
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
    let _ = py_str("A line that asks how long to wait before following up, with nothing named in it at all.");
    return g_FOLLOW_UP_WHEN();
    Py::None
}
pub fn _asks_for_the_time(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line is a request for time off, not a statement that the time is theirs.\n\n    The reply function can stand down, but the row the table picks should be the row that\n    answers, or a test that reads the table sees the wrong one. Both checks live here.\n    ");
    said = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ALREADY_HAD()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__AWAY_OR_HOME()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__USE_THE_WEEK()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__MONTHS_NAMED()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__HOW_LONG()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ASKS_TO_HAVE()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__HAS_THE_TIME()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ASKING_FRAME()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ASKING_FRAME()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _time_off(args: &[Py]) -> Py {
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
    let mut asking: Py = Py::None;
    let mut early: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A day off wants the dates; an afternoon off wants the time and who covers it.");
    said = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    early = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_EARLY_WORDS()) { let mut phrase: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &phrase.clone()), &py_str(" ")))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &phrase.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    asking = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_YES_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&early.clone()) {
    return if py_truthy(&asking.clone()) { g_EARLY_SCRIPT_YES() } else { g_EARLY_SCRIPT() };
    }
    return if py_truthy(&asking.clone()) { g_TIME_OFF_SCRIPT_YES() } else { g_TIME_OFF_SCRIPT() };
    Py::None
}
pub fn _follow_up(args: &[Py]) -> Py {
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
    let mut chasing: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thanked: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The thank-you note if there was an interview, the chase if there has not been one yet.");
    said = { let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    thanked = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_INTERVIEW_WORDS()) { let mut word: Py = __item.clone(); __out.push(_said(&[said.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    chasing = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CHASE_WORDS()) { let mut word: Py = __item.clone(); __out.push(_said(&[said.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_WHEN_WORDS()) { let mut word: Py = __item.clone(); __out.push(_said(&[said.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return if py_truthy(&thanked.clone()) { g_FOLLOW_UP_AFTER_INTERVIEW() } else { g_FOLLOW_UP_CHASE() };
    }
    if py_truthy(&thanked.clone()) {
    return mcall(&py_str(" "), "join", &[py_tuple(vec![g_FOLLOW_UP_THANKS(), g_FOLLOW_UP_CHASE()])]);
    }
    if py_truthy(&chasing.clone()) {
    return mcall(&py_str(" "), "join", &[py_tuple(vec![g_FOLLOW_UP_CHASE(), g_FOLLOW_UP_THANKS()])]);
    }
    return mcall(&py_str(" "), "join", &[py_tuple(vec![g_FOLLOW_UP_THANKS(), g_FOLLOW_UP_CHASE()])]);
    Py::None
}
pub fn sign_said(args: &[Py]) -> Py {
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
    let mut label: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut phrases: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The sign the person actually named, in their words, or None when they named none.");
    said = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    let __iter1 = py_iter_or_raise(&g_SIGNS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    phrases = __unpk2[0].clone();
    label = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrases.clone()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return label.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn nerves_opening(args: &[Py]) -> Py {
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
    let mut situation: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "situation") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: situation"))) };
    let mut tail: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tail"))) };
    let mut label: Py = Py::None;
    let _ = py_str("`Shaky hands before an interview are ...` in the person's own words.\n\n    `situation` is \"before an exam\"; `tail` starts with the predicate (\"adrenaline, not a bug\").\n    A line that names no sign gets \"That feeling\", which is true of every one of them.\n    ");
    label = sign_said(&[text.clone()]);
    if py_truthy(&Py::Bool(py_is(&label.clone(), &Py::None))) {
    return op_add(&op_add(&op_add(&py_str("That feeling "), &situation.clone()), &py_str(" is ")), &tail.clone());
    }
    return op_add(&op_add(&op_add(&op_add(&label.clone(), &py_str(" ")), &situation.clone()), &if py_truthy(&Py::Bool(py_contains(&g__SIGN_PLURAL(), &label.clone()))) { py_str(" are ") } else { py_str(" is ") }), &tail.clone());
    Py::None
}
pub fn _comfort(args: &[Py]) -> Py {
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
    let mut earlier: Py = Py::None;
    let mut leans: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The first sentence for the bad thing the line names, and the same rule under it.");
    said = op_add(&op_add(&py_str(" "), &mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])), &py_str(" "));
    leans = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("leans_on")]);
    earlier = py_str("");
    if py_truthy(&{ let __b = call_builtin("isinstance", &[leans.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[mcall(&leans.clone(), "get", &[py_str("words")]), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    earlier = op_add(&op_add(&py_str(" "), &mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&leans.clone(), &py_str("words"))) { let mut w: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[w.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(w.clone()); } } py_list(__out) })() }]), "lower", &[])), &py_str(" "));
    }
    let __iter1 = py_iter_or_raise(&g_COMFORT());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    reply = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&words.clone());
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&word.clone(), &py_str(" ")))) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" ")))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&earlier.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" ")))) } }) {
    return reply.clone();
    }
    } else {
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), op_add(&said.clone(), &earlier.clone())]).unwrap_or_else(|e| exc::raise(e))) {
    return reply.clone();
    }
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&g_COMFORT_SITUATIONS(), &g_COMFORT_UNNAMED())) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), op_add(&said.clone(), &earlier.clone())]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    return g_COMFORT_LAST();
    Py::None
}
pub fn _the_notice_is_being_written(args: &[Py]) -> Py {
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
    let mut reading: Py = Py::None;
    let _ = py_str("Whether the notice is being written now, rather than reported after the fact.");
    reading = { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    return call_builtin("bool", &[mcall(&g__BEING_WRITTEN(), "search", &[call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.wording".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ASKING" => Some(g_ASKING()),
            "A_BEHAVIOUR_TOWARD_THE_ASKER" => Some(g_A_BEHAVIOUR_TOWARD_THE_ASKER()),
            "A_JOB_LEFT_UNDONE" => Some(g_A_JOB_LEFT_UNDONE()),
            "CHASE_WORDS" => Some(g_CHASE_WORDS()),
            "COMFORT" => Some(g_COMFORT()),
            "COMFORT_LAST" => Some(g_COMFORT_LAST()),
            "COMFORT_PLAIN" => Some(g_COMFORT_PLAIN()),
            "COMFORT_SAY" => Some(g_COMFORT_SAY()),
            "COMFORT_SITUATIONS" => Some(g_COMFORT_SITUATIONS()),
            "COMFORT_UNNAMED" => Some(g_COMFORT_UNNAMED()),
            "EARLY_SCRIPT" => Some(g_EARLY_SCRIPT()),
            "EARLY_SCRIPT_YES" => Some(g_EARLY_SCRIPT_YES()),
            "EARLY_WORDS" => Some(g_EARLY_WORDS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "FOLLOW_UP_AFTER_INTERVIEW" => Some(g_FOLLOW_UP_AFTER_INTERVIEW()),
            "FOLLOW_UP_ASK" => Some(g_FOLLOW_UP_ASK()),
            "FOLLOW_UP_CHASE" => Some(g_FOLLOW_UP_CHASE()),
            "FOLLOW_UP_THANKS" => Some(g_FOLLOW_UP_THANKS()),
            "FOLLOW_UP_WHEN" => Some(g_FOLLOW_UP_WHEN()),
            "INTERVIEW_WORDS" => Some(g_INTERVIEW_WORDS()),
            "LEAVING_SPEECH" => Some(g_LEAVING_SPEECH()),
            "MACHINE_SOUNDS" => Some(g_MACHINE_SOUNDS()),
            "MOURNING" => Some(g_MOURNING()),
            "MY_DOG_NOISE" => Some(g_MY_DOG_NOISE()),
            "NERVES_BODY" => Some(g_NERVES_BODY()),
            "NOISE_WORDS" => Some(g_NOISE_WORDS()),
            "NOT_A_JOB" => Some(g_NOT_A_JOB()),
            "ON_THE_PHONE" => Some(g_ON_THE_PHONE()),
            "SAID" => Some(g_SAID()),
            "SAY" => Some(g_SAY()),
            "SIGNS" => Some(g_SIGNS()),
            "SLIPS" => Some(g_SLIPS()),
            "SPELLINGS" => Some(g_SPELLINGS()),
            "TALKING_IS_NOT_A_CHAT" => Some(g_TALKING_IS_NOT_A_CHAT()),
            "THANKS_ANY" => Some(g_THANKS_ANY()),
            "THANKS_FAVOUR" => Some(g_THANKS_FAVOUR()),
            "THANKS_FLOWERS" => Some(g_THANKS_FLOWERS()),
            "THANKS_GIFT" => Some(g_THANKS_GIFT()),
            "THANKS_LIFT" => Some(g_THANKS_LIFT()),
            "THANKS_MEAL" => Some(g_THANKS_MEAL()),
            "THANKS_PERSON" => Some(g_THANKS_PERSON()),
            "THANKS_STAY" => Some(g_THANKS_STAY()),
            "THANKS_WRITTEN" => Some(g_THANKS_WRITTEN()),
            "TIME_OFF_SCRIPT" => Some(g_TIME_OFF_SCRIPT()),
            "TIME_OFF_SCRIPT_YES" => Some(g_TIME_OFF_SCRIPT_YES()),
            "TOLD_OFF" => Some(g_TOLD_OFF()),
            "WHEN_WORDS" => Some(g_WHEN_WORDS()),
            "YES_WORDS" => Some(g_YES_WORDS()),
            "_ALREADY_HAD" => Some(g__ALREADY_HAD()),
            "_ASKING_FRAME" => Some(g__ASKING_FRAME()),
            "_ASKS_FOR_THE_WORDS" => Some(g__ASKS_FOR_THE_WORDS()),
            "_ASKS_TO_HAVE" => Some(g__ASKS_TO_HAVE()),
            "_ASK_VERBS" => Some(g__ASK_VERBS()),
            "_AWAY_OR_HOME" => Some(g__AWAY_OR_HOME()),
            "_A_CHILD_NAMED" => Some(g__A_CHILD_NAMED()),
            "_A_CLOSE_ONE" => Some(g__A_CLOSE_ONE()),
            "_A_COLLEAGUE" => Some(g__A_COLLEAGUE()),
            "_BABY" => Some(g__BABY()),
            "_BEING_WRITTEN" => Some(g__BEING_WRITTEN()),
            "_EULOGY_LENGTH" => Some(g__EULOGY_LENGTH()),
            "_EULOGY_START" => Some(g__EULOGY_START()),
            "_HAS_THE_TIME" => Some(g__HAS_THE_TIME()),
            "_HOUSEHOLD_MONEY" => Some(g__HOUSEHOLD_MONEY()),
            "_HOW_LONG" => Some(g__HOW_LONG()),
            "_HURT_PART" => Some(g__HURT_PART()),
            "_MESS" => Some(g__MESS()),
            "_MONEY" => Some(g__MONEY()),
            "_MONTHS_NAMED" => Some(g__MONTHS_NAMED()),
            "_MOVING_AWAY" => Some(g__MOVING_AWAY()),
            "_MOVING_OUT" => Some(g__MOVING_OUT()),
            "_MY_OWN_SCREENS" => Some(g__MY_OWN_SCREENS()),
            "_NOT_A_SIMPLE_NO" => Some(g__NOT_A_SIMPLE_NO()),
            "_NO_FUSS" => Some(g__NO_FUSS()),
            "_ONLY_SAID_ABOUT_A_CHILD" => Some(g__ONLY_SAID_ABOUT_A_CHILD()),
            "_PARENT_MONEY" => Some(g__PARENT_MONEY()),
            "_RAISE_ANIMAL" => Some(g__RAISE_ANIMAL()),
            "_RAISE_ASK_SUBJECTS" => Some(g__RAISE_ASK_SUBJECTS()),
            "_RAISE_CAT" => Some(g__RAISE_CAT()),
            "_RAISE_CAT_OUTSIDE" => Some(g__RAISE_CAT_OUTSIDE()),
            "_RAISE_CHILD" => Some(g__RAISE_CHILD()),
            "_RAISE_EXAMPLES" => Some(g__RAISE_EXAMPLES()),
            "_RAISE_HOUSE" => Some(g__RAISE_HOUSE()),
            "_RAISE_MONEY" => Some(g__RAISE_MONEY()),
            "_RAISE_PLAIN" => Some(g__RAISE_PLAIN()),
            "_RAISE_SMOKE" => Some(g__RAISE_SMOKE()),
            "_SAID_IT_IS_NEWS" => Some(g__SAID_IT_IS_NEWS()),
            "_SHARED_HOUSE" => Some(g__SHARED_HOUSE()),
            "_SHARE_MONEY" => Some(g__SHARE_MONEY()),
            "_SIGN_PLURAL" => Some(g__SIGN_PLURAL()),
            "_SOMEBODY_AT_HOME" => Some(g__SOMEBODY_AT_HOME()),
            "_SOMEONE_HURT_A_PART" => Some(g__SOMEONE_HURT_A_PART()),
            "_SPELLING_ENTRY_IDS" => Some(g__SPELLING_ENTRY_IDS()),
            "_SPELLING_SLIPS" => Some(g__SPELLING_SLIPS()),
            "_SPELLING_TIPS" => Some(g__SPELLING_TIPS()),
            "_SPELLING_WORDS" => Some(g__SPELLING_WORDS()),
            "_TALKS_OVER" => Some(g__TALKS_OVER()),
            "_THEIR_SCREENS" => Some(g__THEIR_SCREENS()),
            "_TRADES" => Some(g__TRADES()),
            "_USE_THE_WEEK" => Some(g__USE_THE_WEEK()),
            "_asks_for_the_time" => Some(py_func("_asks_for_the_time", |args: &[Py]| _asks_for_the_time(args))),
            "_asks_for_the_words" => Some(py_func("_asks_for_the_words", |args: &[Py]| _asks_for_the_words(args))),
            "_birthday_age" => Some(py_func("_birthday_age", |args: &[Py]| _birthday_age(args))),
            "_card_birthday" => Some(py_func("_card_birthday", |args: &[Py]| _card_birthday(args))),
            "_card_plain" => Some(py_func("_card_plain", |args: &[Py]| _card_plain(args))),
            "_comfort" => Some(py_func("_comfort", |args: &[Py]| _comfort(args))),
            "_eulogy" => Some(py_func("_eulogy", |args: &[Py]| _eulogy(args))),
            "_follow_up" => Some(py_func("_follow_up", |args: &[Py]| _follow_up(args))),
            "_follow_up_general" => Some(py_func("_follow_up_general", |args: &[Py]| _follow_up_general(args))),
            "_inflections" => Some(py_func("_inflections", |args: &[Py]| _inflections(args))),
            "_names_an_age" => Some(py_func("_names_an_age", |args: &[Py]| _names_an_age(args))),
            "_names_one_of" => Some(py_func("_names_one_of", |args: &[Py]| _names_one_of(args))),
            "_not_a_hurt_body" => Some(py_func("_not_a_hurt_body", |args: &[Py]| _not_a_hurt_body(args))),
            "_ordinal" => Some(py_func("_ordinal", |args: &[Py]| _ordinal(args))),
            "_raise_example" => Some(py_func("_raise_example", |args: &[Py]| _raise_example(args))),
            "_raise_problem" => Some(py_func("_raise_problem", |args: &[Py]| _raise_problem(args))),
            "_said" => Some(py_func("_said", |args: &[Py]| _said(args))),
            "_saying_it" => Some(py_func("_saying_it", |args: &[Py]| _saying_it(args))),
            "_slip" => Some(g__slip()),
            "_spelling" => Some(py_func("_spelling", |args: &[Py]| _spelling(args))),
            "_thanks_has" => Some(py_func("_thanks_has", |args: &[Py]| _thanks_has(args))),
            "_the_notice_is_being_written" => Some(py_func("_the_notice_is_being_written", |args: &[Py]| _the_notice_is_being_written(args))),
            "_time_off" => Some(py_func("_time_off", |args: &[Py]| _time_off(args))),
            "entry" => Some(g_entry()),
            "nerves_opening" => Some(py_func("nerves_opening", |args: &[Py]| nerves_opening(args))),
            "owns_the_deed" => Some(g_owns_the_deed()),
            "screens_are_a_childs" => Some(py_func("screens_are_a_childs", |args: &[Py]| screens_are_a_childs(args))),
            "screens_are_somebody_elses" => Some(py_func("screens_are_somebody_elses", |args: &[Py]| screens_are_somebody_elses(args))),
            "sign_said" => Some(py_func("sign_said", |args: &[Py]| sign_said(args))),
            "thanks_note" => Some(py_func("thanks_note", |args: &[Py]| thanks_note(args))),
            "the_fault" => Some(g_the_fault()),
            "the_line_said" => Some(py_func("the_line_said", |args: &[Py]| the_line_said(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ASKING" => s_ASKING(v),
            "A_BEHAVIOUR_TOWARD_THE_ASKER" => s_A_BEHAVIOUR_TOWARD_THE_ASKER(v),
            "A_JOB_LEFT_UNDONE" => s_A_JOB_LEFT_UNDONE(v),
            "CHASE_WORDS" => s_CHASE_WORDS(v),
            "COMFORT" => s_COMFORT(v),
            "COMFORT_LAST" => s_COMFORT_LAST(v),
            "COMFORT_PLAIN" => s_COMFORT_PLAIN(v),
            "COMFORT_SAY" => s_COMFORT_SAY(v),
            "COMFORT_SITUATIONS" => s_COMFORT_SITUATIONS(v),
            "COMFORT_UNNAMED" => s_COMFORT_UNNAMED(v),
            "EARLY_SCRIPT" => s_EARLY_SCRIPT(v),
            "EARLY_SCRIPT_YES" => s_EARLY_SCRIPT_YES(v),
            "EARLY_WORDS" => s_EARLY_WORDS(v),
            "ENTRIES" => s_ENTRIES(v),
            "FOLLOW_UP_AFTER_INTERVIEW" => s_FOLLOW_UP_AFTER_INTERVIEW(v),
            "FOLLOW_UP_ASK" => s_FOLLOW_UP_ASK(v),
            "FOLLOW_UP_CHASE" => s_FOLLOW_UP_CHASE(v),
            "FOLLOW_UP_THANKS" => s_FOLLOW_UP_THANKS(v),
            "FOLLOW_UP_WHEN" => s_FOLLOW_UP_WHEN(v),
            "INTERVIEW_WORDS" => s_INTERVIEW_WORDS(v),
            "LEAVING_SPEECH" => s_LEAVING_SPEECH(v),
            "MACHINE_SOUNDS" => s_MACHINE_SOUNDS(v),
            "MOURNING" => s_MOURNING(v),
            "MY_DOG_NOISE" => s_MY_DOG_NOISE(v),
            "NERVES_BODY" => s_NERVES_BODY(v),
            "NOISE_WORDS" => s_NOISE_WORDS(v),
            "NOT_A_JOB" => s_NOT_A_JOB(v),
            "ON_THE_PHONE" => s_ON_THE_PHONE(v),
            "SAID" => s_SAID(v),
            "SAY" => s_SAY(v),
            "SIGNS" => s_SIGNS(v),
            "SLIPS" => s_SLIPS(v),
            "SPELLINGS" => s_SPELLINGS(v),
            "TALKING_IS_NOT_A_CHAT" => s_TALKING_IS_NOT_A_CHAT(v),
            "THANKS_ANY" => s_THANKS_ANY(v),
            "THANKS_FAVOUR" => s_THANKS_FAVOUR(v),
            "THANKS_FLOWERS" => s_THANKS_FLOWERS(v),
            "THANKS_GIFT" => s_THANKS_GIFT(v),
            "THANKS_LIFT" => s_THANKS_LIFT(v),
            "THANKS_MEAL" => s_THANKS_MEAL(v),
            "THANKS_PERSON" => s_THANKS_PERSON(v),
            "THANKS_STAY" => s_THANKS_STAY(v),
            "THANKS_WRITTEN" => s_THANKS_WRITTEN(v),
            "TIME_OFF_SCRIPT" => s_TIME_OFF_SCRIPT(v),
            "TIME_OFF_SCRIPT_YES" => s_TIME_OFF_SCRIPT_YES(v),
            "TOLD_OFF" => s_TOLD_OFF(v),
            "WHEN_WORDS" => s_WHEN_WORDS(v),
            "YES_WORDS" => s_YES_WORDS(v),
            "_ALREADY_HAD" => s__ALREADY_HAD(v),
            "_ASKING_FRAME" => s__ASKING_FRAME(v),
            "_ASKS_FOR_THE_WORDS" => s__ASKS_FOR_THE_WORDS(v),
            "_ASKS_TO_HAVE" => s__ASKS_TO_HAVE(v),
            "_ASK_VERBS" => s__ASK_VERBS(v),
            "_AWAY_OR_HOME" => s__AWAY_OR_HOME(v),
            "_A_CHILD_NAMED" => s__A_CHILD_NAMED(v),
            "_A_CLOSE_ONE" => s__A_CLOSE_ONE(v),
            "_A_COLLEAGUE" => s__A_COLLEAGUE(v),
            "_BABY" => s__BABY(v),
            "_BEING_WRITTEN" => s__BEING_WRITTEN(v),
            "_EULOGY_LENGTH" => s__EULOGY_LENGTH(v),
            "_EULOGY_START" => s__EULOGY_START(v),
            "_HAS_THE_TIME" => s__HAS_THE_TIME(v),
            "_HOUSEHOLD_MONEY" => s__HOUSEHOLD_MONEY(v),
            "_HOW_LONG" => s__HOW_LONG(v),
            "_HURT_PART" => s__HURT_PART(v),
            "_MESS" => s__MESS(v),
            "_MONEY" => s__MONEY(v),
            "_MONTHS_NAMED" => s__MONTHS_NAMED(v),
            "_MOVING_AWAY" => s__MOVING_AWAY(v),
            "_MOVING_OUT" => s__MOVING_OUT(v),
            "_MY_OWN_SCREENS" => s__MY_OWN_SCREENS(v),
            "_NOT_A_SIMPLE_NO" => s__NOT_A_SIMPLE_NO(v),
            "_NO_FUSS" => s__NO_FUSS(v),
            "_ONLY_SAID_ABOUT_A_CHILD" => s__ONLY_SAID_ABOUT_A_CHILD(v),
            "_PARENT_MONEY" => s__PARENT_MONEY(v),
            "_RAISE_ANIMAL" => s__RAISE_ANIMAL(v),
            "_RAISE_ASK_SUBJECTS" => s__RAISE_ASK_SUBJECTS(v),
            "_RAISE_CAT" => s__RAISE_CAT(v),
            "_RAISE_CAT_OUTSIDE" => s__RAISE_CAT_OUTSIDE(v),
            "_RAISE_CHILD" => s__RAISE_CHILD(v),
            "_RAISE_EXAMPLES" => s__RAISE_EXAMPLES(v),
            "_RAISE_HOUSE" => s__RAISE_HOUSE(v),
            "_RAISE_MONEY" => s__RAISE_MONEY(v),
            "_RAISE_PLAIN" => s__RAISE_PLAIN(v),
            "_RAISE_SMOKE" => s__RAISE_SMOKE(v),
            "_SAID_IT_IS_NEWS" => s__SAID_IT_IS_NEWS(v),
            "_SHARED_HOUSE" => s__SHARED_HOUSE(v),
            "_SHARE_MONEY" => s__SHARE_MONEY(v),
            "_SIGN_PLURAL" => s__SIGN_PLURAL(v),
            "_SOMEBODY_AT_HOME" => s__SOMEBODY_AT_HOME(v),
            "_SOMEONE_HURT_A_PART" => s__SOMEONE_HURT_A_PART(v),
            "_SPELLING_ENTRY_IDS" => s__SPELLING_ENTRY_IDS(v),
            "_SPELLING_SLIPS" => s__SPELLING_SLIPS(v),
            "_SPELLING_TIPS" => s__SPELLING_TIPS(v),
            "_SPELLING_WORDS" => s__SPELLING_WORDS(v),
            "_TALKS_OVER" => s__TALKS_OVER(v),
            "_THEIR_SCREENS" => s__THEIR_SCREENS(v),
            "_TRADES" => s__TRADES(v),
            "_USE_THE_WEEK" => s__USE_THE_WEEK(v),
            "_slip" => s__slip(v),
            "entry" => s_entry(v),
            "owns_the_deed" => s_owns_the_deed(v),
            "the_fault" => s_the_fault(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.wording", module_obj);
    pyrt::xrefs::reg_fn("knowledge.wording", "the_line_said", py_func("the_line_said", |args: &[Py]| the_line_said(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_names_one_of", py_func("_names_one_of", |args: &[Py]| _names_one_of(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "screens_are_a_childs", py_func("screens_are_a_childs", |args: &[Py]| screens_are_a_childs(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "screens_are_somebody_elses", py_func("screens_are_somebody_elses", |args: &[Py]| screens_are_somebody_elses(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_birthday_age", py_func("_birthday_age", |args: &[Py]| _birthday_age(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_ordinal", py_func("_ordinal", |args: &[Py]| _ordinal(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_names_an_age", py_func("_names_an_age", |args: &[Py]| _names_an_age(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_card_birthday", py_func("_card_birthday", |args: &[Py]| _card_birthday(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_card_plain", py_func("_card_plain", |args: &[Py]| _card_plain(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_asks_for_the_words", py_func("_asks_for_the_words", |args: &[Py]| _asks_for_the_words(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_raise_example", py_func("_raise_example", |args: &[Py]| _raise_example(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_not_a_hurt_body", py_func("_not_a_hurt_body", |args: &[Py]| _not_a_hurt_body(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_raise_problem", py_func("_raise_problem", |args: &[Py]| _raise_problem(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_saying_it", py_func("_saying_it", |args: &[Py]| _saying_it(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_thanks_has", py_func("_thanks_has", |args: &[Py]| _thanks_has(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "thanks_note", py_func("thanks_note", |args: &[Py]| thanks_note(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_inflections", py_func("_inflections", |args: &[Py]| _inflections(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_spelling", py_func("_spelling", |args: &[Py]| _spelling(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_eulogy", py_func("_eulogy", |args: &[Py]| _eulogy(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_said", py_func("_said", |args: &[Py]| _said(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_follow_up_general", py_func("_follow_up_general", |args: &[Py]| _follow_up_general(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_asks_for_the_time", py_func("_asks_for_the_time", |args: &[Py]| _asks_for_the_time(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_time_off", py_func("_time_off", |args: &[Py]| _time_off(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_follow_up", py_func("_follow_up", |args: &[Py]| _follow_up(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "sign_said", py_func("sign_said", |args: &[Py]| sign_said(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "nerves_opening", py_func("nerves_opening", |args: &[Py]| nerves_opening(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_comfort", py_func("_comfort", |args: &[Py]| _comfort(args)));
    pyrt::xrefs::reg_fn("knowledge.wording", "_the_notice_is_being_written", py_func("_the_notice_is_being_written", |args: &[Py]| _the_notice_is_being_written(args)));
    pyrt::xrefs::reg_access("knowledge.wording", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.wording", "ASKING", g_ASKING, s_ASKING);
    pyrt::xrefs::reg_access("knowledge.wording", "A_BEHAVIOUR_TOWARD_THE_ASKER", g_A_BEHAVIOUR_TOWARD_THE_ASKER, s_A_BEHAVIOUR_TOWARD_THE_ASKER);
    pyrt::xrefs::reg_access("knowledge.wording", "A_JOB_LEFT_UNDONE", g_A_JOB_LEFT_UNDONE, s_A_JOB_LEFT_UNDONE);
    pyrt::xrefs::reg_access("knowledge.wording", "CHASE_WORDS", g_CHASE_WORDS, s_CHASE_WORDS);
    pyrt::xrefs::reg_access("knowledge.wording", "COMFORT", g_COMFORT, s_COMFORT);
    pyrt::xrefs::reg_access("knowledge.wording", "COMFORT_LAST", g_COMFORT_LAST, s_COMFORT_LAST);
    pyrt::xrefs::reg_access("knowledge.wording", "COMFORT_PLAIN", g_COMFORT_PLAIN, s_COMFORT_PLAIN);
    pyrt::xrefs::reg_access("knowledge.wording", "COMFORT_SAY", g_COMFORT_SAY, s_COMFORT_SAY);
    pyrt::xrefs::reg_access("knowledge.wording", "COMFORT_SITUATIONS", g_COMFORT_SITUATIONS, s_COMFORT_SITUATIONS);
    pyrt::xrefs::reg_access("knowledge.wording", "COMFORT_UNNAMED", g_COMFORT_UNNAMED, s_COMFORT_UNNAMED);
    pyrt::xrefs::reg_access("knowledge.wording", "EARLY_SCRIPT", g_EARLY_SCRIPT, s_EARLY_SCRIPT);
    pyrt::xrefs::reg_access("knowledge.wording", "EARLY_SCRIPT_YES", g_EARLY_SCRIPT_YES, s_EARLY_SCRIPT_YES);
    pyrt::xrefs::reg_access("knowledge.wording", "EARLY_WORDS", g_EARLY_WORDS, s_EARLY_WORDS);
    pyrt::xrefs::reg_access("knowledge.wording", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.wording", "FOLLOW_UP_AFTER_INTERVIEW", g_FOLLOW_UP_AFTER_INTERVIEW, s_FOLLOW_UP_AFTER_INTERVIEW);
    pyrt::xrefs::reg_access("knowledge.wording", "FOLLOW_UP_ASK", g_FOLLOW_UP_ASK, s_FOLLOW_UP_ASK);
    pyrt::xrefs::reg_access("knowledge.wording", "FOLLOW_UP_CHASE", g_FOLLOW_UP_CHASE, s_FOLLOW_UP_CHASE);
    pyrt::xrefs::reg_access("knowledge.wording", "FOLLOW_UP_THANKS", g_FOLLOW_UP_THANKS, s_FOLLOW_UP_THANKS);
    pyrt::xrefs::reg_access("knowledge.wording", "FOLLOW_UP_WHEN", g_FOLLOW_UP_WHEN, s_FOLLOW_UP_WHEN);
    pyrt::xrefs::reg_access("knowledge.wording", "INTERVIEW_WORDS", g_INTERVIEW_WORDS, s_INTERVIEW_WORDS);
    pyrt::xrefs::reg_access("knowledge.wording", "LEAVING_SPEECH", g_LEAVING_SPEECH, s_LEAVING_SPEECH);
    pyrt::xrefs::reg_access("knowledge.wording", "MACHINE_SOUNDS", g_MACHINE_SOUNDS, s_MACHINE_SOUNDS);
    pyrt::xrefs::reg_access("knowledge.wording", "MOURNING", g_MOURNING, s_MOURNING);
    pyrt::xrefs::reg_access("knowledge.wording", "MY_DOG_NOISE", g_MY_DOG_NOISE, s_MY_DOG_NOISE);
    pyrt::xrefs::reg_access("knowledge.wording", "NERVES_BODY", g_NERVES_BODY, s_NERVES_BODY);
    pyrt::xrefs::reg_access("knowledge.wording", "NOISE_WORDS", g_NOISE_WORDS, s_NOISE_WORDS);
    pyrt::xrefs::reg_access("knowledge.wording", "NOT_A_JOB", g_NOT_A_JOB, s_NOT_A_JOB);
    pyrt::xrefs::reg_access("knowledge.wording", "ON_THE_PHONE", g_ON_THE_PHONE, s_ON_THE_PHONE);
    pyrt::xrefs::reg_access("knowledge.wording", "SAID", g_SAID, s_SAID);
    pyrt::xrefs::reg_access("knowledge.wording", "SAY", g_SAY, s_SAY);
    pyrt::xrefs::reg_access("knowledge.wording", "SIGNS", g_SIGNS, s_SIGNS);
    pyrt::xrefs::reg_access("knowledge.wording", "SLIPS", g_SLIPS, s_SLIPS);
    pyrt::xrefs::reg_access("knowledge.wording", "SPELLINGS", g_SPELLINGS, s_SPELLINGS);
    pyrt::xrefs::reg_access("knowledge.wording", "TALKING_IS_NOT_A_CHAT", g_TALKING_IS_NOT_A_CHAT, s_TALKING_IS_NOT_A_CHAT);
    pyrt::xrefs::reg_access("knowledge.wording", "THANKS_ANY", g_THANKS_ANY, s_THANKS_ANY);
    pyrt::xrefs::reg_access("knowledge.wording", "THANKS_FAVOUR", g_THANKS_FAVOUR, s_THANKS_FAVOUR);
    pyrt::xrefs::reg_access("knowledge.wording", "THANKS_FLOWERS", g_THANKS_FLOWERS, s_THANKS_FLOWERS);
    pyrt::xrefs::reg_access("knowledge.wording", "THANKS_GIFT", g_THANKS_GIFT, s_THANKS_GIFT);
    pyrt::xrefs::reg_access("knowledge.wording", "THANKS_LIFT", g_THANKS_LIFT, s_THANKS_LIFT);
    pyrt::xrefs::reg_access("knowledge.wording", "THANKS_MEAL", g_THANKS_MEAL, s_THANKS_MEAL);
    pyrt::xrefs::reg_access("knowledge.wording", "THANKS_PERSON", g_THANKS_PERSON, s_THANKS_PERSON);
    pyrt::xrefs::reg_access("knowledge.wording", "THANKS_STAY", g_THANKS_STAY, s_THANKS_STAY);
    pyrt::xrefs::reg_access("knowledge.wording", "THANKS_WRITTEN", g_THANKS_WRITTEN, s_THANKS_WRITTEN);
    pyrt::xrefs::reg_access("knowledge.wording", "TIME_OFF_SCRIPT", g_TIME_OFF_SCRIPT, s_TIME_OFF_SCRIPT);
    pyrt::xrefs::reg_access("knowledge.wording", "TIME_OFF_SCRIPT_YES", g_TIME_OFF_SCRIPT_YES, s_TIME_OFF_SCRIPT_YES);
    pyrt::xrefs::reg_access("knowledge.wording", "TOLD_OFF", g_TOLD_OFF, s_TOLD_OFF);
    pyrt::xrefs::reg_access("knowledge.wording", "WHEN_WORDS", g_WHEN_WORDS, s_WHEN_WORDS);
    pyrt::xrefs::reg_access("knowledge.wording", "YES_WORDS", g_YES_WORDS, s_YES_WORDS);
    pyrt::xrefs::reg_access("knowledge.wording", "_ALREADY_HAD", g__ALREADY_HAD, s__ALREADY_HAD);
    pyrt::xrefs::reg_access("knowledge.wording", "_ASKING_FRAME", g__ASKING_FRAME, s__ASKING_FRAME);
    pyrt::xrefs::reg_access("knowledge.wording", "_ASKS_FOR_THE_WORDS", g__ASKS_FOR_THE_WORDS, s__ASKS_FOR_THE_WORDS);
    pyrt::xrefs::reg_access("knowledge.wording", "_ASKS_TO_HAVE", g__ASKS_TO_HAVE, s__ASKS_TO_HAVE);
    pyrt::xrefs::reg_access("knowledge.wording", "_ASK_VERBS", g__ASK_VERBS, s__ASK_VERBS);
    pyrt::xrefs::reg_access("knowledge.wording", "_AWAY_OR_HOME", g__AWAY_OR_HOME, s__AWAY_OR_HOME);
    pyrt::xrefs::reg_access("knowledge.wording", "_A_CHILD_NAMED", g__A_CHILD_NAMED, s__A_CHILD_NAMED);
    pyrt::xrefs::reg_access("knowledge.wording", "_A_CLOSE_ONE", g__A_CLOSE_ONE, s__A_CLOSE_ONE);
    pyrt::xrefs::reg_access("knowledge.wording", "_A_COLLEAGUE", g__A_COLLEAGUE, s__A_COLLEAGUE);
    pyrt::xrefs::reg_access("knowledge.wording", "_BABY", g__BABY, s__BABY);
    pyrt::xrefs::reg_access("knowledge.wording", "_BEING_WRITTEN", g__BEING_WRITTEN, s__BEING_WRITTEN);
    pyrt::xrefs::reg_access("knowledge.wording", "_EULOGY_LENGTH", g__EULOGY_LENGTH, s__EULOGY_LENGTH);
    pyrt::xrefs::reg_access("knowledge.wording", "_EULOGY_START", g__EULOGY_START, s__EULOGY_START);
    pyrt::xrefs::reg_access("knowledge.wording", "_HAS_THE_TIME", g__HAS_THE_TIME, s__HAS_THE_TIME);
    pyrt::xrefs::reg_access("knowledge.wording", "_HOUSEHOLD_MONEY", g__HOUSEHOLD_MONEY, s__HOUSEHOLD_MONEY);
    pyrt::xrefs::reg_access("knowledge.wording", "_HOW_LONG", g__HOW_LONG, s__HOW_LONG);
    pyrt::xrefs::reg_access("knowledge.wording", "_HURT_PART", g__HURT_PART, s__HURT_PART);
    pyrt::xrefs::reg_access("knowledge.wording", "_MESS", g__MESS, s__MESS);
    pyrt::xrefs::reg_access("knowledge.wording", "_MONEY", g__MONEY, s__MONEY);
    pyrt::xrefs::reg_access("knowledge.wording", "_MONTHS_NAMED", g__MONTHS_NAMED, s__MONTHS_NAMED);
    pyrt::xrefs::reg_access("knowledge.wording", "_MOVING_AWAY", g__MOVING_AWAY, s__MOVING_AWAY);
    pyrt::xrefs::reg_access("knowledge.wording", "_MOVING_OUT", g__MOVING_OUT, s__MOVING_OUT);
    pyrt::xrefs::reg_access("knowledge.wording", "_MY_OWN_SCREENS", g__MY_OWN_SCREENS, s__MY_OWN_SCREENS);
    pyrt::xrefs::reg_access("knowledge.wording", "_NOT_A_SIMPLE_NO", g__NOT_A_SIMPLE_NO, s__NOT_A_SIMPLE_NO);
    pyrt::xrefs::reg_access("knowledge.wording", "_NO_FUSS", g__NO_FUSS, s__NO_FUSS);
    pyrt::xrefs::reg_access("knowledge.wording", "_ONLY_SAID_ABOUT_A_CHILD", g__ONLY_SAID_ABOUT_A_CHILD, s__ONLY_SAID_ABOUT_A_CHILD);
    pyrt::xrefs::reg_access("knowledge.wording", "_PARENT_MONEY", g__PARENT_MONEY, s__PARENT_MONEY);
    pyrt::xrefs::reg_access("knowledge.wording", "_RAISE_ANIMAL", g__RAISE_ANIMAL, s__RAISE_ANIMAL);
    pyrt::xrefs::reg_access("knowledge.wording", "_RAISE_ASK_SUBJECTS", g__RAISE_ASK_SUBJECTS, s__RAISE_ASK_SUBJECTS);
    pyrt::xrefs::reg_access("knowledge.wording", "_RAISE_CAT", g__RAISE_CAT, s__RAISE_CAT);
    pyrt::xrefs::reg_access("knowledge.wording", "_RAISE_CAT_OUTSIDE", g__RAISE_CAT_OUTSIDE, s__RAISE_CAT_OUTSIDE);
    pyrt::xrefs::reg_access("knowledge.wording", "_RAISE_CHILD", g__RAISE_CHILD, s__RAISE_CHILD);
    pyrt::xrefs::reg_access("knowledge.wording", "_RAISE_EXAMPLES", g__RAISE_EXAMPLES, s__RAISE_EXAMPLES);
    pyrt::xrefs::reg_access("knowledge.wording", "_RAISE_HOUSE", g__RAISE_HOUSE, s__RAISE_HOUSE);
    pyrt::xrefs::reg_access("knowledge.wording", "_RAISE_MONEY", g__RAISE_MONEY, s__RAISE_MONEY);
    pyrt::xrefs::reg_access("knowledge.wording", "_RAISE_PLAIN", g__RAISE_PLAIN, s__RAISE_PLAIN);
    pyrt::xrefs::reg_access("knowledge.wording", "_RAISE_SMOKE", g__RAISE_SMOKE, s__RAISE_SMOKE);
    pyrt::xrefs::reg_access("knowledge.wording", "_SAID_IT_IS_NEWS", g__SAID_IT_IS_NEWS, s__SAID_IT_IS_NEWS);
    pyrt::xrefs::reg_access("knowledge.wording", "_SHARED_HOUSE", g__SHARED_HOUSE, s__SHARED_HOUSE);
    pyrt::xrefs::reg_access("knowledge.wording", "_SHARE_MONEY", g__SHARE_MONEY, s__SHARE_MONEY);
    pyrt::xrefs::reg_access("knowledge.wording", "_SIGN_PLURAL", g__SIGN_PLURAL, s__SIGN_PLURAL);
    pyrt::xrefs::reg_access("knowledge.wording", "_SOMEBODY_AT_HOME", g__SOMEBODY_AT_HOME, s__SOMEBODY_AT_HOME);
    pyrt::xrefs::reg_access("knowledge.wording", "_SOMEONE_HURT_A_PART", g__SOMEONE_HURT_A_PART, s__SOMEONE_HURT_A_PART);
    pyrt::xrefs::reg_access("knowledge.wording", "_SPELLING_ENTRY_IDS", g__SPELLING_ENTRY_IDS, s__SPELLING_ENTRY_IDS);
    pyrt::xrefs::reg_access("knowledge.wording", "_SPELLING_SLIPS", g__SPELLING_SLIPS, s__SPELLING_SLIPS);
    pyrt::xrefs::reg_access("knowledge.wording", "_SPELLING_TIPS", g__SPELLING_TIPS, s__SPELLING_TIPS);
    pyrt::xrefs::reg_access("knowledge.wording", "_SPELLING_WORDS", g__SPELLING_WORDS, s__SPELLING_WORDS);
    pyrt::xrefs::reg_access("knowledge.wording", "_TALKS_OVER", g__TALKS_OVER, s__TALKS_OVER);
    pyrt::xrefs::reg_access("knowledge.wording", "_THEIR_SCREENS", g__THEIR_SCREENS, s__THEIR_SCREENS);
    pyrt::xrefs::reg_access("knowledge.wording", "_TRADES", g__TRADES, s__TRADES);
    pyrt::xrefs::reg_access("knowledge.wording", "_USE_THE_WEEK", g__USE_THE_WEEK, s__USE_THE_WEEK);
    pyrt::xrefs::reg_access("knowledge.wording", "_slip", g__slip, s__slip);
    pyrt::xrefs::reg_access("knowledge.wording", "entry", g_entry, s_entry);
    pyrt::xrefs::reg_access("knowledge.wording", "owns_the_deed", g_owns_the_deed, s_owns_the_deed);
    pyrt::xrefs::reg_access("knowledge.wording", "the_fault", g_the_fault, s_the_fault);
}

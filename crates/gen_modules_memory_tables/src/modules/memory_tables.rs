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


thread_local! { static G_ADVICE_MODULES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_FIX_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AMOUNT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANY_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKED_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_WITHOUT_MY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AT_TIME_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SHORT_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BAD_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_ECHO_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_SLOTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUDGET_FITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUDGET_MORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUDGET_TIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CALENDAR_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CALL_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAR_CAPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAT_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGE_HEAVY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGE_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECK_A_FEELING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECK_A_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECK_NOTHING_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECK_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_RELATIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOCK_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOSE_HEAVY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOSE_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONCEPTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION_KEEPS_DOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION_RULES_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTED_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTED_AGAIN_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTED_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTION_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTION_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTION_SAY_DROP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTION_SAY_PAIR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT_ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT_BARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_OF_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DENIAL_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DENIAL_STEMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DENIAL_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIET_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIET_RULES_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOG_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DROP_LISTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ELIDED_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ELIDED_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ELIDED_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ELIDED_NOT_SURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ELIDED_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FACT_FAMILY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAMILY_GROUP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FEEDING_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_ANSWER_LEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_QUALIFIER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_UNITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOOD_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRIEF_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROUP_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAVE_ANY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELD_THREAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HER_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_MANY_OF_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IN_LAW_AFTER_TEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEPT_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEY_SLOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIMITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIMIT_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOOK_BACK_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEMORY_POINTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MISSED_CATCH_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTH_ORDER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_BY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_ADVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_FIX_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_GIVEN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_STARTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_ON_A_MEDIUM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_RETRACTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THE_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_PLANS_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_KIND_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWNED_BY_THE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_HELD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_KEYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_GROUP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_AGE_FIX_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_RENAMED_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLURAL_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLURAL_PETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POSSESSIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREGNANT_OWN_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREGNANT_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREGNANT_WEEKS_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRONOUN_FITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRONOUN_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUOTE_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECALL_HINTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RELATION_ALIKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPEAT_ADVERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RESTATED_KEYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RESTRICT_OPENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RESUME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RETRACT_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTINE_SAY_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTINE_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_AS_A_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAME_FACT_THEIR_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLOT_KEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SMALL_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPELLING_APART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STATE_WAS_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SWAPPABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAIL_WINDOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THAT_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOPIC_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TURNED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNIT_FAMILY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNIT_SECONDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEKDAY_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_I_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_END_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_END_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_INVERTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_INVERTED_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_QUALITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_START_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_THING_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_VAGUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD_NUMBERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ABOUT_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AMOUNTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AMOUNT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONVERT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FACT_SLOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SUBJECT_KEYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TENS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__UNIT_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEEKLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.memory_tables: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ADVICE_MODULES() -> Py { ensure_init(); G_ADVICE_MODULES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADVICE_MODULES(v: Py) { ensure_init(); G_ADVICE_MODULES.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_FIX_WORDS() -> Py { ensure_init(); G_AGE_FIX_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_FIX_WORDS(v: Py) { ensure_init(); G_AGE_FIX_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_AMOUNT_WORD() -> Py { ensure_init(); G_AMOUNT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AMOUNT_WORD(v: Py) { ensure_init(); G_AMOUNT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ANY_FIGURE() -> Py { ensure_init(); G_ANY_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANY_FIGURE(v: Py) { ensure_init(); G_ANY_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKED_LEADS() -> Py { ensure_init(); G_ASKED_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKED_LEADS(v: Py) { ensure_init(); G_ASKED_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_WITHOUT_MY() -> Py { ensure_init(); G_ASK_WITHOUT_MY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_WITHOUT_MY(v: Py) { ensure_init(); G_ASK_WITHOUT_MY.with(|c| *c.borrow_mut() = v); }
pub fn g_AT_TIME_VALUE() -> Py { ensure_init(); G_AT_TIME_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AT_TIME_VALUE(v: Py) { ensure_init(); G_AT_TIME_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SHORT_ANSWER() -> Py { ensure_init(); G_A_SHORT_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SHORT_ANSWER(v: Py) { ensure_init(); G_A_SHORT_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_BAD_NEWS() -> Py { ensure_init(); G_BAD_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BAD_NEWS(v: Py) { ensure_init(); G_BAD_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_ECHO_ONLY() -> Py { ensure_init(); G_BARE_ECHO_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_ECHO_ONLY(v: Py) { ensure_init(); G_BARE_ECHO_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_SLOTS() -> Py { ensure_init(); G_BARE_SLOTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_SLOTS(v: Py) { ensure_init(); G_BARE_SLOTS.with(|c| *c.borrow_mut() = v); }
pub fn g_BUDGET_FITS() -> Py { ensure_init(); G_BUDGET_FITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUDGET_FITS(v: Py) { ensure_init(); G_BUDGET_FITS.with(|c| *c.borrow_mut() = v); }
pub fn g_BUDGET_MORE() -> Py { ensure_init(); G_BUDGET_MORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUDGET_MORE(v: Py) { ensure_init(); G_BUDGET_MORE.with(|c| *c.borrow_mut() = v); }
pub fn g_BUDGET_TIGHT() -> Py { ensure_init(); G_BUDGET_TIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUDGET_TIGHT(v: Py) { ensure_init(); G_BUDGET_TIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_CALENDAR_SAID() -> Py { ensure_init(); G_CALENDAR_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALENDAR_SAID(v: Py) { ensure_init(); G_CALENDAR_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_CALL_LEADS() -> Py { ensure_init(); G_CALL_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALL_LEADS(v: Py) { ensure_init(); G_CALL_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_CAR_CAPS() -> Py { ensure_init(); G_CAR_CAPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAR_CAPS(v: Py) { ensure_init(); G_CAR_CAPS.with(|c| *c.borrow_mut() = v); }
pub fn g_CAT_NAMES() -> Py { ensure_init(); G_CAT_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAT_NAMES(v: Py) { ensure_init(); G_CAT_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGE_HEAVY() -> Py { ensure_init(); G_CHANGE_HEAVY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGE_HEAVY(v: Py) { ensure_init(); G_CHANGE_HEAVY.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGE_PLAIN() -> Py { ensure_init(); G_CHANGE_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGE_PLAIN(v: Py) { ensure_init(); G_CHANGE_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECK_A_FEELING() -> Py { ensure_init(); G_CHECK_A_FEELING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECK_A_FEELING(v: Py) { ensure_init(); G_CHECK_A_FEELING.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECK_A_QUESTION() -> Py { ensure_init(); G_CHECK_A_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECK_A_QUESTION(v: Py) { ensure_init(); G_CHECK_A_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECK_NOTHING_SAID() -> Py { ensure_init(); G_CHECK_NOTHING_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECK_NOTHING_SAID(v: Py) { ensure_init(); G_CHECK_NOTHING_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECK_PLAIN() -> Py { ensure_init(); G_CHECK_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECK_PLAIN(v: Py) { ensure_init(); G_CHECK_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD() -> Py { ensure_init(); G_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD(v: Py) { ensure_init(); G_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_OF() -> Py { ensure_init(); G_CHILD_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_OF(v: Py) { ensure_init(); G_CHILD_OF.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_RELATIONS() -> Py { ensure_init(); G_CHILD_RELATIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_RELATIONS(v: Py) { ensure_init(); G_CHILD_RELATIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOCK_SAID() -> Py { ensure_init(); G_CLOCK_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOCK_SAID(v: Py) { ensure_init(); G_CLOCK_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOSE_HEAVY() -> Py { ensure_init(); G_CLOSE_HEAVY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOSE_HEAVY(v: Py) { ensure_init(); G_CLOSE_HEAVY.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOSE_PLAIN() -> Py { ensure_init(); G_CLOSE_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOSE_PLAIN(v: Py) { ensure_init(); G_CLOSE_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_CONCEPTS() -> Py { ensure_init(); G_CONCEPTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONCEPTS(v: Py) { ensure_init(); G_CONCEPTS.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION_KEEPS_DOWN() -> Py { ensure_init(); G_CONDITION_KEEPS_DOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION_KEEPS_DOWN(v: Py) { ensure_init(); G_CONDITION_KEEPS_DOWN.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION_RULES_OUT() -> Py { ensure_init(); G_CONDITION_RULES_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION_RULES_OUT(v: Py) { ensure_init(); G_CONDITION_RULES_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTED_AGAIN() -> Py { ensure_init(); G_CORRECTED_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTED_AGAIN(v: Py) { ensure_init(); G_CORRECTED_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTED_AGAIN_WHO() -> Py { ensure_init(); G_CORRECTED_AGAIN_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTED_AGAIN_WHO(v: Py) { ensure_init(); G_CORRECTED_AGAIN_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTED_SAY() -> Py { ensure_init(); G_CORRECTED_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTED_SAY(v: Py) { ensure_init(); G_CORRECTED_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTION_BACK() -> Py { ensure_init(); G_CORRECTION_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTION_BACK(v: Py) { ensure_init(); G_CORRECTION_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTION_SAY() -> Py { ensure_init(); G_CORRECTION_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTION_SAY(v: Py) { ensure_init(); G_CORRECTION_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTION_SAY_DROP() -> Py { ensure_init(); G_CORRECTION_SAY_DROP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTION_SAY_DROP(v: Py) { ensure_init(); G_CORRECTION_SAY_DROP.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTION_SAY_PAIR() -> Py { ensure_init(); G_CORRECTION_SAY_PAIR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTION_SAY_PAIR(v: Py) { ensure_init(); G_CORRECTION_SAY_PAIR.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT_ANIMAL() -> Py { ensure_init(); G_COUNT_ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT_ANIMAL(v: Py) { ensure_init(); G_COUNT_ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT_BARE() -> Py { ensure_init(); G_COUNT_BARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT_BARE(v: Py) { ensure_init(); G_COUNT_BARE.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT_WORD() -> Py { ensure_init(); G_COUNT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT_WORD(v: Py) { ensure_init(); G_COUNT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT_WORDS() -> Py { ensure_init(); G_COUNT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT_WORDS(v: Py) { ensure_init(); G_COUNT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_ASKS() -> Py { ensure_init(); G_DAY_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_ASKS(v: Py) { ensure_init(); G_DAY_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_IT() -> Py { ensure_init(); G_DAY_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_IT(v: Py) { ensure_init(); G_DAY_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_OF_MONTH() -> Py { ensure_init(); G_DAY_OF_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_OF_MONTH(v: Py) { ensure_init(); G_DAY_OF_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_THING() -> Py { ensure_init(); G_DAY_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_THING(v: Py) { ensure_init(); G_DAY_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_DENIAL_SAY() -> Py { ensure_init(); G_DENIAL_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DENIAL_SAY(v: Py) { ensure_init(); G_DENIAL_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_DENIAL_STEMS() -> Py { ensure_init(); G_DENIAL_STEMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DENIAL_STEMS(v: Py) { ensure_init(); G_DENIAL_STEMS.with(|c| *c.borrow_mut() = v); }
pub fn g_DENIAL_WORDS() -> Py { ensure_init(); G_DENIAL_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DENIAL_WORDS(v: Py) { ensure_init(); G_DENIAL_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_DIET_FOOD() -> Py { ensure_init(); G_DIET_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIET_FOOD(v: Py) { ensure_init(); G_DIET_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_DIET_RULES_OUT() -> Py { ensure_init(); G_DIET_RULES_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIET_RULES_OUT(v: Py) { ensure_init(); G_DIET_RULES_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_DOG_NAMES() -> Py { ensure_init(); G_DOG_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOG_NAMES(v: Py) { ensure_init(); G_DOG_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_DROP_LISTS() -> Py { ensure_init(); G_DROP_LISTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DROP_LISTS(v: Py) { ensure_init(); G_DROP_LISTS.with(|c| *c.borrow_mut() = v); }
pub fn g_ELIDED_AGAIN() -> Py { ensure_init(); G_ELIDED_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ELIDED_AGAIN(v: Py) { ensure_init(); G_ELIDED_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_ELIDED_ASK() -> Py { ensure_init(); G_ELIDED_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ELIDED_ASK(v: Py) { ensure_init(); G_ELIDED_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ELIDED_NAMES() -> Py { ensure_init(); G_ELIDED_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ELIDED_NAMES(v: Py) { ensure_init(); G_ELIDED_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_ELIDED_NOT_SURE() -> Py { ensure_init(); G_ELIDED_NOT_SURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ELIDED_NOT_SURE(v: Py) { ensure_init(); G_ELIDED_NOT_SURE.with(|c| *c.borrow_mut() = v); }
pub fn g_ELIDED_WORDS() -> Py { ensure_init(); G_ELIDED_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ELIDED_WORDS(v: Py) { ensure_init(); G_ELIDED_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FACT_FAMILY() -> Py { ensure_init(); G_FACT_FAMILY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FACT_FAMILY(v: Py) { ensure_init(); G_FACT_FAMILY.with(|c| *c.borrow_mut() = v); }
pub fn g_FAMILY_GROUP() -> Py { ensure_init(); G_FAMILY_GROUP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAMILY_GROUP(v: Py) { ensure_init(); G_FAMILY_GROUP.with(|c| *c.borrow_mut() = v); }
pub fn g_FEEDING_SAY() -> Py { ensure_init(); G_FEEDING_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FEEDING_SAY(v: Py) { ensure_init(); G_FEEDING_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_ANSWER_LEN() -> Py { ensure_init(); G_FIGURE_ANSWER_LEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_ANSWER_LEN(v: Py) { ensure_init(); G_FIGURE_ANSWER_LEN.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_NUMBER() -> Py { ensure_init(); G_FIGURE_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_NUMBER(v: Py) { ensure_init(); G_FIGURE_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_QUALIFIER() -> Py { ensure_init(); G_FIGURE_QUALIFIER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_QUALIFIER(v: Py) { ensure_init(); G_FIGURE_QUALIFIER.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_TIME() -> Py { ensure_init(); G_FIGURE_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_TIME(v: Py) { ensure_init(); G_FIGURE_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_UNITS() -> Py { ensure_init(); G_FIGURE_UNITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_UNITS(v: Py) { ensure_init(); G_FIGURE_UNITS.with(|c| *c.borrow_mut() = v); }
pub fn g_FOOD_PHRASES() -> Py { ensure_init(); G_FOOD_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOOD_PHRASES(v: Py) { ensure_init(); G_FOOD_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_GRIEF_WORDS() -> Py { ensure_init(); G_GRIEF_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRIEF_WORDS(v: Py) { ensure_init(); G_GRIEF_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_GROUP_OF() -> Py { ensure_init(); G_GROUP_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROUP_OF(v: Py) { ensure_init(); G_GROUP_OF.with(|c| *c.borrow_mut() = v); }
pub fn g_HAVE_ANY_WORDS() -> Py { ensure_init(); G_HAVE_ANY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAVE_ANY_WORDS(v: Py) { ensure_init(); G_HAVE_ANY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_HELD_THREAD() -> Py { ensure_init(); G_HELD_THREAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELD_THREAD(v: Py) { ensure_init(); G_HELD_THREAD.with(|c| *c.borrow_mut() = v); }
pub fn g_HER_WORD() -> Py { ensure_init(); G_HER_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HER_WORD(v: Py) { ensure_init(); G_HER_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_MANY_OF_MINE() -> Py { ensure_init(); G_HOW_MANY_OF_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_MANY_OF_MINE(v: Py) { ensure_init(); G_HOW_MANY_OF_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_IN_LAW_AFTER_TEXT() -> Py { ensure_init(); G_IN_LAW_AFTER_TEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IN_LAW_AFTER_TEXT(v: Py) { ensure_init(); G_IN_LAW_AFTER_TEXT.with(|c| *c.borrow_mut() = v); }
pub fn g_KEPT_THINGS() -> Py { ensure_init(); G_KEPT_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEPT_THINGS(v: Py) { ensure_init(); G_KEPT_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_KEY_SLOT() -> Py { ensure_init(); G_KEY_SLOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEY_SLOT(v: Py) { ensure_init(); G_KEY_SLOT.with(|c| *c.borrow_mut() = v); }
pub fn g_KIN() -> Py { ensure_init(); G_KIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIN(v: Py) { ensure_init(); G_KIN.with(|c| *c.borrow_mut() = v); }
pub fn g_KIN_ASK() -> Py { ensure_init(); G_KIN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIN_ASK(v: Py) { ensure_init(); G_KIN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_LENGTH() -> Py { ensure_init(); G_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LENGTH(v: Py) { ensure_init(); G_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_LIMITS() -> Py { ensure_init(); G_LIMITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIMITS(v: Py) { ensure_init(); G_LIMITS.with(|c| *c.borrow_mut() = v); }
pub fn g_LIMIT_SAY() -> Py { ensure_init(); G_LIMIT_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIMIT_SAY(v: Py) { ensure_init(); G_LIMIT_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_LOOK_BACK_FIGURE() -> Py { ensure_init(); G_LOOK_BACK_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOOK_BACK_FIGURE(v: Py) { ensure_init(); G_LOOK_BACK_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_MEMORY_POINTER() -> Py { ensure_init(); G_MEMORY_POINTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEMORY_POINTER(v: Py) { ensure_init(); G_MEMORY_POINTER.with(|c| *c.borrow_mut() = v); }
pub fn g_MISSED_CATCH_UP() -> Py { ensure_init(); G_MISSED_CATCH_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MISSED_CATCH_UP(v: Py) { ensure_init(); G_MISSED_CATCH_UP.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTH_ORDER() -> Py { ensure_init(); G_MONTH_ORDER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTH_ORDER(v: Py) { ensure_init(); G_MONTH_ORDER.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_BY() -> Py { ensure_init(); G_NAMED_BY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_BY(v: Py) { ensure_init(); G_NAMED_BY.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_ADVICE() -> Py { ensure_init(); G_NAME_ADVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_ADVICE(v: Py) { ensure_init(); G_NAME_ADVICE.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_FIX_WORDS() -> Py { ensure_init(); G_NAME_FIX_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_FIX_WORDS(v: Py) { ensure_init(); G_NAME_FIX_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_GIVEN_WORDS() -> Py { ensure_init(); G_NAME_GIVEN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_GIVEN_WORDS(v: Py) { ensure_init(); G_NAME_GIVEN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_STARTED() -> Py { ensure_init(); G_NOTHING_STARTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_STARTED(v: Py) { ensure_init(); G_NOTHING_STARTED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_UNIT() -> Py { ensure_init(); G_NOT_A_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_UNIT(v: Py) { ensure_init(); G_NOT_A_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_ON_A_MEDIUM() -> Py { ensure_init(); G_NOT_ON_A_MEDIUM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_ON_A_MEDIUM(v: Py) { ensure_init(); G_NOT_ON_A_MEDIUM.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_RETRACTED() -> Py { ensure_init(); G_NOT_RETRACTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_RETRACTED(v: Py) { ensure_init(); G_NOT_RETRACTED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THE_PLAN() -> Py { ensure_init(); G_NOT_THE_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_PLAN(v: Py) { ensure_init(); G_NOT_THE_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_PLANS_REPLY() -> Py { ensure_init(); G_NO_PLANS_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_PLANS_REPLY(v: Py) { ensure_init(); G_NO_PLANS_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER_WORDS() -> Py { ensure_init(); G_NUMBER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER_WORDS(v: Py) { ensure_init(); G_NUMBER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_KIND_SAY() -> Py { ensure_init(); G_OTHER_KIND_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_KIND_SAY(v: Py) { ensure_init(); G_OTHER_KIND_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_SAID() -> Py { ensure_init(); G_OTHER_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_SAID(v: Py) { ensure_init(); G_OTHER_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_OWNED_BY_THE_VERB() -> Py { ensure_init(); G_OWNED_BY_THE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWNED_BY_THE_VERB(v: Py) { ensure_init(); G_OWNED_BY_THE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_HELD() -> Py { ensure_init(); G_PART_HELD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_HELD(v: Py) { ensure_init(); G_PART_HELD.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_KEYS() -> Py { ensure_init(); G_PART_KEYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_KEYS(v: Py) { ensure_init(); G_PART_KEYS.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_GROUP() -> Py { ensure_init(); G_PERSON_GROUP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_GROUP(v: Py) { ensure_init(); G_PERSON_GROUP.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_AGE_FIX_WORDS() -> Py { ensure_init(); G_PET_AGE_FIX_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_AGE_FIX_WORDS(v: Py) { ensure_init(); G_PET_AGE_FIX_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_RENAMED_SAY() -> Py { ensure_init(); G_PET_RENAMED_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_RENAMED_SAY(v: Py) { ensure_init(); G_PET_RENAMED_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_WORDS() -> Py { ensure_init(); G_PET_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_WORDS(v: Py) { ensure_init(); G_PET_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_NOT() -> Py { ensure_init(); G_PLACE_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_NOT(v: Py) { ensure_init(); G_PLACE_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_ASKS() -> Py { ensure_init(); G_PLAN_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_ASKS(v: Py) { ensure_init(); G_PLAN_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_MINE() -> Py { ensure_init(); G_PLAN_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_MINE(v: Py) { ensure_init(); G_PLAN_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_PLURAL_PEOPLE() -> Py { ensure_init(); G_PLURAL_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLURAL_PEOPLE(v: Py) { ensure_init(); G_PLURAL_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_PLURAL_PETS() -> Py { ensure_init(); G_PLURAL_PETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLURAL_PETS(v: Py) { ensure_init(); G_PLURAL_PETS.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_BACK() -> Py { ensure_init(); G_POINTS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK(v: Py) { ensure_init(); G_POINTS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_POSSESSIVE() -> Py { ensure_init(); G_POSSESSIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POSSESSIVE(v: Py) { ensure_init(); G_POSSESSIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_PREGNANT_OWN_SAY() -> Py { ensure_init(); G_PREGNANT_OWN_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREGNANT_OWN_SAY(v: Py) { ensure_init(); G_PREGNANT_OWN_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_PREGNANT_SAY() -> Py { ensure_init(); G_PREGNANT_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREGNANT_SAY(v: Py) { ensure_init(); G_PREGNANT_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_PREGNANT_WEEKS_SAY() -> Py { ensure_init(); G_PREGNANT_WEEKS_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREGNANT_WEEKS_SAY(v: Py) { ensure_init(); G_PREGNANT_WEEKS_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_PRONOUN_FITS() -> Py { ensure_init(); G_PRONOUN_FITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRONOUN_FITS(v: Py) { ensure_init(); G_PRONOUN_FITS.with(|c| *c.borrow_mut() = v); }
pub fn g_PRONOUN_PEOPLE() -> Py { ensure_init(); G_PRONOUN_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRONOUN_PEOPLE(v: Py) { ensure_init(); G_PRONOUN_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_QUOTE_LEADS() -> Py { ensure_init(); G_QUOTE_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUOTE_LEADS(v: Py) { ensure_init(); G_QUOTE_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_RECALL_HINTS() -> Py { ensure_init(); G_RECALL_HINTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECALL_HINTS(v: Py) { ensure_init(); G_RECALL_HINTS.with(|c| *c.borrow_mut() = v); }
pub fn g_RELATION_ALIKE() -> Py { ensure_init(); G_RELATION_ALIKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RELATION_ALIKE(v: Py) { ensure_init(); G_RELATION_ALIKE.with(|c| *c.borrow_mut() = v); }
pub fn g_REPEAT_ADVERB() -> Py { ensure_init(); G_REPEAT_ADVERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPEAT_ADVERB(v: Py) { ensure_init(); G_REPEAT_ADVERB.with(|c| *c.borrow_mut() = v); }
pub fn g_RESTATED_KEYS() -> Py { ensure_init(); G_RESTATED_KEYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RESTATED_KEYS(v: Py) { ensure_init(); G_RESTATED_KEYS.with(|c| *c.borrow_mut() = v); }
pub fn g_RESTRICT_OPENS() -> Py { ensure_init(); G_RESTRICT_OPENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RESTRICT_OPENS(v: Py) { ensure_init(); G_RESTRICT_OPENS.with(|c| *c.borrow_mut() = v); }
pub fn g_RESUME() -> Py { ensure_init(); G_RESUME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RESUME(v: Py) { ensure_init(); G_RESUME.with(|c| *c.borrow_mut() = v); }
pub fn g_RETRACT_SAY() -> Py { ensure_init(); G_RETRACT_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RETRACT_SAY(v: Py) { ensure_init(); G_RETRACT_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTINE_SAY_BACK() -> Py { ensure_init(); G_ROUTINE_SAY_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_SAY_BACK(v: Py) { ensure_init(); G_ROUTINE_SAY_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTINE_SUBJECT() -> Py { ensure_init(); G_ROUTINE_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_SUBJECT(v: Py) { ensure_init(); G_ROUTINE_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_ASK() -> Py { ensure_init(); G_SAID_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_ASK(v: Py) { ensure_init(); G_SAID_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_AS_A_SENTENCE() -> Py { ensure_init(); G_SAID_AS_A_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_AS_A_SENTENCE(v: Py) { ensure_init(); G_SAID_AS_A_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_SAME_FACT_THEIR_OWN() -> Py { ensure_init(); G_SAME_FACT_THEIR_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAME_FACT_THEIR_OWN(v: Py) { ensure_init(); G_SAME_FACT_THEIR_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF() -> Py { ensure_init(); G_SELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF(v: Py) { ensure_init(); G_SELF.with(|c| *c.borrow_mut() = v); }
pub fn g_SLOT_KEY() -> Py { ensure_init(); G_SLOT_KEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SLOT_KEY(v: Py) { ensure_init(); G_SLOT_KEY.with(|c| *c.borrow_mut() = v); }
pub fn g_SMALL_NAMES() -> Py { ensure_init(); G_SMALL_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SMALL_NAMES(v: Py) { ensure_init(); G_SMALL_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_SPELLING_APART() -> Py { ensure_init(); G_SPELLING_APART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPELLING_APART(v: Py) { ensure_init(); G_SPELLING_APART.with(|c| *c.borrow_mut() = v); }
pub fn g_STATE_WAS_SAY() -> Py { ensure_init(); G_STATE_WAS_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STATE_WAS_SAY(v: Py) { ensure_init(); G_STATE_WAS_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_SWAPPABLE() -> Py { ensure_init(); G_SWAPPABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SWAPPABLE(v: Py) { ensure_init(); G_SWAPPABLE.with(|c| *c.borrow_mut() = v); }
pub fn g_TAIL_WINDOW() -> Py { ensure_init(); G_TAIL_WINDOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAIL_WINDOW(v: Py) { ensure_init(); G_TAIL_WINDOW.with(|c| *c.borrow_mut() = v); }
pub fn g_THAT_DATE() -> Py { ensure_init(); G_THAT_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THAT_DATE(v: Py) { ensure_init(); G_THAT_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_ASKS() -> Py { ensure_init(); G_TIME_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_ASKS(v: Py) { ensure_init(); G_TIME_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_IT() -> Py { ensure_init(); G_TIME_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_IT(v: Py) { ensure_init(); G_TIME_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_THING() -> Py { ensure_init(); G_TIME_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_THING(v: Py) { ensure_init(); G_TIME_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_TOPIC_ASK() -> Py { ensure_init(); G_TOPIC_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOPIC_ASK(v: Py) { ensure_init(); G_TOPIC_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_TURNED() -> Py { ensure_init(); G_TURNED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TURNED(v: Py) { ensure_init(); G_TURNED.with(|c| *c.borrow_mut() = v); }
pub fn g_UNIT_FAMILY() -> Py { ensure_init(); G_UNIT_FAMILY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNIT_FAMILY(v: Py) { ensure_init(); G_UNIT_FAMILY.with(|c| *c.borrow_mut() = v); }
pub fn g_UNIT_SECONDS() -> Py { ensure_init(); G_UNIT_SECONDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNIT_SECONDS(v: Py) { ensure_init(); G_UNIT_SECONDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEK() -> Py { ensure_init(); G_WEEK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEK(v: Py) { ensure_init(); G_WEEK.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEKDAY_WORD() -> Py { ensure_init(); G_WEEKDAY_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEKDAY_WORD(v: Py) { ensure_init(); G_WEEKDAY_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_I_SAID() -> Py { ensure_init(); G_WHAT_I_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_I_SAID(v: Py) { ensure_init(); G_WHAT_I_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_ASKS() -> Py { ensure_init(); G_WHEN_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_ASKS(v: Py) { ensure_init(); G_WHEN_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_END_ASK() -> Py { ensure_init(); G_WHEN_END_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_END_ASK(v: Py) { ensure_init(); G_WHEN_END_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_END_WORD() -> Py { ensure_init(); G_WHEN_END_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_END_WORD(v: Py) { ensure_init(); G_WHEN_END_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_INVERTED() -> Py { ensure_init(); G_WHEN_INVERTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_INVERTED(v: Py) { ensure_init(); G_WHEN_INVERTED.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_INVERTED_SUBJECT() -> Py { ensure_init(); G_WHEN_INVERTED_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_INVERTED_SUBJECT(v: Py) { ensure_init(); G_WHEN_INVERTED_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_PLAN() -> Py { ensure_init(); G_WHEN_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_PLAN(v: Py) { ensure_init(); G_WHEN_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_QUALITY() -> Py { ensure_init(); G_WHEN_QUALITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_QUALITY(v: Py) { ensure_init(); G_WHEN_QUALITY.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_START_ASK() -> Py { ensure_init(); G_WHEN_START_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_START_ASK(v: Py) { ensure_init(); G_WHEN_START_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_THING() -> Py { ensure_init(); G_WHEN_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_THING(v: Py) { ensure_init(); G_WHEN_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_THING_VERB() -> Py { ensure_init(); G_WHEN_THING_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_THING_VERB(v: Py) { ensure_init(); G_WHEN_THING_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_VAGUE() -> Py { ensure_init(); G_WHEN_VAGUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_VAGUE(v: Py) { ensure_init(); G_WHEN_VAGUE.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_VERB() -> Py { ensure_init(); G_WHEN_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_VERB(v: Py) { ensure_init(); G_WHEN_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_ASK() -> Py { ensure_init(); G_WHERE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_ASK(v: Py) { ensure_init(); G_WHERE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_PLAN() -> Py { ensure_init(); G_WHERE_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_PLAN(v: Py) { ensure_init(); G_WHERE_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD_NUMBERS() -> Py { ensure_init(); G_WORD_NUMBERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD_NUMBERS(v: Py) { ensure_init(); G_WORD_NUMBERS.with(|c| *c.borrow_mut() = v); }
pub fn g__ABOUT_WHO() -> Py { ensure_init(); G__ABOUT_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ABOUT_WHO(v: Py) { ensure_init(); G__ABOUT_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g__AMOUNTS() -> Py { ensure_init(); G__AMOUNTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AMOUNTS(v: Py) { ensure_init(); G__AMOUNTS.with(|c| *c.borrow_mut() = v); }
pub fn g__AMOUNT_WORDS() -> Py { ensure_init(); G__AMOUNT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AMOUNT_WORDS(v: Py) { ensure_init(); G__AMOUNT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__CONVERT() -> Py { ensure_init(); G__CONVERT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONVERT(v: Py) { ensure_init(); G__CONVERT.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT_WORD() -> Py { ensure_init(); G__COUNT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT_WORD(v: Py) { ensure_init(); G__COUNT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__FACT_SLOT() -> Py { ensure_init(); G__FACT_SLOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FACT_SLOT(v: Py) { ensure_init(); G__FACT_SLOT.with(|c| *c.borrow_mut() = v); }
pub fn g__SUBJECT_KEYS() -> Py { ensure_init(); G__SUBJECT_KEYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SUBJECT_KEYS(v: Py) { ensure_init(); G__SUBJECT_KEYS.with(|c| *c.borrow_mut() = v); }
pub fn g__TENS_WORDS() -> Py { ensure_init(); G__TENS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TENS_WORDS(v: Py) { ensure_init(); G__TENS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__UNIT_SAID() -> Py { ensure_init(); G__UNIT_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__UNIT_SAID(v: Py) { ensure_init(); G__UNIT_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__WEEKLY() -> Py { ensure_init(); G__WEEKLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEEKLY(v: Py) { ensure_init(); G__WEEKLY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The constant tables of the fallback composer.\n\nLifted out of `bot/compose/fallback.py`, which is over the Hub's one-mebibyte file\ncap - one file over that cap stops *every* branch in the swarm from being uploaded,\nwhatever it changes. The same lift moved `bot/knowledge/home_diy.py`,\n`bot/modules/crisis_replies.py` and `bot/memory/patterns.py` out of their files.\n\nNothing here is code: `re` patterns, `frozenset`s of words and tuples of reply\nlines, moved as they were written and imported back by name in `fallback.py`.\nGenerated by `tools/lift_fallback_tables.py`, which also refuses to move anything\nwhose value reads a function or a mutable name of the file - run it again rather\nthan editing a table here and leaving the two out of step.\n");
    s_PRIORITY(pyrt::xrefs::xget("memory.store_lines", "PRIORITY"));
    s_ANSWERS(pyrt::xrefs::xget("memory.store_lines", "ANSWERS"));
    s_RECALL_HINTS(pyrt::xrefs::xget("memory.store_lines", "RECALL_HINTS"));
    s_SELF(pyrt::xrefs::xget("memory.store_lines", "SELF"));
    s_NOT_THE_PLAN(pyrt::xrefs::xget("memory.store_lines", "NOT_THE_PLAN"));
    s_NOT_ON_A_MEDIUM(pyrt::xrefs::xget("memory.store_lines", "NOT_ON_A_MEDIUM"));
    s_REPEAT_ADVERB(pyrt::xrefs::xget("memory.store_lines", "REPEAT_ADVERB"));
    s_LIMITS(pyrt::xrefs::xget("memory.store_lines", "LIMITS"));
    s_DENIAL_SAY(pyrt::xrefs::xget("memory.store_lines", "DENIAL_SAY"));
    s_CORRECTED_SAY(pyrt::xrefs::xget("memory.store_lines", "CORRECTED_SAY"));
    s_RETRACT_SAY(pyrt::xrefs::xget("memory.store_lines", "RETRACT_SAY"));
    s_NOT_RETRACTED(pyrt::xrefs::xget("memory.store_lines", "NOT_RETRACTED"));
    s__FACT_SLOT(pyrt::xrefs::xget("memory.store_lines", "_FACT_SLOT"));
    s_PET_AGE_FIX_WORDS(pyrt::xrefs::xget("memory.store_lines", "PET_AGE_FIX_WORDS"));
    s_OTHER_SAID(pyrt::xrefs::xget("memory.store_lines", "OTHER_SAID"));
    s_OTHER_KIND_SAY(pyrt::xrefs::xget("memory.store_lines", "OTHER_KIND_SAY"));
    s_AGE_FIX_WORDS(pyrt::xrefs::xget("memory.store_lines", "AGE_FIX_WORDS"));
    s_NAME_FIX_WORDS(pyrt::xrefs::xget("memory.store_lines", "NAME_FIX_WORDS"));
    s_NAME_GIVEN_WORDS(pyrt::xrefs::xget("memory.store_lines", "NAME_GIVEN_WORDS"));
    s_UNIT_FAMILY(pyrt::xrefs::xget("memory.store_lines", "UNIT_FAMILY"));
    s_FACT_FAMILY(pyrt::xrefs::xget("memory.store_lines", "FACT_FAMILY"));
    s_NOT_A_UNIT(pyrt::xrefs::xget("memory.store_lines", "NOT_A_UNIT"));
    s_AMOUNT_WORD(pyrt::xrefs::xget("memory.store_lines", "AMOUNT_WORD"));
    s_SPELLING_APART(pyrt::xrefs::xget("memory.store_lines", "SPELLING_APART"));
    s_CORRECTION_SAY_DROP(pyrt::xrefs::xget("memory.store_lines", "CORRECTION_SAY_DROP"));
    s_DROP_LISTS(pyrt::xrefs::xget("memory.store_lines", "DROP_LISTS"));
    s_CORRECTION_SAY(pyrt::xrefs::xget("memory.store_lines", "CORRECTION_SAY"));
    s_CORRECTION_SAY_PAIR(pyrt::xrefs::xget("memory.store_lines", "CORRECTION_SAY_PAIR"));
    s_PET_RENAMED_SAY(pyrt::xrefs::xget("memory.store_lines", "PET_RENAMED_SAY"));
    s_BARE_ECHO_ONLY(pyrt::xrefs::xget("memory.store_lines", "BARE_ECHO_ONLY"));
    s_CORRECTION_BACK(pyrt::xrefs::xget("memory.store_lines", "CORRECTION_BACK"));
    s_AT_TIME_VALUE(pyrt::xrefs::xget("memory.store_lines", "AT_TIME_VALUE"));
    s_KIN_ASK(pyrt::xrefs::xget("memory.store_lines", "KIN_ASK"));
    s_IN_LAW_AFTER_TEXT(pyrt::xrefs::xget("memory.store_lines", "IN_LAW_AFTER_TEXT"));
    s_PRONOUN_FITS(pyrt::xrefs::xget("memory.store_lines", "PRONOUN_FITS"));
    s_CHILD_RELATIONS(pyrt::xrefs::xget("memory.store_lines", "CHILD_RELATIONS"));
    s__ABOUT_WHO(pyrt::xrefs::xget("memory.store_lines", "_ABOUT_WHO"));
    s_CHILD(pyrt::xrefs::xget("memory.store_lines", "CHILD"));
    s_CHILD_OF(pyrt::xrefs::xget("memory.store_lines", "CHILD_OF"));
    s_FAMILY_GROUP(pyrt::xrefs::xget("memory.store_lines", "FAMILY_GROUP"));
    s_FOOD_PHRASES(pyrt::xrefs::xget("memory.store_lines", "FOOD_PHRASES"));
    s_ADVICE_MODULES(pyrt::xrefs::xget("memory.store_lines", "ADVICE_MODULES"));
    s_SLOT_KEY(pyrt::xrefs::xget("memory.store_lines", "SLOT_KEY"));
    s_BARE_SLOTS(pyrt::xrefs::xget("memory.store_lines", "BARE_SLOTS"));
    s_WHERE_ASK(pyrt::xrefs::xget("memory.store_lines", "WHERE_ASK"));
    s_KIN(pyrt::xrefs::xget("memory.store_lines", "KIN"));
    s_BAD_NEWS(pyrt::xrefs::xget("memory.store_lines", "BAD_NEWS"));
    s__UNIT_SAID(pyrt::xrefs::xget("memory.store_lines", "_UNIT_SAID"));
    s__AMOUNTS(pyrt::xrefs::xget("memory.store_lines", "_AMOUNTS"));
    s__AMOUNT_WORDS(pyrt::xrefs::xget("memory.store_lines", "_AMOUNT_WORDS"));
    s__COUNT_WORD(pyrt::xrefs::xget("memory.store_lines", "_COUNT_WORD"));
    s__CONVERT(pyrt::xrefs::xget("memory.store_lines", "_CONVERT"));
    s__WEEKLY(pyrt::xrefs::xget("memory.store_lines", "_WEEKLY"));
    s_FIGURE_UNITS(pyrt::xrefs::xget("memory.store_lines", "FIGURE_UNITS"));
    s_FIGURE_TIME(pyrt::xrefs::xget("memory.store_lines", "FIGURE_TIME"));
    s_FIGURE_NUMBER(pyrt::xrefs::xget("memory.store_lines", "FIGURE_NUMBER"));
    s_FIGURE_QUALIFIER(pyrt::xrefs::xget("memory.store_lines", "FIGURE_QUALIFIER"));
    s_LOOK_BACK_FIGURE(pyrt::xrefs::xget("memory.store_lines", "LOOK_BACK_FIGURE"));
    s_FIGURE_ANSWER_LEN(pyrt::xrefs::xget("memory.store_lines", "FIGURE_ANSWER_LEN"));
    s_COUNT_WORD(pyrt::xrefs::xget("memory.store_lines", "COUNT_WORD"));
    s_PLACE_NOT(pyrt::xrefs::xget("memory.store_lines", "PLACE_NOT"));
    s_TURNED(pyrt::xrefs::xget("memory.store_lines", "TURNED"));
    s_PART_HELD(pyrt::xrefs::xget("memory.store_lines", "PART_HELD"));
    s_PART_KEYS(pyrt::xrefs::xget("memory.store_lines", "PART_KEYS"));
    s_KEPT_THINGS(pyrt::xrefs::xget("memory.store_lines", "KEPT_THINGS"));
    s_NAMED_BY(pyrt::xrefs::xget("memory.store_lines", "NAMED_BY"));
    s_RESTATED_KEYS(pyrt::xrefs::xget("memory.store_lines", "RESTATED_KEYS"));
    s_SWAPPABLE(pyrt::xrefs::xget("memory.store_lines", "SWAPPABLE"));
    s__TENS_WORDS(pyrt::xrefs::xget("memory.store_lines", "_TENS_WORDS"));
    s_CORRECTED_AGAIN(pyrt::xrefs::xget("memory.store_lines", "CORRECTED_AGAIN"));
    s_CORRECTED_AGAIN_WHO(pyrt::xrefs::xget("memory.store_lines", "CORRECTED_AGAIN_WHO"));
    s_CAT_NAMES(pyrt::xrefs::xget("memory.store_lines", "CAT_NAMES"));
    s_DOG_NAMES(pyrt::xrefs::xget("memory.store_lines", "DOG_NAMES"));
    s_SMALL_NAMES(pyrt::xrefs::xget("memory.store_lines", "SMALL_NAMES"));
    s_NAME_ADVICE(pyrt::xrefs::xget("memory.store_lines", "NAME_ADVICE"));
    s_RESUME(pyrt::xrefs::xget("memory.store_lines", "RESUME"));
    s_NO_PLANS_REPLY(pyrt::xrefs::xget("memory.store_lines", "NO_PLANS_REPLY"));
    s_WEEK(pyrt::xrefs::xget("memory.store_lines", "WEEK"));
    s_TAIL_WINDOW(pyrt::xrefs::xget("memory.store_lines", "TAIL_WINDOW"));
    s_PLURAL_PEOPLE(pyrt::xrefs::xget("memory.store_lines", "PLURAL_PEOPLE"));
    s_DIET_RULES_OUT(pyrt::xrefs::xget("memory.store_lines", "DIET_RULES_OUT"));
    s_DENIAL_WORDS(pyrt::xrefs::xget("memory.store_lines", "DENIAL_WORDS"));
    s_DENIAL_STEMS(pyrt::xrefs::xget("memory.store_lines", "DENIAL_STEMS"));
    s_CHILD_RELATIONS(pyrt::xrefs::xget("memory.store_lines", "CHILD_RELATIONS"));
    s_GROUP_OF(pyrt::xrefs::xget("memory.store_lines", "GROUP_OF"));
    s_RELATION_ALIKE(pyrt::xrefs::xget("memory.store_lines", "RELATION_ALIKE"));
    s_MONTH_ORDER(pyrt::xrefs::xget("memory.store_lines", "MONTH_ORDER"));
    s_HAVE_ANY_WORDS(pyrt::xrefs::xget("memory.store_lines", "HAVE_ANY_WORDS"));
    s_HER_WORD(pyrt::xrefs::xget("memory.store_lines", "HER_WORD"));
    s_DIET_FOOD(pyrt::xrefs::xget("memory.store_lines", "DIET_FOOD"));
    s_PERSON_GROUP(pyrt::xrefs::xget("memory.store_lines", "PERSON_GROUP"));
    s_PLURAL_PEOPLE(pyrt::xrefs::xget("memory.store_lines", "PLURAL_PEOPLE"));
    s_OWNED_BY_THE_VERB(pyrt::xrefs::xget("memory.store_lines", "OWNED_BY_THE_VERB"));
    s_PREGNANT_OWN_SAY(pyrt::xrefs::xget("memory.store_lines", "PREGNANT_OWN_SAY"));
    s_PREGNANT_SAY(pyrt::xrefs::xget("memory.store_lines", "PREGNANT_SAY"));
    s_PREGNANT_WEEKS_SAY(pyrt::xrefs::xget("memory.store_lines", "PREGNANT_WEEKS_SAY"));
    s_LIMIT_SAY(pyrt::xrefs::xget("memory.store_lines", "LIMIT_SAY"));
    s_FEEDING_SAY(pyrt::xrefs::xget("memory.store_lines", "FEEDING_SAY"));
    s_STATE_WAS_SAY(pyrt::xrefs::xget("memory.store_lines", "STATE_WAS_SAY"));
    s_PET_WORDS(pyrt::xrefs::xget("memory.store_lines", "PET_WORDS"));
    s_PRONOUN_PEOPLE(pyrt::xrefs::xget("memory.store_lines", "PRONOUN_PEOPLE"));
    s_SAME_FACT_THEIR_OWN(pyrt::xrefs::xget("memory.store_lines", "SAME_FACT_THEIR_OWN"));
    s_CONDITION_RULES_OUT(pyrt::xrefs::xget("memory.store_lines", "CONDITION_RULES_OUT"));
    s_CONDITION_KEEPS_DOWN(pyrt::xrefs::xget("memory.store_lines", "CONDITION_KEEPS_DOWN"));
    s_CAR_CAPS(pyrt::xrefs::xget("memory.store_lines", "CAR_CAPS"));
    s_NUMBER_WORDS(pyrt::xrefs::xget("memory.store_lines", "NUMBER_WORDS"));
    s__SUBJECT_KEYS(pyrt::xrefs::xget("memory.store_lines", "_SUBJECT_KEYS"));
    s_PLURAL_PETS(pyrt::xrefs::xget("memory.store_lines", "PLURAL_PETS"));
    s_COUNT_WORDS(pyrt::xrefs::xget("memory.store_lines", "COUNT_WORDS"));
    s_A_SHORT_ANSWER(pyrt::xrefs::xget("memory.store_lines", "A_SHORT_ANSWER"));
    s_KEY_SLOT(pyrt::xrefs::xget("memory.store_lines", "KEY_SLOT"));
    s_CONCEPTS(pyrt::xrefs::xget("memory.store_lines", "CONCEPTS"));
    s_POSSESSIVE(pyrt::xrefs::xget("memory.store_lines", "POSSESSIVE"));
    s_QUOTE_LEADS(pyrt::xrefs::xget("memory.store_lines", "QUOTE_LEADS"));
    s_ASKED_LEADS(pyrt::xrefs::xget("memory.store_lines", "ASKED_LEADS"));
    s_NOTHING_STARTED(pyrt::xrefs::xget("memory.store_lines", "NOTHING_STARTED"));
    s_RESTRICT_OPENS(pyrt::xrefs::xget("memory.store_lines", "RESTRICT_OPENS"));
    s_CALL_LEADS(pyrt::xrefs::xget("memory.store_lines", "CALL_LEADS"));
    s_LENGTH(pyrt::xrefs::xget("memory.store_lines", "LENGTH"));
    s_WORD_NUMBERS(pyrt::xrefs::xget("memory.store_lines", "WORD_NUMBERS"));
    s_UNIT_SECONDS(pyrt::xrefs::xget("memory.store_lines", "UNIT_SECONDS"));
    s_BUDGET_FITS(pyrt::xrefs::xget("memory.store_lines", "BUDGET_FITS"));
    s_BUDGET_MORE(pyrt::xrefs::xget("memory.store_lines", "BUDGET_MORE"));
    s_BUDGET_TIGHT(pyrt::xrefs::xget("memory.store_lines", "BUDGET_TIGHT"));
    s_ANY_FIGURE(pyrt::xrefs::xget("memory.store_lines", "ANY_FIGURE"));
    s_HELD_THREAD(pyrt::xrefs::xget("memory.store_lines", "HELD_THREAD"));
    s_GRIEF_WORDS(pyrt::xrefs::xget("memory.store_lines", "GRIEF_WORDS"));
    s_CLOSE_PLAIN(pyrt::xrefs::xget("memory.store_lines", "CLOSE_PLAIN"));
    s_CLOSE_HEAVY(pyrt::xrefs::xget("memory.store_lines", "CLOSE_HEAVY"));
    s_CHANGE_PLAIN(pyrt::xrefs::xget("memory.store_lines", "CHANGE_PLAIN"));
    s_CHANGE_HEAVY(pyrt::xrefs::xget("memory.store_lines", "CHANGE_HEAVY"));
    s_CHECK_NOTHING_SAID(pyrt::xrefs::xget("memory.store_lines", "CHECK_NOTHING_SAID"));
    s_CHECK_A_FEELING(pyrt::xrefs::xget("memory.store_lines", "CHECK_A_FEELING"));
    s_CHECK_A_QUESTION(pyrt::xrefs::xget("memory.store_lines", "CHECK_A_QUESTION"));
    s_CHECK_PLAIN(pyrt::xrefs::xget("memory.store_lines", "CHECK_PLAIN"));
    s_CLOCK_SAID(pyrt::xrefs::xget("memory.store_lines", "CLOCK_SAID"));
    s_ROUTINE_SUBJECT(pyrt::xrefs::xget("memory.store_lines", "ROUTINE_SUBJECT"));
    s_ROUTINE_SAY_BACK(pyrt::xrefs::xget("memory.store_lines", "ROUTINE_SAY_BACK"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.MISSED_CATCH_UP"); }
    s_MISSED_CATCH_UP(stdlib_call("re", "compile", &[py_str("\\bwhat (?:have|had|did) i miss(?:ed)?\\b(?=\\s*(?:so far|then|here|yet|today|again)?\\s*[?.,!]*$)|\\b(?:have|did) i miss(?:ed)? anything\\b(?=\\s*[?.,!]*$)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.TOPIC_ASK"); }
    s_TOPIC_ASK(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\bwhat (?:were|are|was|did|do|have|had) we\\s+(?:talk|chat|discuss|mention|speak|spoke|say|said|cover)\\w*\\s*(?:about|of|on)\\b|\\bwhat (?:were|was|did|have|had) we\\s+(?:talk|chat|discuss|mention|speak|spoke|say|said|cover|do|done)\\w*(?=\\s*[?.!,]|\\s*$)|\\bwhat we\\s+(?:talk|chat|discuss|mention)\\w*\\s*(?:about|of|on)\\b|\\bwhat (?:were|are|was) (?:we|i|you) (?:talking|chatting|saying|on) about\\b|\\bwhat (?:were|are|was) (?:we|i|you) saying\\b|\\bwhere (?:were we|are we)(?=\\s*[?.!,]|\\s*$)|\\bwhere did we (?:leave|get to|stop)\\b|\\bwhat was i (?:saying|on about|asking|learning|studying|reading|researching|working on|up to)\\b|\\bwhat (?:was|is) (?:my|the|that) question\\b\\s*(?:again)?\\s*[?.!,]*$|\\bwhat question (?:did i (?:just )?ask|(?:was|is) (?:it|that))\\b|\\bwhat (?:was|were) (?:i|we|you) up to\\b|\\bwhat (?:am|was) i (?:learning|studying|reading|researching)\\b|\\bcatch me up\\b|\\brecap\\b|"), &gattr(&g_MISSED_CATCH_UP(), "pattern")), &py_str("|\\bremind me (?:what|where|how)\\b|\\bsum (?:up )?what we\\b|\\bwhat (?:did|have) we (?:talk|chat|say) about\\b|\\bwhat we (?:have|had) (?:covered|talked|discussed|done|said|been on)\\b|\\bwhat we were (?:on|up to)\\b|\\bwhat did i (?:just )?(?:ask|want to know)\\b(?![^.?!]*\\babout\\b(?!\\s+(?:me|it|that|this|them|us)\\b))|\\bwhat did i (?:just )?say\\b(?!\\s+(?:about|to|when|in|on|at|for|during|before|after|since|wrong)\\b)"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.ASK_WITHOUT_MY"); }
    s_ASK_WITHOUT_MY(stdlib_call("re", "compile", &[py_str("\\b(?:what|which) (?:pets?|dogs?|cats?|animals?) (?:do|did|have) i\\b|\\b(?:do|have|did) i (?:have|got) (?:any )?(?:pets?|dogs?|cats?|animals?|allergies|a budget)\\b|\\bam i allergic to\\b|\\b(?:anything|something) i(?:'m| am) allergic to\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.HOW_MANY_OF_MINE"); }
    s_HOW_MANY_OF_MINE(stdlib_call("re", "compile", &[py_str("\\bhow many (?:pets?|dogs?|pupp(?:y|ies)|cats?|kittens?|animals?|rabbits?|hamsters?|parrots?|fish|horses?|pon(?:y|ies)) (?:do|did) (?:i|we) (?:have|got|own)\\b|\\bhow many (?:pets?|dogs?|pupp(?:y|ies)|cats?|kittens?|animals?|rabbits?|hamsters?|parrots?|fish|horses?|pon(?:y|ies)) (?:have|had) (?:i|we) got\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.MEMORY_POINTER"); }
    s_MEMORY_POINTER(stdlib_call("re", "compile", &[py_str("\\b(?:again|remind me|did i (?:say|tell)|for a living|for work)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.PLAN_ASKS"); }
    s_PLAN_ASKS(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\bwhat (?:am i|are we) (?:planning|up to)\\b|\\bwhat (?:am i|are we) doing\\b(?!\\s+wrong)\\b|\\bwhat(?:'s| is|s)?\\s+(?:coming up|on for (?:me|us))\\b|\\bwhat (?:have|has) (?:i|we) got (?:coming up|on|planned|left)\\b|\\bwhat (?:have|has) (?:i|we) got to (?:do|sort|get|finish|arrange)\\b|\\bwhat (?:have|has) (?:i|we) got (?:in|on) (?:the|my|our) (?:diary|calendar)\\b|\\bwhat (?:have|has) (?:i|we) got\\s*[?.!,]*\\s*$|\\bwhat do (?:i|we) have (?:coming up|on|planned)\\b|\\banything (?:planned|coming up)\\b|\\banything on\\b"), &pyrt::xrefs::xget("memory.store_lines", "NOT_ON_A_MEDIUM")), &py_str("|\\bwhat am i looking forward to\\b|\\bwhat (?:was|were) (?:i|we) (?:planning|saving|saving up)\\b|\\bwhat (?:am i|are we|is) (?:saving|saving up)\\b|\\bwhat i (?:was|am) (?:planning|saving)\\b|\\bwhat(?:'s| is| was) my plan\\b")), &pyrt::xrefs::xget("memory.store_lines", "NOT_THE_PLAN")), &py_str("|\\bwhat was (?:the|my) plan\\b")), &pyrt::xrefs::xget("memory.store_lines", "NOT_THE_PLAN")), &py_str("|\\bwhat was i going to do\\b(?:\\s*then)?\\s*[?.!,]*$|\\bwhere (?:are|were|was|am) (?:we|i) (?:going|off|headed|heading|away)\\b|\\bwhen (?:are|were|was|am) (?:we|i) (?:off|going|away|moving|flying|travelling)\\b|\\b(?:which|what) (?:month|day|week|date) (?:are|am|is) (?:we|i)\\b|\\bremind me where\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.PLAN_MINE"); }
    s_PLAN_MINE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\bwhat (?:was|is) my plan\\b|\\bwhat my plan (?:was|is)\\b|\\bwhat (?:did|do) i say (?:my plan|i was planning)\\b|\\bwhat (?:are|were) (?:my|our) plans?\\b"), &pyrt::xrefs::xget("memory.store_lines", "NOT_THE_PLAN")), &py_str("|\\bwhat plans (?:do|did) (?:i|we) have\\b")), &pyrt::xrefs::xget("memory.store_lines", "NOT_THE_PLAN")), &py_str("|\\b(?:do|did) (?:i|we) have (?:any |some )?plans?\\b|\\b(?:do|did) (?:i|we) have anything (?:planned|coming up|on)\\b|\\bhave (?:i|we) got (?:any |some )?(?:plans?|anything (?:planned|coming up|on))\\b|\\b(?:is|are) there anything (?:planned|coming up|on) for (?:me|us)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.WHEN_ASKS"); }
    s_WHEN_ASKS(stdlib_call("re", "compile", &[py_str("\\bwhen (?:do|does|will|am|are) (?:i|we)\\s+(?:going to\\s+)?([a-z]+)")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.DAY_ASKS"); }
    s_DAY_ASKS(stdlib_call("re", "compile", &[py_str("\\b(?:what|which) (?:day|date) (?:do|does|will|am|are) (?:i|we)\\s+(?:going to\\s+)?([a-z]+)")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.WHERE_PLAN"); }
    s_WHERE_PLAN(stdlib_call("re", "compile", &[py_str("\\bwhere (?:are|were|was|am) (?:we|i) (?:going|off|headed|heading|away|moving|flying|travelling|traveling|driving)\\b|\\bwhere (?:we|i) (?:are|were|am|was) (?:going|off|away|moving|flying|travelling|traveling|driving)\\b|\\bwhere to again\\b|\\bwhere (?:are|were) we off to\\b|\\bremind me where\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.SAID_AS_A_SENTENCE"); }
    s_SAID_AS_A_SENTENCE(stdlib_call("re", "compile", &[py_str("^(?:we|i|we'?ve|i'?ve|we'?re|i'?m|we'?ll|i'?ll|we'?d|i'?d|we are|i am|we were|i was|my|our)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.WHEN_PLAN"); }
    s_WHEN_PLAN(stdlib_call("re", "compile", &[py_str("\\bwhen (?:are|were|was|am) (?:we|i) (?:off|going|away|moving|flying|travelling|traveling)\\b|\\b(?:which|what) (?:month|day|week|date|time of year) (?:are|am|is) (?:we|i)\\b|\\b(?:which|what) (?:month|day|week|date|time of year) (?:is|was) that\\b|\\bwhat date (?:are|am) (?:we|i)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.THAT_DATE"); }
    s_THAT_DATE(stdlib_call("re", "compile", &[py_str("\\b(?:which|what) (?:date|day) (?:is|was) that\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.CALENDAR_SAID"); }
    s_CALENDAR_SAID(stdlib_call("re", "compile", &[py_str("\\b\\d{1,2} (?:january|february|march|april|may|june|july|august|september|october|november|december)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.WEEKDAY_WORD"); }
    s_WEEKDAY_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.DAY_OF_MONTH"); }
    s_DAY_OF_MONTH(stdlib_call("re", "compile", &[py_str("\\b(?:on the |the )?(\\d{1,2}(?:st|nd|rd|th))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.WHEN_THING"); }
    s_WHEN_THING(stdlib_call("re", "compile", &[py_str("\\bwhen(?:'s| is| are| was| were)\\s+(?:the|my|our|his|her|their|your|a|an)\\s+([a-z][a-z'-]*)")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.WHEN_INVERTED"); }
    s_WHEN_INVERTED(stdlib_call("re", "compile", &[py_str("^(?:when|when's)\\s+(the|my|our|his|her|their|your|a|an)\\s+([a-z][a-z'-]*)\\s+(is|are|was|were)\\b(?:[\\s,]*(?:again|please|then))?$")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.WHEN_INVERTED_SUBJECT"); }
    s_WHEN_INVERTED_SUBJECT(stdlib_call("re", "compile", &[py_str("^when\\s+(i|we)\\s+((?:am|are|was|were|do|does|did|will|have|has)\\b.*)$")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.WHEN_THING_VERB"); }
    s_WHEN_THING_VERB(stdlib_call("re", "compile", &[py_str("\\bwhen (?:does|do|did|will|would|can|could)\\s+(?:the|my|our|his|her|their|your)\\s+([a-z][a-z'-]*)\\s+([a-z][a-z'-]*)")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.WHEN_END_ASK"); }
    s_WHEN_END_ASK(call_builtin("frozenset", &[mcall(&py_str("end ends finish finishes last lasts leave leaves return returns come\narrive arrives land lands close closes stop stops expire expires wrap"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.WHEN_START_ASK"); }
    s_WHEN_START_ASK(call_builtin("frozenset", &[mcall(&py_str("start starts begin begins open opens kick"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.WHEN_END_WORD"); }
    s_WHEN_END_WORD(call_builtin("frozenset", &[mcall(&py_str("end ends finish finishes last lasts over expire expires close closes wrap wrapped"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.TIME_ASKS"); }
    s_TIME_ASKS(stdlib_call("re", "compile", &[py_str("\\bwhat time (?:do|does|will|am|are) (?:i|we)\\s+(?:going to\\s+)?([a-z]+)")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.TIME_THING"); }
    s_TIME_THING(stdlib_call("re", "compile", &[py_str("\\bwhat time(?:'s| is| are| was| were)\\s+(?:the|my|our|his|her|their|your|a|an)\\s+([a-z][a-z'-]*)")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.DAY_THING"); }
    s_DAY_THING(stdlib_call("re", "compile", &[py_str("\\b(?:what|which) (?:day|date) (?:is|was|are|were)\\s+(?:the|my|our|his|her|their|your|a|an)\\s+([a-z][a-z'-]*)")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.DAY_IT"); }
    s_DAY_IT(stdlib_call("re", "compile", &[py_str("\\b(?:what|which) (?:day|date) (?:is|was) (?:that|this)\\b|\\b(?:what|which) (?:day|date) (?:is|was) (?:it|that)\\s+(?:on|at|for|again)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.TIME_IT"); }
    s_TIME_IT(stdlib_call("re", "compile", &[py_str("\\bwhat time (?:is|was|will be) (?:that|this)\\b|\\bwhat time (?:is|was) (?:it|that)\\s+(?:on|at|for|again)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.WHEN_VAGUE"); }
    s_WHEN_VAGUE(call_builtin("frozenset", &[mcall(&py_str("next last best first good right usual actual whole thing stuff one time"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.WHEN_QUALITY"); }
    s_WHEN_QUALITY(call_builtin("frozenset", &[mcall(&py_str("best good right usual actual"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.WHEN_VERB"); }
    s_WHEN_VERB(call_builtin("frozenset", &[mcall(&py_str("prep prepare get go leave book buy tell say send start take plant apply ask\ncall visit fly travel do make set put order renew sort pack move pay file check wrap post"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.POINTS_BACK"); }
    s_POINTS_BACK(call_builtin("frozenset", &[mcall(&py_str("it its them there that this these those one ones back again"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.ELIDED_ASK"); }
    s_ELIDED_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat (?:did|do) i (?:say|call|name) (?:it|him|her|them|they|he|she)(?: (?:was|were|is|are))?(?: (?:called|named))?\\b|\\b(?:and )?what (?:was|is) (?:it|that) again\\b|\\b(?:and )?(?:what|which) (?:was|is) (?:its |his |her |their |the )?name(?: again)?\\b|\\b(?:and )?what did i (?:call|name) (?:him|her|them)\\b|\\bwhat (?:was|is) that called again\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.ELIDED_AGAIN"); }
    s_ELIDED_AGAIN(stdlib_call("re", "compile", &[py_str("\\b(?:where|when) (?:was|is) (?:it|that|that one)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.ELIDED_WORDS"); }
    s_ELIDED_WORDS(call_builtin("frozenset", &[mcall(&py_str("\nand so but ok okay oh then well right what which where when was is did do i call called name\nsaid say it that this him her them they he she his hers its their the again to me you my of\nwere was that one\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.ELIDED_NOT_SURE"); }
    s_ELIDED_NOT_SURE(stdlib_call("re", "compile", &[py_str("\\bhaven't (?:told|mentioned)\\b|\\bdon't know (?:it|your)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.ELIDED_NAMES"); }
    s_ELIDED_NAMES(py_tuple(vec![py_tuple(vec![py_str("name"), stdlib_call("re", "compile", &[py_str("\\b(?:call|called|name|named)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("city"), stdlib_call("re", "compile", &[py_str("\\b(?:live|living|from|town|city|moved)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("job"), stdlib_call("re", "compile", &[py_str("\\b(?:work|working|job)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("age"), stdlib_call("re", "compile", &[py_str("\\b(?:old|age)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("budget"), stdlib_call("re", "compile", &[py_str("\\b(?:budget|spend|money|cost)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("allergy"), stdlib_call("re", "compile", &[py_str("\\ballerg\\w*\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("pets"), stdlib_call("re", "compile", &[py_str("\\b(?:pet|pets|dog|cat|animal)\\b")]).unwrap_or_else(|e| exc::raise(e))])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.COUNT_ANIMAL"); }
    s_COUNT_ANIMAL(stdlib_call("re", "compile", &[py_str("^\\s*(?:ok |okay |so |and |but |well |oh |quick |just )*how many\\s+(dogs?|cats?|puppies|puppy|kittens?|kitten|rabbits?|bunnies|bunny|hamsters?|fish|horses?|ponies|parrots?|pets?)\\b\\s*(?:do (?:i|we) have|have (?:i|we) got|did (?:i|we) say|do (?:i|we) own|have we|are there|(?:is|was|are|were) (?:that|it|this)(?: again| now)?|again|then|now)?\\s*[?.!]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.COUNT_BARE"); }
    s_COUNT_BARE(stdlib_call("re", "compile", &[py_str("^\\s*(?:ok |okay |so |and |but |well |oh |quick |just )*how many\\s+(?:do (?:i|we) have|have (?:i|we) got|do (?:i|we) own|are there|did (?:i|we) say|in total|altogether)?\\s*[?.!]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.WHAT_I_SAID"); }
    s_WHAT_I_SAID(stdlib_call("re", "compile", &[py_str("\\bwhat did i (?:just )?(?:say|tell you|mention)\\b|\\bwhat have i (?:said|mentioned)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.memory_tables.SAID_ASK"); }
    s_SAID_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat did i (?:just )?(?:say|tell you)(?: just)?(?: then| last| before| earlier| again)?\\s*\\??$|\\bwhat (?:was|is|were) (?:the |my )?(?:last|previous|most recent) (?:thing|bit|line|message|one) (?:that )?i (?:said|told you|mentioned)\\s*\\??$")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.memory_tables".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ADVICE_MODULES" => Some(g_ADVICE_MODULES()),
            "AGE_FIX_WORDS" => Some(g_AGE_FIX_WORDS()),
            "AMOUNT_WORD" => Some(g_AMOUNT_WORD()),
            "ANSWERS" => Some(g_ANSWERS()),
            "ANY_FIGURE" => Some(g_ANY_FIGURE()),
            "ASKED_LEADS" => Some(g_ASKED_LEADS()),
            "ASK_WITHOUT_MY" => Some(g_ASK_WITHOUT_MY()),
            "AT_TIME_VALUE" => Some(g_AT_TIME_VALUE()),
            "A_SHORT_ANSWER" => Some(g_A_SHORT_ANSWER()),
            "BAD_NEWS" => Some(g_BAD_NEWS()),
            "BARE_ECHO_ONLY" => Some(g_BARE_ECHO_ONLY()),
            "BARE_SLOTS" => Some(g_BARE_SLOTS()),
            "BUDGET_FITS" => Some(g_BUDGET_FITS()),
            "BUDGET_MORE" => Some(g_BUDGET_MORE()),
            "BUDGET_TIGHT" => Some(g_BUDGET_TIGHT()),
            "CALENDAR_SAID" => Some(g_CALENDAR_SAID()),
            "CALL_LEADS" => Some(g_CALL_LEADS()),
            "CAR_CAPS" => Some(g_CAR_CAPS()),
            "CAT_NAMES" => Some(g_CAT_NAMES()),
            "CHANGE_HEAVY" => Some(g_CHANGE_HEAVY()),
            "CHANGE_PLAIN" => Some(g_CHANGE_PLAIN()),
            "CHECK_A_FEELING" => Some(g_CHECK_A_FEELING()),
            "CHECK_A_QUESTION" => Some(g_CHECK_A_QUESTION()),
            "CHECK_NOTHING_SAID" => Some(g_CHECK_NOTHING_SAID()),
            "CHECK_PLAIN" => Some(g_CHECK_PLAIN()),
            "CHILD" => Some(g_CHILD()),
            "CHILD_OF" => Some(g_CHILD_OF()),
            "CHILD_RELATIONS" => Some(g_CHILD_RELATIONS()),
            "CLOCK_SAID" => Some(g_CLOCK_SAID()),
            "CLOSE_HEAVY" => Some(g_CLOSE_HEAVY()),
            "CLOSE_PLAIN" => Some(g_CLOSE_PLAIN()),
            "CONCEPTS" => Some(g_CONCEPTS()),
            "CONDITION_KEEPS_DOWN" => Some(g_CONDITION_KEEPS_DOWN()),
            "CONDITION_RULES_OUT" => Some(g_CONDITION_RULES_OUT()),
            "CORRECTED_AGAIN" => Some(g_CORRECTED_AGAIN()),
            "CORRECTED_AGAIN_WHO" => Some(g_CORRECTED_AGAIN_WHO()),
            "CORRECTED_SAY" => Some(g_CORRECTED_SAY()),
            "CORRECTION_BACK" => Some(g_CORRECTION_BACK()),
            "CORRECTION_SAY" => Some(g_CORRECTION_SAY()),
            "CORRECTION_SAY_DROP" => Some(g_CORRECTION_SAY_DROP()),
            "CORRECTION_SAY_PAIR" => Some(g_CORRECTION_SAY_PAIR()),
            "COUNT_ANIMAL" => Some(g_COUNT_ANIMAL()),
            "COUNT_BARE" => Some(g_COUNT_BARE()),
            "COUNT_WORD" => Some(g_COUNT_WORD()),
            "COUNT_WORDS" => Some(g_COUNT_WORDS()),
            "DAY_ASKS" => Some(g_DAY_ASKS()),
            "DAY_IT" => Some(g_DAY_IT()),
            "DAY_OF_MONTH" => Some(g_DAY_OF_MONTH()),
            "DAY_THING" => Some(g_DAY_THING()),
            "DENIAL_SAY" => Some(g_DENIAL_SAY()),
            "DENIAL_STEMS" => Some(g_DENIAL_STEMS()),
            "DENIAL_WORDS" => Some(g_DENIAL_WORDS()),
            "DIET_FOOD" => Some(g_DIET_FOOD()),
            "DIET_RULES_OUT" => Some(g_DIET_RULES_OUT()),
            "DOG_NAMES" => Some(g_DOG_NAMES()),
            "DROP_LISTS" => Some(g_DROP_LISTS()),
            "ELIDED_AGAIN" => Some(g_ELIDED_AGAIN()),
            "ELIDED_ASK" => Some(g_ELIDED_ASK()),
            "ELIDED_NAMES" => Some(g_ELIDED_NAMES()),
            "ELIDED_NOT_SURE" => Some(g_ELIDED_NOT_SURE()),
            "ELIDED_WORDS" => Some(g_ELIDED_WORDS()),
            "FACT_FAMILY" => Some(g_FACT_FAMILY()),
            "FAMILY_GROUP" => Some(g_FAMILY_GROUP()),
            "FEEDING_SAY" => Some(g_FEEDING_SAY()),
            "FIGURE_ANSWER_LEN" => Some(g_FIGURE_ANSWER_LEN()),
            "FIGURE_NUMBER" => Some(g_FIGURE_NUMBER()),
            "FIGURE_QUALIFIER" => Some(g_FIGURE_QUALIFIER()),
            "FIGURE_TIME" => Some(g_FIGURE_TIME()),
            "FIGURE_UNITS" => Some(g_FIGURE_UNITS()),
            "FOOD_PHRASES" => Some(g_FOOD_PHRASES()),
            "GRIEF_WORDS" => Some(g_GRIEF_WORDS()),
            "GROUP_OF" => Some(g_GROUP_OF()),
            "HAVE_ANY_WORDS" => Some(g_HAVE_ANY_WORDS()),
            "HELD_THREAD" => Some(g_HELD_THREAD()),
            "HER_WORD" => Some(g_HER_WORD()),
            "HOW_MANY_OF_MINE" => Some(g_HOW_MANY_OF_MINE()),
            "IN_LAW_AFTER_TEXT" => Some(g_IN_LAW_AFTER_TEXT()),
            "KEPT_THINGS" => Some(g_KEPT_THINGS()),
            "KEY_SLOT" => Some(g_KEY_SLOT()),
            "KIN" => Some(g_KIN()),
            "KIN_ASK" => Some(g_KIN_ASK()),
            "LENGTH" => Some(g_LENGTH()),
            "LIMITS" => Some(g_LIMITS()),
            "LIMIT_SAY" => Some(g_LIMIT_SAY()),
            "LOOK_BACK_FIGURE" => Some(g_LOOK_BACK_FIGURE()),
            "MEMORY_POINTER" => Some(g_MEMORY_POINTER()),
            "MISSED_CATCH_UP" => Some(g_MISSED_CATCH_UP()),
            "MONTH_ORDER" => Some(g_MONTH_ORDER()),
            "NAMED_BY" => Some(g_NAMED_BY()),
            "NAME_ADVICE" => Some(g_NAME_ADVICE()),
            "NAME_FIX_WORDS" => Some(g_NAME_FIX_WORDS()),
            "NAME_GIVEN_WORDS" => Some(g_NAME_GIVEN_WORDS()),
            "NOTHING_STARTED" => Some(g_NOTHING_STARTED()),
            "NOT_A_UNIT" => Some(g_NOT_A_UNIT()),
            "NOT_ON_A_MEDIUM" => Some(g_NOT_ON_A_MEDIUM()),
            "NOT_RETRACTED" => Some(g_NOT_RETRACTED()),
            "NOT_THE_PLAN" => Some(g_NOT_THE_PLAN()),
            "NO_PLANS_REPLY" => Some(g_NO_PLANS_REPLY()),
            "NUMBER_WORDS" => Some(g_NUMBER_WORDS()),
            "OTHER_KIND_SAY" => Some(g_OTHER_KIND_SAY()),
            "OTHER_SAID" => Some(g_OTHER_SAID()),
            "OWNED_BY_THE_VERB" => Some(g_OWNED_BY_THE_VERB()),
            "PART_HELD" => Some(g_PART_HELD()),
            "PART_KEYS" => Some(g_PART_KEYS()),
            "PERSON_GROUP" => Some(g_PERSON_GROUP()),
            "PET_AGE_FIX_WORDS" => Some(g_PET_AGE_FIX_WORDS()),
            "PET_RENAMED_SAY" => Some(g_PET_RENAMED_SAY()),
            "PET_WORDS" => Some(g_PET_WORDS()),
            "PLACE_NOT" => Some(g_PLACE_NOT()),
            "PLAN_ASKS" => Some(g_PLAN_ASKS()),
            "PLAN_MINE" => Some(g_PLAN_MINE()),
            "PLURAL_PEOPLE" => Some(g_PLURAL_PEOPLE()),
            "PLURAL_PETS" => Some(g_PLURAL_PETS()),
            "POINTS_BACK" => Some(g_POINTS_BACK()),
            "POSSESSIVE" => Some(g_POSSESSIVE()),
            "PREGNANT_OWN_SAY" => Some(g_PREGNANT_OWN_SAY()),
            "PREGNANT_SAY" => Some(g_PREGNANT_SAY()),
            "PREGNANT_WEEKS_SAY" => Some(g_PREGNANT_WEEKS_SAY()),
            "PRIORITY" => Some(g_PRIORITY()),
            "PRONOUN_FITS" => Some(g_PRONOUN_FITS()),
            "PRONOUN_PEOPLE" => Some(g_PRONOUN_PEOPLE()),
            "QUOTE_LEADS" => Some(g_QUOTE_LEADS()),
            "RECALL_HINTS" => Some(g_RECALL_HINTS()),
            "RELATION_ALIKE" => Some(g_RELATION_ALIKE()),
            "REPEAT_ADVERB" => Some(g_REPEAT_ADVERB()),
            "RESTATED_KEYS" => Some(g_RESTATED_KEYS()),
            "RESTRICT_OPENS" => Some(g_RESTRICT_OPENS()),
            "RESUME" => Some(g_RESUME()),
            "RETRACT_SAY" => Some(g_RETRACT_SAY()),
            "ROUTINE_SAY_BACK" => Some(g_ROUTINE_SAY_BACK()),
            "ROUTINE_SUBJECT" => Some(g_ROUTINE_SUBJECT()),
            "SAID_ASK" => Some(g_SAID_ASK()),
            "SAID_AS_A_SENTENCE" => Some(g_SAID_AS_A_SENTENCE()),
            "SAME_FACT_THEIR_OWN" => Some(g_SAME_FACT_THEIR_OWN()),
            "SELF" => Some(g_SELF()),
            "SLOT_KEY" => Some(g_SLOT_KEY()),
            "SMALL_NAMES" => Some(g_SMALL_NAMES()),
            "SPELLING_APART" => Some(g_SPELLING_APART()),
            "STATE_WAS_SAY" => Some(g_STATE_WAS_SAY()),
            "SWAPPABLE" => Some(g_SWAPPABLE()),
            "TAIL_WINDOW" => Some(g_TAIL_WINDOW()),
            "THAT_DATE" => Some(g_THAT_DATE()),
            "TIME_ASKS" => Some(g_TIME_ASKS()),
            "TIME_IT" => Some(g_TIME_IT()),
            "TIME_THING" => Some(g_TIME_THING()),
            "TOPIC_ASK" => Some(g_TOPIC_ASK()),
            "TURNED" => Some(g_TURNED()),
            "UNIT_FAMILY" => Some(g_UNIT_FAMILY()),
            "UNIT_SECONDS" => Some(g_UNIT_SECONDS()),
            "WEEK" => Some(g_WEEK()),
            "WEEKDAY_WORD" => Some(g_WEEKDAY_WORD()),
            "WHAT_I_SAID" => Some(g_WHAT_I_SAID()),
            "WHEN_ASKS" => Some(g_WHEN_ASKS()),
            "WHEN_END_ASK" => Some(g_WHEN_END_ASK()),
            "WHEN_END_WORD" => Some(g_WHEN_END_WORD()),
            "WHEN_INVERTED" => Some(g_WHEN_INVERTED()),
            "WHEN_INVERTED_SUBJECT" => Some(g_WHEN_INVERTED_SUBJECT()),
            "WHEN_PLAN" => Some(g_WHEN_PLAN()),
            "WHEN_QUALITY" => Some(g_WHEN_QUALITY()),
            "WHEN_START_ASK" => Some(g_WHEN_START_ASK()),
            "WHEN_THING" => Some(g_WHEN_THING()),
            "WHEN_THING_VERB" => Some(g_WHEN_THING_VERB()),
            "WHEN_VAGUE" => Some(g_WHEN_VAGUE()),
            "WHEN_VERB" => Some(g_WHEN_VERB()),
            "WHERE_ASK" => Some(g_WHERE_ASK()),
            "WHERE_PLAN" => Some(g_WHERE_PLAN()),
            "WORD_NUMBERS" => Some(g_WORD_NUMBERS()),
            "_ABOUT_WHO" => Some(g__ABOUT_WHO()),
            "_AMOUNTS" => Some(g__AMOUNTS()),
            "_AMOUNT_WORDS" => Some(g__AMOUNT_WORDS()),
            "_CONVERT" => Some(g__CONVERT()),
            "_COUNT_WORD" => Some(g__COUNT_WORD()),
            "_FACT_SLOT" => Some(g__FACT_SLOT()),
            "_SUBJECT_KEYS" => Some(g__SUBJECT_KEYS()),
            "_TENS_WORDS" => Some(g__TENS_WORDS()),
            "_UNIT_SAID" => Some(g__UNIT_SAID()),
            "_WEEKLY" => Some(g__WEEKLY()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ADVICE_MODULES" => s_ADVICE_MODULES(v),
            "AGE_FIX_WORDS" => s_AGE_FIX_WORDS(v),
            "AMOUNT_WORD" => s_AMOUNT_WORD(v),
            "ANSWERS" => s_ANSWERS(v),
            "ANY_FIGURE" => s_ANY_FIGURE(v),
            "ASKED_LEADS" => s_ASKED_LEADS(v),
            "ASK_WITHOUT_MY" => s_ASK_WITHOUT_MY(v),
            "AT_TIME_VALUE" => s_AT_TIME_VALUE(v),
            "A_SHORT_ANSWER" => s_A_SHORT_ANSWER(v),
            "BAD_NEWS" => s_BAD_NEWS(v),
            "BARE_ECHO_ONLY" => s_BARE_ECHO_ONLY(v),
            "BARE_SLOTS" => s_BARE_SLOTS(v),
            "BUDGET_FITS" => s_BUDGET_FITS(v),
            "BUDGET_MORE" => s_BUDGET_MORE(v),
            "BUDGET_TIGHT" => s_BUDGET_TIGHT(v),
            "CALENDAR_SAID" => s_CALENDAR_SAID(v),
            "CALL_LEADS" => s_CALL_LEADS(v),
            "CAR_CAPS" => s_CAR_CAPS(v),
            "CAT_NAMES" => s_CAT_NAMES(v),
            "CHANGE_HEAVY" => s_CHANGE_HEAVY(v),
            "CHANGE_PLAIN" => s_CHANGE_PLAIN(v),
            "CHECK_A_FEELING" => s_CHECK_A_FEELING(v),
            "CHECK_A_QUESTION" => s_CHECK_A_QUESTION(v),
            "CHECK_NOTHING_SAID" => s_CHECK_NOTHING_SAID(v),
            "CHECK_PLAIN" => s_CHECK_PLAIN(v),
            "CHILD" => s_CHILD(v),
            "CHILD_OF" => s_CHILD_OF(v),
            "CHILD_RELATIONS" => s_CHILD_RELATIONS(v),
            "CLOCK_SAID" => s_CLOCK_SAID(v),
            "CLOSE_HEAVY" => s_CLOSE_HEAVY(v),
            "CLOSE_PLAIN" => s_CLOSE_PLAIN(v),
            "CONCEPTS" => s_CONCEPTS(v),
            "CONDITION_KEEPS_DOWN" => s_CONDITION_KEEPS_DOWN(v),
            "CONDITION_RULES_OUT" => s_CONDITION_RULES_OUT(v),
            "CORRECTED_AGAIN" => s_CORRECTED_AGAIN(v),
            "CORRECTED_AGAIN_WHO" => s_CORRECTED_AGAIN_WHO(v),
            "CORRECTED_SAY" => s_CORRECTED_SAY(v),
            "CORRECTION_BACK" => s_CORRECTION_BACK(v),
            "CORRECTION_SAY" => s_CORRECTION_SAY(v),
            "CORRECTION_SAY_DROP" => s_CORRECTION_SAY_DROP(v),
            "CORRECTION_SAY_PAIR" => s_CORRECTION_SAY_PAIR(v),
            "COUNT_ANIMAL" => s_COUNT_ANIMAL(v),
            "COUNT_BARE" => s_COUNT_BARE(v),
            "COUNT_WORD" => s_COUNT_WORD(v),
            "COUNT_WORDS" => s_COUNT_WORDS(v),
            "DAY_ASKS" => s_DAY_ASKS(v),
            "DAY_IT" => s_DAY_IT(v),
            "DAY_OF_MONTH" => s_DAY_OF_MONTH(v),
            "DAY_THING" => s_DAY_THING(v),
            "DENIAL_SAY" => s_DENIAL_SAY(v),
            "DENIAL_STEMS" => s_DENIAL_STEMS(v),
            "DENIAL_WORDS" => s_DENIAL_WORDS(v),
            "DIET_FOOD" => s_DIET_FOOD(v),
            "DIET_RULES_OUT" => s_DIET_RULES_OUT(v),
            "DOG_NAMES" => s_DOG_NAMES(v),
            "DROP_LISTS" => s_DROP_LISTS(v),
            "ELIDED_AGAIN" => s_ELIDED_AGAIN(v),
            "ELIDED_ASK" => s_ELIDED_ASK(v),
            "ELIDED_NAMES" => s_ELIDED_NAMES(v),
            "ELIDED_NOT_SURE" => s_ELIDED_NOT_SURE(v),
            "ELIDED_WORDS" => s_ELIDED_WORDS(v),
            "FACT_FAMILY" => s_FACT_FAMILY(v),
            "FAMILY_GROUP" => s_FAMILY_GROUP(v),
            "FEEDING_SAY" => s_FEEDING_SAY(v),
            "FIGURE_ANSWER_LEN" => s_FIGURE_ANSWER_LEN(v),
            "FIGURE_NUMBER" => s_FIGURE_NUMBER(v),
            "FIGURE_QUALIFIER" => s_FIGURE_QUALIFIER(v),
            "FIGURE_TIME" => s_FIGURE_TIME(v),
            "FIGURE_UNITS" => s_FIGURE_UNITS(v),
            "FOOD_PHRASES" => s_FOOD_PHRASES(v),
            "GRIEF_WORDS" => s_GRIEF_WORDS(v),
            "GROUP_OF" => s_GROUP_OF(v),
            "HAVE_ANY_WORDS" => s_HAVE_ANY_WORDS(v),
            "HELD_THREAD" => s_HELD_THREAD(v),
            "HER_WORD" => s_HER_WORD(v),
            "HOW_MANY_OF_MINE" => s_HOW_MANY_OF_MINE(v),
            "IN_LAW_AFTER_TEXT" => s_IN_LAW_AFTER_TEXT(v),
            "KEPT_THINGS" => s_KEPT_THINGS(v),
            "KEY_SLOT" => s_KEY_SLOT(v),
            "KIN" => s_KIN(v),
            "KIN_ASK" => s_KIN_ASK(v),
            "LENGTH" => s_LENGTH(v),
            "LIMITS" => s_LIMITS(v),
            "LIMIT_SAY" => s_LIMIT_SAY(v),
            "LOOK_BACK_FIGURE" => s_LOOK_BACK_FIGURE(v),
            "MEMORY_POINTER" => s_MEMORY_POINTER(v),
            "MISSED_CATCH_UP" => s_MISSED_CATCH_UP(v),
            "MONTH_ORDER" => s_MONTH_ORDER(v),
            "NAMED_BY" => s_NAMED_BY(v),
            "NAME_ADVICE" => s_NAME_ADVICE(v),
            "NAME_FIX_WORDS" => s_NAME_FIX_WORDS(v),
            "NAME_GIVEN_WORDS" => s_NAME_GIVEN_WORDS(v),
            "NOTHING_STARTED" => s_NOTHING_STARTED(v),
            "NOT_A_UNIT" => s_NOT_A_UNIT(v),
            "NOT_ON_A_MEDIUM" => s_NOT_ON_A_MEDIUM(v),
            "NOT_RETRACTED" => s_NOT_RETRACTED(v),
            "NOT_THE_PLAN" => s_NOT_THE_PLAN(v),
            "NO_PLANS_REPLY" => s_NO_PLANS_REPLY(v),
            "NUMBER_WORDS" => s_NUMBER_WORDS(v),
            "OTHER_KIND_SAY" => s_OTHER_KIND_SAY(v),
            "OTHER_SAID" => s_OTHER_SAID(v),
            "OWNED_BY_THE_VERB" => s_OWNED_BY_THE_VERB(v),
            "PART_HELD" => s_PART_HELD(v),
            "PART_KEYS" => s_PART_KEYS(v),
            "PERSON_GROUP" => s_PERSON_GROUP(v),
            "PET_AGE_FIX_WORDS" => s_PET_AGE_FIX_WORDS(v),
            "PET_RENAMED_SAY" => s_PET_RENAMED_SAY(v),
            "PET_WORDS" => s_PET_WORDS(v),
            "PLACE_NOT" => s_PLACE_NOT(v),
            "PLAN_ASKS" => s_PLAN_ASKS(v),
            "PLAN_MINE" => s_PLAN_MINE(v),
            "PLURAL_PEOPLE" => s_PLURAL_PEOPLE(v),
            "PLURAL_PETS" => s_PLURAL_PETS(v),
            "POINTS_BACK" => s_POINTS_BACK(v),
            "POSSESSIVE" => s_POSSESSIVE(v),
            "PREGNANT_OWN_SAY" => s_PREGNANT_OWN_SAY(v),
            "PREGNANT_SAY" => s_PREGNANT_SAY(v),
            "PREGNANT_WEEKS_SAY" => s_PREGNANT_WEEKS_SAY(v),
            "PRIORITY" => s_PRIORITY(v),
            "PRONOUN_FITS" => s_PRONOUN_FITS(v),
            "PRONOUN_PEOPLE" => s_PRONOUN_PEOPLE(v),
            "QUOTE_LEADS" => s_QUOTE_LEADS(v),
            "RECALL_HINTS" => s_RECALL_HINTS(v),
            "RELATION_ALIKE" => s_RELATION_ALIKE(v),
            "REPEAT_ADVERB" => s_REPEAT_ADVERB(v),
            "RESTATED_KEYS" => s_RESTATED_KEYS(v),
            "RESTRICT_OPENS" => s_RESTRICT_OPENS(v),
            "RESUME" => s_RESUME(v),
            "RETRACT_SAY" => s_RETRACT_SAY(v),
            "ROUTINE_SAY_BACK" => s_ROUTINE_SAY_BACK(v),
            "ROUTINE_SUBJECT" => s_ROUTINE_SUBJECT(v),
            "SAID_ASK" => s_SAID_ASK(v),
            "SAID_AS_A_SENTENCE" => s_SAID_AS_A_SENTENCE(v),
            "SAME_FACT_THEIR_OWN" => s_SAME_FACT_THEIR_OWN(v),
            "SELF" => s_SELF(v),
            "SLOT_KEY" => s_SLOT_KEY(v),
            "SMALL_NAMES" => s_SMALL_NAMES(v),
            "SPELLING_APART" => s_SPELLING_APART(v),
            "STATE_WAS_SAY" => s_STATE_WAS_SAY(v),
            "SWAPPABLE" => s_SWAPPABLE(v),
            "TAIL_WINDOW" => s_TAIL_WINDOW(v),
            "THAT_DATE" => s_THAT_DATE(v),
            "TIME_ASKS" => s_TIME_ASKS(v),
            "TIME_IT" => s_TIME_IT(v),
            "TIME_THING" => s_TIME_THING(v),
            "TOPIC_ASK" => s_TOPIC_ASK(v),
            "TURNED" => s_TURNED(v),
            "UNIT_FAMILY" => s_UNIT_FAMILY(v),
            "UNIT_SECONDS" => s_UNIT_SECONDS(v),
            "WEEK" => s_WEEK(v),
            "WEEKDAY_WORD" => s_WEEKDAY_WORD(v),
            "WHAT_I_SAID" => s_WHAT_I_SAID(v),
            "WHEN_ASKS" => s_WHEN_ASKS(v),
            "WHEN_END_ASK" => s_WHEN_END_ASK(v),
            "WHEN_END_WORD" => s_WHEN_END_WORD(v),
            "WHEN_INVERTED" => s_WHEN_INVERTED(v),
            "WHEN_INVERTED_SUBJECT" => s_WHEN_INVERTED_SUBJECT(v),
            "WHEN_PLAN" => s_WHEN_PLAN(v),
            "WHEN_QUALITY" => s_WHEN_QUALITY(v),
            "WHEN_START_ASK" => s_WHEN_START_ASK(v),
            "WHEN_THING" => s_WHEN_THING(v),
            "WHEN_THING_VERB" => s_WHEN_THING_VERB(v),
            "WHEN_VAGUE" => s_WHEN_VAGUE(v),
            "WHEN_VERB" => s_WHEN_VERB(v),
            "WHERE_ASK" => s_WHERE_ASK(v),
            "WHERE_PLAN" => s_WHERE_PLAN(v),
            "WORD_NUMBERS" => s_WORD_NUMBERS(v),
            "_ABOUT_WHO" => s__ABOUT_WHO(v),
            "_AMOUNTS" => s__AMOUNTS(v),
            "_AMOUNT_WORDS" => s__AMOUNT_WORDS(v),
            "_CONVERT" => s__CONVERT(v),
            "_COUNT_WORD" => s__COUNT_WORD(v),
            "_FACT_SLOT" => s__FACT_SLOT(v),
            "_SUBJECT_KEYS" => s__SUBJECT_KEYS(v),
            "_TENS_WORDS" => s__TENS_WORDS(v),
            "_UNIT_SAID" => s__UNIT_SAID(v),
            "_WEEKLY" => s__WEEKLY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.memory_tables", module_obj);
    pyrt::xrefs::reg_access("modules.memory_tables", "ADVICE_MODULES", g_ADVICE_MODULES, s_ADVICE_MODULES);
    pyrt::xrefs::reg_access("modules.memory_tables", "AGE_FIX_WORDS", g_AGE_FIX_WORDS, s_AGE_FIX_WORDS);
    pyrt::xrefs::reg_access("modules.memory_tables", "AMOUNT_WORD", g_AMOUNT_WORD, s_AMOUNT_WORD);
    pyrt::xrefs::reg_access("modules.memory_tables", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("modules.memory_tables", "ANY_FIGURE", g_ANY_FIGURE, s_ANY_FIGURE);
    pyrt::xrefs::reg_access("modules.memory_tables", "ASKED_LEADS", g_ASKED_LEADS, s_ASKED_LEADS);
    pyrt::xrefs::reg_access("modules.memory_tables", "ASK_WITHOUT_MY", g_ASK_WITHOUT_MY, s_ASK_WITHOUT_MY);
    pyrt::xrefs::reg_access("modules.memory_tables", "AT_TIME_VALUE", g_AT_TIME_VALUE, s_AT_TIME_VALUE);
    pyrt::xrefs::reg_access("modules.memory_tables", "A_SHORT_ANSWER", g_A_SHORT_ANSWER, s_A_SHORT_ANSWER);
    pyrt::xrefs::reg_access("modules.memory_tables", "BAD_NEWS", g_BAD_NEWS, s_BAD_NEWS);
    pyrt::xrefs::reg_access("modules.memory_tables", "BARE_ECHO_ONLY", g_BARE_ECHO_ONLY, s_BARE_ECHO_ONLY);
    pyrt::xrefs::reg_access("modules.memory_tables", "BARE_SLOTS", g_BARE_SLOTS, s_BARE_SLOTS);
    pyrt::xrefs::reg_access("modules.memory_tables", "BUDGET_FITS", g_BUDGET_FITS, s_BUDGET_FITS);
    pyrt::xrefs::reg_access("modules.memory_tables", "BUDGET_MORE", g_BUDGET_MORE, s_BUDGET_MORE);
    pyrt::xrefs::reg_access("modules.memory_tables", "BUDGET_TIGHT", g_BUDGET_TIGHT, s_BUDGET_TIGHT);
    pyrt::xrefs::reg_access("modules.memory_tables", "CALENDAR_SAID", g_CALENDAR_SAID, s_CALENDAR_SAID);
    pyrt::xrefs::reg_access("modules.memory_tables", "CALL_LEADS", g_CALL_LEADS, s_CALL_LEADS);
    pyrt::xrefs::reg_access("modules.memory_tables", "CAR_CAPS", g_CAR_CAPS, s_CAR_CAPS);
    pyrt::xrefs::reg_access("modules.memory_tables", "CAT_NAMES", g_CAT_NAMES, s_CAT_NAMES);
    pyrt::xrefs::reg_access("modules.memory_tables", "CHANGE_HEAVY", g_CHANGE_HEAVY, s_CHANGE_HEAVY);
    pyrt::xrefs::reg_access("modules.memory_tables", "CHANGE_PLAIN", g_CHANGE_PLAIN, s_CHANGE_PLAIN);
    pyrt::xrefs::reg_access("modules.memory_tables", "CHECK_A_FEELING", g_CHECK_A_FEELING, s_CHECK_A_FEELING);
    pyrt::xrefs::reg_access("modules.memory_tables", "CHECK_A_QUESTION", g_CHECK_A_QUESTION, s_CHECK_A_QUESTION);
    pyrt::xrefs::reg_access("modules.memory_tables", "CHECK_NOTHING_SAID", g_CHECK_NOTHING_SAID, s_CHECK_NOTHING_SAID);
    pyrt::xrefs::reg_access("modules.memory_tables", "CHECK_PLAIN", g_CHECK_PLAIN, s_CHECK_PLAIN);
    pyrt::xrefs::reg_access("modules.memory_tables", "CHILD", g_CHILD, s_CHILD);
    pyrt::xrefs::reg_access("modules.memory_tables", "CHILD_OF", g_CHILD_OF, s_CHILD_OF);
    pyrt::xrefs::reg_access("modules.memory_tables", "CHILD_RELATIONS", g_CHILD_RELATIONS, s_CHILD_RELATIONS);
    pyrt::xrefs::reg_access("modules.memory_tables", "CLOCK_SAID", g_CLOCK_SAID, s_CLOCK_SAID);
    pyrt::xrefs::reg_access("modules.memory_tables", "CLOSE_HEAVY", g_CLOSE_HEAVY, s_CLOSE_HEAVY);
    pyrt::xrefs::reg_access("modules.memory_tables", "CLOSE_PLAIN", g_CLOSE_PLAIN, s_CLOSE_PLAIN);
    pyrt::xrefs::reg_access("modules.memory_tables", "CONCEPTS", g_CONCEPTS, s_CONCEPTS);
    pyrt::xrefs::reg_access("modules.memory_tables", "CONDITION_KEEPS_DOWN", g_CONDITION_KEEPS_DOWN, s_CONDITION_KEEPS_DOWN);
    pyrt::xrefs::reg_access("modules.memory_tables", "CONDITION_RULES_OUT", g_CONDITION_RULES_OUT, s_CONDITION_RULES_OUT);
    pyrt::xrefs::reg_access("modules.memory_tables", "CORRECTED_AGAIN", g_CORRECTED_AGAIN, s_CORRECTED_AGAIN);
    pyrt::xrefs::reg_access("modules.memory_tables", "CORRECTED_AGAIN_WHO", g_CORRECTED_AGAIN_WHO, s_CORRECTED_AGAIN_WHO);
    pyrt::xrefs::reg_access("modules.memory_tables", "CORRECTED_SAY", g_CORRECTED_SAY, s_CORRECTED_SAY);
    pyrt::xrefs::reg_access("modules.memory_tables", "CORRECTION_BACK", g_CORRECTION_BACK, s_CORRECTION_BACK);
    pyrt::xrefs::reg_access("modules.memory_tables", "CORRECTION_SAY", g_CORRECTION_SAY, s_CORRECTION_SAY);
    pyrt::xrefs::reg_access("modules.memory_tables", "CORRECTION_SAY_DROP", g_CORRECTION_SAY_DROP, s_CORRECTION_SAY_DROP);
    pyrt::xrefs::reg_access("modules.memory_tables", "CORRECTION_SAY_PAIR", g_CORRECTION_SAY_PAIR, s_CORRECTION_SAY_PAIR);
    pyrt::xrefs::reg_access("modules.memory_tables", "COUNT_ANIMAL", g_COUNT_ANIMAL, s_COUNT_ANIMAL);
    pyrt::xrefs::reg_access("modules.memory_tables", "COUNT_BARE", g_COUNT_BARE, s_COUNT_BARE);
    pyrt::xrefs::reg_access("modules.memory_tables", "COUNT_WORD", g_COUNT_WORD, s_COUNT_WORD);
    pyrt::xrefs::reg_access("modules.memory_tables", "COUNT_WORDS", g_COUNT_WORDS, s_COUNT_WORDS);
    pyrt::xrefs::reg_access("modules.memory_tables", "DAY_ASKS", g_DAY_ASKS, s_DAY_ASKS);
    pyrt::xrefs::reg_access("modules.memory_tables", "DAY_IT", g_DAY_IT, s_DAY_IT);
    pyrt::xrefs::reg_access("modules.memory_tables", "DAY_OF_MONTH", g_DAY_OF_MONTH, s_DAY_OF_MONTH);
    pyrt::xrefs::reg_access("modules.memory_tables", "DAY_THING", g_DAY_THING, s_DAY_THING);
    pyrt::xrefs::reg_access("modules.memory_tables", "DENIAL_SAY", g_DENIAL_SAY, s_DENIAL_SAY);
    pyrt::xrefs::reg_access("modules.memory_tables", "DENIAL_STEMS", g_DENIAL_STEMS, s_DENIAL_STEMS);
    pyrt::xrefs::reg_access("modules.memory_tables", "DENIAL_WORDS", g_DENIAL_WORDS, s_DENIAL_WORDS);
    pyrt::xrefs::reg_access("modules.memory_tables", "DIET_FOOD", g_DIET_FOOD, s_DIET_FOOD);
    pyrt::xrefs::reg_access("modules.memory_tables", "DIET_RULES_OUT", g_DIET_RULES_OUT, s_DIET_RULES_OUT);
    pyrt::xrefs::reg_access("modules.memory_tables", "DOG_NAMES", g_DOG_NAMES, s_DOG_NAMES);
    pyrt::xrefs::reg_access("modules.memory_tables", "DROP_LISTS", g_DROP_LISTS, s_DROP_LISTS);
    pyrt::xrefs::reg_access("modules.memory_tables", "ELIDED_AGAIN", g_ELIDED_AGAIN, s_ELIDED_AGAIN);
    pyrt::xrefs::reg_access("modules.memory_tables", "ELIDED_ASK", g_ELIDED_ASK, s_ELIDED_ASK);
    pyrt::xrefs::reg_access("modules.memory_tables", "ELIDED_NAMES", g_ELIDED_NAMES, s_ELIDED_NAMES);
    pyrt::xrefs::reg_access("modules.memory_tables", "ELIDED_NOT_SURE", g_ELIDED_NOT_SURE, s_ELIDED_NOT_SURE);
    pyrt::xrefs::reg_access("modules.memory_tables", "ELIDED_WORDS", g_ELIDED_WORDS, s_ELIDED_WORDS);
    pyrt::xrefs::reg_access("modules.memory_tables", "FACT_FAMILY", g_FACT_FAMILY, s_FACT_FAMILY);
    pyrt::xrefs::reg_access("modules.memory_tables", "FAMILY_GROUP", g_FAMILY_GROUP, s_FAMILY_GROUP);
    pyrt::xrefs::reg_access("modules.memory_tables", "FEEDING_SAY", g_FEEDING_SAY, s_FEEDING_SAY);
    pyrt::xrefs::reg_access("modules.memory_tables", "FIGURE_ANSWER_LEN", g_FIGURE_ANSWER_LEN, s_FIGURE_ANSWER_LEN);
    pyrt::xrefs::reg_access("modules.memory_tables", "FIGURE_NUMBER", g_FIGURE_NUMBER, s_FIGURE_NUMBER);
    pyrt::xrefs::reg_access("modules.memory_tables", "FIGURE_QUALIFIER", g_FIGURE_QUALIFIER, s_FIGURE_QUALIFIER);
    pyrt::xrefs::reg_access("modules.memory_tables", "FIGURE_TIME", g_FIGURE_TIME, s_FIGURE_TIME);
    pyrt::xrefs::reg_access("modules.memory_tables", "FIGURE_UNITS", g_FIGURE_UNITS, s_FIGURE_UNITS);
    pyrt::xrefs::reg_access("modules.memory_tables", "FOOD_PHRASES", g_FOOD_PHRASES, s_FOOD_PHRASES);
    pyrt::xrefs::reg_access("modules.memory_tables", "GRIEF_WORDS", g_GRIEF_WORDS, s_GRIEF_WORDS);
    pyrt::xrefs::reg_access("modules.memory_tables", "GROUP_OF", g_GROUP_OF, s_GROUP_OF);
    pyrt::xrefs::reg_access("modules.memory_tables", "HAVE_ANY_WORDS", g_HAVE_ANY_WORDS, s_HAVE_ANY_WORDS);
    pyrt::xrefs::reg_access("modules.memory_tables", "HELD_THREAD", g_HELD_THREAD, s_HELD_THREAD);
    pyrt::xrefs::reg_access("modules.memory_tables", "HER_WORD", g_HER_WORD, s_HER_WORD);
    pyrt::xrefs::reg_access("modules.memory_tables", "HOW_MANY_OF_MINE", g_HOW_MANY_OF_MINE, s_HOW_MANY_OF_MINE);
    pyrt::xrefs::reg_access("modules.memory_tables", "IN_LAW_AFTER_TEXT", g_IN_LAW_AFTER_TEXT, s_IN_LAW_AFTER_TEXT);
    pyrt::xrefs::reg_access("modules.memory_tables", "KEPT_THINGS", g_KEPT_THINGS, s_KEPT_THINGS);
    pyrt::xrefs::reg_access("modules.memory_tables", "KEY_SLOT", g_KEY_SLOT, s_KEY_SLOT);
    pyrt::xrefs::reg_access("modules.memory_tables", "KIN", g_KIN, s_KIN);
    pyrt::xrefs::reg_access("modules.memory_tables", "KIN_ASK", g_KIN_ASK, s_KIN_ASK);
    pyrt::xrefs::reg_access("modules.memory_tables", "LENGTH", g_LENGTH, s_LENGTH);
    pyrt::xrefs::reg_access("modules.memory_tables", "LIMITS", g_LIMITS, s_LIMITS);
    pyrt::xrefs::reg_access("modules.memory_tables", "LIMIT_SAY", g_LIMIT_SAY, s_LIMIT_SAY);
    pyrt::xrefs::reg_access("modules.memory_tables", "LOOK_BACK_FIGURE", g_LOOK_BACK_FIGURE, s_LOOK_BACK_FIGURE);
    pyrt::xrefs::reg_access("modules.memory_tables", "MEMORY_POINTER", g_MEMORY_POINTER, s_MEMORY_POINTER);
    pyrt::xrefs::reg_access("modules.memory_tables", "MISSED_CATCH_UP", g_MISSED_CATCH_UP, s_MISSED_CATCH_UP);
    pyrt::xrefs::reg_access("modules.memory_tables", "MONTH_ORDER", g_MONTH_ORDER, s_MONTH_ORDER);
    pyrt::xrefs::reg_access("modules.memory_tables", "NAMED_BY", g_NAMED_BY, s_NAMED_BY);
    pyrt::xrefs::reg_access("modules.memory_tables", "NAME_ADVICE", g_NAME_ADVICE, s_NAME_ADVICE);
    pyrt::xrefs::reg_access("modules.memory_tables", "NAME_FIX_WORDS", g_NAME_FIX_WORDS, s_NAME_FIX_WORDS);
    pyrt::xrefs::reg_access("modules.memory_tables", "NAME_GIVEN_WORDS", g_NAME_GIVEN_WORDS, s_NAME_GIVEN_WORDS);
    pyrt::xrefs::reg_access("modules.memory_tables", "NOTHING_STARTED", g_NOTHING_STARTED, s_NOTHING_STARTED);
    pyrt::xrefs::reg_access("modules.memory_tables", "NOT_A_UNIT", g_NOT_A_UNIT, s_NOT_A_UNIT);
    pyrt::xrefs::reg_access("modules.memory_tables", "NOT_ON_A_MEDIUM", g_NOT_ON_A_MEDIUM, s_NOT_ON_A_MEDIUM);
    pyrt::xrefs::reg_access("modules.memory_tables", "NOT_RETRACTED", g_NOT_RETRACTED, s_NOT_RETRACTED);
    pyrt::xrefs::reg_access("modules.memory_tables", "NOT_THE_PLAN", g_NOT_THE_PLAN, s_NOT_THE_PLAN);
    pyrt::xrefs::reg_access("modules.memory_tables", "NO_PLANS_REPLY", g_NO_PLANS_REPLY, s_NO_PLANS_REPLY);
    pyrt::xrefs::reg_access("modules.memory_tables", "NUMBER_WORDS", g_NUMBER_WORDS, s_NUMBER_WORDS);
    pyrt::xrefs::reg_access("modules.memory_tables", "OTHER_KIND_SAY", g_OTHER_KIND_SAY, s_OTHER_KIND_SAY);
    pyrt::xrefs::reg_access("modules.memory_tables", "OTHER_SAID", g_OTHER_SAID, s_OTHER_SAID);
    pyrt::xrefs::reg_access("modules.memory_tables", "OWNED_BY_THE_VERB", g_OWNED_BY_THE_VERB, s_OWNED_BY_THE_VERB);
    pyrt::xrefs::reg_access("modules.memory_tables", "PART_HELD", g_PART_HELD, s_PART_HELD);
    pyrt::xrefs::reg_access("modules.memory_tables", "PART_KEYS", g_PART_KEYS, s_PART_KEYS);
    pyrt::xrefs::reg_access("modules.memory_tables", "PERSON_GROUP", g_PERSON_GROUP, s_PERSON_GROUP);
    pyrt::xrefs::reg_access("modules.memory_tables", "PET_AGE_FIX_WORDS", g_PET_AGE_FIX_WORDS, s_PET_AGE_FIX_WORDS);
    pyrt::xrefs::reg_access("modules.memory_tables", "PET_RENAMED_SAY", g_PET_RENAMED_SAY, s_PET_RENAMED_SAY);
    pyrt::xrefs::reg_access("modules.memory_tables", "PET_WORDS", g_PET_WORDS, s_PET_WORDS);
    pyrt::xrefs::reg_access("modules.memory_tables", "PLACE_NOT", g_PLACE_NOT, s_PLACE_NOT);
    pyrt::xrefs::reg_access("modules.memory_tables", "PLAN_ASKS", g_PLAN_ASKS, s_PLAN_ASKS);
    pyrt::xrefs::reg_access("modules.memory_tables", "PLAN_MINE", g_PLAN_MINE, s_PLAN_MINE);
    pyrt::xrefs::reg_access("modules.memory_tables", "PLURAL_PEOPLE", g_PLURAL_PEOPLE, s_PLURAL_PEOPLE);
    pyrt::xrefs::reg_access("modules.memory_tables", "PLURAL_PETS", g_PLURAL_PETS, s_PLURAL_PETS);
    pyrt::xrefs::reg_access("modules.memory_tables", "POINTS_BACK", g_POINTS_BACK, s_POINTS_BACK);
    pyrt::xrefs::reg_access("modules.memory_tables", "POSSESSIVE", g_POSSESSIVE, s_POSSESSIVE);
    pyrt::xrefs::reg_access("modules.memory_tables", "PREGNANT_OWN_SAY", g_PREGNANT_OWN_SAY, s_PREGNANT_OWN_SAY);
    pyrt::xrefs::reg_access("modules.memory_tables", "PREGNANT_SAY", g_PREGNANT_SAY, s_PREGNANT_SAY);
    pyrt::xrefs::reg_access("modules.memory_tables", "PREGNANT_WEEKS_SAY", g_PREGNANT_WEEKS_SAY, s_PREGNANT_WEEKS_SAY);
    pyrt::xrefs::reg_access("modules.memory_tables", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.memory_tables", "PRONOUN_FITS", g_PRONOUN_FITS, s_PRONOUN_FITS);
    pyrt::xrefs::reg_access("modules.memory_tables", "PRONOUN_PEOPLE", g_PRONOUN_PEOPLE, s_PRONOUN_PEOPLE);
    pyrt::xrefs::reg_access("modules.memory_tables", "QUOTE_LEADS", g_QUOTE_LEADS, s_QUOTE_LEADS);
    pyrt::xrefs::reg_access("modules.memory_tables", "RECALL_HINTS", g_RECALL_HINTS, s_RECALL_HINTS);
    pyrt::xrefs::reg_access("modules.memory_tables", "RELATION_ALIKE", g_RELATION_ALIKE, s_RELATION_ALIKE);
    pyrt::xrefs::reg_access("modules.memory_tables", "REPEAT_ADVERB", g_REPEAT_ADVERB, s_REPEAT_ADVERB);
    pyrt::xrefs::reg_access("modules.memory_tables", "RESTATED_KEYS", g_RESTATED_KEYS, s_RESTATED_KEYS);
    pyrt::xrefs::reg_access("modules.memory_tables", "RESTRICT_OPENS", g_RESTRICT_OPENS, s_RESTRICT_OPENS);
    pyrt::xrefs::reg_access("modules.memory_tables", "RESUME", g_RESUME, s_RESUME);
    pyrt::xrefs::reg_access("modules.memory_tables", "RETRACT_SAY", g_RETRACT_SAY, s_RETRACT_SAY);
    pyrt::xrefs::reg_access("modules.memory_tables", "ROUTINE_SAY_BACK", g_ROUTINE_SAY_BACK, s_ROUTINE_SAY_BACK);
    pyrt::xrefs::reg_access("modules.memory_tables", "ROUTINE_SUBJECT", g_ROUTINE_SUBJECT, s_ROUTINE_SUBJECT);
    pyrt::xrefs::reg_access("modules.memory_tables", "SAID_ASK", g_SAID_ASK, s_SAID_ASK);
    pyrt::xrefs::reg_access("modules.memory_tables", "SAID_AS_A_SENTENCE", g_SAID_AS_A_SENTENCE, s_SAID_AS_A_SENTENCE);
    pyrt::xrefs::reg_access("modules.memory_tables", "SAME_FACT_THEIR_OWN", g_SAME_FACT_THEIR_OWN, s_SAME_FACT_THEIR_OWN);
    pyrt::xrefs::reg_access("modules.memory_tables", "SELF", g_SELF, s_SELF);
    pyrt::xrefs::reg_access("modules.memory_tables", "SLOT_KEY", g_SLOT_KEY, s_SLOT_KEY);
    pyrt::xrefs::reg_access("modules.memory_tables", "SMALL_NAMES", g_SMALL_NAMES, s_SMALL_NAMES);
    pyrt::xrefs::reg_access("modules.memory_tables", "SPELLING_APART", g_SPELLING_APART, s_SPELLING_APART);
    pyrt::xrefs::reg_access("modules.memory_tables", "STATE_WAS_SAY", g_STATE_WAS_SAY, s_STATE_WAS_SAY);
    pyrt::xrefs::reg_access("modules.memory_tables", "SWAPPABLE", g_SWAPPABLE, s_SWAPPABLE);
    pyrt::xrefs::reg_access("modules.memory_tables", "TAIL_WINDOW", g_TAIL_WINDOW, s_TAIL_WINDOW);
    pyrt::xrefs::reg_access("modules.memory_tables", "THAT_DATE", g_THAT_DATE, s_THAT_DATE);
    pyrt::xrefs::reg_access("modules.memory_tables", "TIME_ASKS", g_TIME_ASKS, s_TIME_ASKS);
    pyrt::xrefs::reg_access("modules.memory_tables", "TIME_IT", g_TIME_IT, s_TIME_IT);
    pyrt::xrefs::reg_access("modules.memory_tables", "TIME_THING", g_TIME_THING, s_TIME_THING);
    pyrt::xrefs::reg_access("modules.memory_tables", "TOPIC_ASK", g_TOPIC_ASK, s_TOPIC_ASK);
    pyrt::xrefs::reg_access("modules.memory_tables", "TURNED", g_TURNED, s_TURNED);
    pyrt::xrefs::reg_access("modules.memory_tables", "UNIT_FAMILY", g_UNIT_FAMILY, s_UNIT_FAMILY);
    pyrt::xrefs::reg_access("modules.memory_tables", "UNIT_SECONDS", g_UNIT_SECONDS, s_UNIT_SECONDS);
    pyrt::xrefs::reg_access("modules.memory_tables", "WEEK", g_WEEK, s_WEEK);
    pyrt::xrefs::reg_access("modules.memory_tables", "WEEKDAY_WORD", g_WEEKDAY_WORD, s_WEEKDAY_WORD);
    pyrt::xrefs::reg_access("modules.memory_tables", "WHAT_I_SAID", g_WHAT_I_SAID, s_WHAT_I_SAID);
    pyrt::xrefs::reg_access("modules.memory_tables", "WHEN_ASKS", g_WHEN_ASKS, s_WHEN_ASKS);
    pyrt::xrefs::reg_access("modules.memory_tables", "WHEN_END_ASK", g_WHEN_END_ASK, s_WHEN_END_ASK);
    pyrt::xrefs::reg_access("modules.memory_tables", "WHEN_END_WORD", g_WHEN_END_WORD, s_WHEN_END_WORD);
    pyrt::xrefs::reg_access("modules.memory_tables", "WHEN_INVERTED", g_WHEN_INVERTED, s_WHEN_INVERTED);
    pyrt::xrefs::reg_access("modules.memory_tables", "WHEN_INVERTED_SUBJECT", g_WHEN_INVERTED_SUBJECT, s_WHEN_INVERTED_SUBJECT);
    pyrt::xrefs::reg_access("modules.memory_tables", "WHEN_PLAN", g_WHEN_PLAN, s_WHEN_PLAN);
    pyrt::xrefs::reg_access("modules.memory_tables", "WHEN_QUALITY", g_WHEN_QUALITY, s_WHEN_QUALITY);
    pyrt::xrefs::reg_access("modules.memory_tables", "WHEN_START_ASK", g_WHEN_START_ASK, s_WHEN_START_ASK);
    pyrt::xrefs::reg_access("modules.memory_tables", "WHEN_THING", g_WHEN_THING, s_WHEN_THING);
    pyrt::xrefs::reg_access("modules.memory_tables", "WHEN_THING_VERB", g_WHEN_THING_VERB, s_WHEN_THING_VERB);
    pyrt::xrefs::reg_access("modules.memory_tables", "WHEN_VAGUE", g_WHEN_VAGUE, s_WHEN_VAGUE);
    pyrt::xrefs::reg_access("modules.memory_tables", "WHEN_VERB", g_WHEN_VERB, s_WHEN_VERB);
    pyrt::xrefs::reg_access("modules.memory_tables", "WHERE_ASK", g_WHERE_ASK, s_WHERE_ASK);
    pyrt::xrefs::reg_access("modules.memory_tables", "WHERE_PLAN", g_WHERE_PLAN, s_WHERE_PLAN);
    pyrt::xrefs::reg_access("modules.memory_tables", "WORD_NUMBERS", g_WORD_NUMBERS, s_WORD_NUMBERS);
    pyrt::xrefs::reg_access("modules.memory_tables", "_ABOUT_WHO", g__ABOUT_WHO, s__ABOUT_WHO);
    pyrt::xrefs::reg_access("modules.memory_tables", "_AMOUNTS", g__AMOUNTS, s__AMOUNTS);
    pyrt::xrefs::reg_access("modules.memory_tables", "_AMOUNT_WORDS", g__AMOUNT_WORDS, s__AMOUNT_WORDS);
    pyrt::xrefs::reg_access("modules.memory_tables", "_CONVERT", g__CONVERT, s__CONVERT);
    pyrt::xrefs::reg_access("modules.memory_tables", "_COUNT_WORD", g__COUNT_WORD, s__COUNT_WORD);
    pyrt::xrefs::reg_access("modules.memory_tables", "_FACT_SLOT", g__FACT_SLOT, s__FACT_SLOT);
    pyrt::xrefs::reg_access("modules.memory_tables", "_SUBJECT_KEYS", g__SUBJECT_KEYS, s__SUBJECT_KEYS);
    pyrt::xrefs::reg_access("modules.memory_tables", "_TENS_WORDS", g__TENS_WORDS, s__TENS_WORDS);
    pyrt::xrefs::reg_access("modules.memory_tables", "_UNIT_SAID", g__UNIT_SAID, s__UNIT_SAID);
    pyrt::xrefs::reg_access("modules.memory_tables", "_WEEKLY", g__WEEKLY, s__WEEKLY);
}

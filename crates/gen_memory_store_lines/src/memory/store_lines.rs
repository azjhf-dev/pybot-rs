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
thread_local! { static G_AT_TIME_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SHORT_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BAD_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_ECHO_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_SLOTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUDGET_FITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUDGET_MORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUDGET_TIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
thread_local! { static G_CORRECTED_WHICH_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTION_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTION_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTION_SAY_DROP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTION_SAY_PAIR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DENIAL_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DENIAL_STEMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DENIAL_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIET_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIET_RULES_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOG_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DROP_LISTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
thread_local! { static G_IN_LAW_AFTER_TEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEPT_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEY_SLOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIMITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIMIT_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOOK_BACK_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
thread_local! { static G_ON_A_MEDIUM_OR_AT_A_VENUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
thread_local! { static G_PLURAL_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLURAL_PETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTING_WORD_NOTHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_BACK_NOTHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
thread_local! { static G_SAME_FACT_THEIR_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLOT_KEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SMALL_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPELLING_APART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STATE_WAS_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SWAPPABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAIL_WINDOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TURNED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNIT_FAMILY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNIT_SECONDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_I_SAID_NOTHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.store_lines: {} {}", e.class_name, e.message); } });
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
pub fn g_CORRECTED_WHICH_PERSON() -> Py { ensure_init(); G_CORRECTED_WHICH_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTED_WHICH_PERSON(v: Py) { ensure_init(); G_CORRECTED_WHICH_PERSON.with(|c| *c.borrow_mut() = v); }
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
pub fn g_COUNT_WORD() -> Py { ensure_init(); G_COUNT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT_WORD(v: Py) { ensure_init(); G_COUNT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT_WORDS() -> Py { ensure_init(); G_COUNT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT_WORDS(v: Py) { ensure_init(); G_COUNT_WORDS.with(|c| *c.borrow_mut() = v); }
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
pub fn g_ON_A_MEDIUM_OR_AT_A_VENUE() -> Py { ensure_init(); G_ON_A_MEDIUM_OR_AT_A_VENUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_A_MEDIUM_OR_AT_A_VENUE(v: Py) { ensure_init(); G_ON_A_MEDIUM_OR_AT_A_VENUE.with(|c| *c.borrow_mut() = v); }
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
pub fn g_PLURAL_PEOPLE() -> Py { ensure_init(); G_PLURAL_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLURAL_PEOPLE(v: Py) { ensure_init(); G_PLURAL_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_PLURAL_PETS() -> Py { ensure_init(); G_PLURAL_PETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLURAL_PETS(v: Py) { ensure_init(); G_PLURAL_PETS.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTING_WORD_NOTHING() -> Py { ensure_init(); G_POINTING_WORD_NOTHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTING_WORD_NOTHING(v: Py) { ensure_init(); G_POINTING_WORD_NOTHING.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_BACK_NOTHING() -> Py { ensure_init(); G_POINTS_BACK_NOTHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK_NOTHING(v: Py) { ensure_init(); G_POINTS_BACK_NOTHING.with(|c| *c.borrow_mut() = v); }
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
pub fn g_WHAT_I_SAID_NOTHING() -> Py { ensure_init(); G_WHAT_I_SAID_NOTHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_I_SAID_NOTHING(v: Py) { ensure_init(); G_WHAT_I_SAID_NOTHING.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_ASK() -> Py { ensure_init(); G_WHERE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_ASK(v: Py) { ensure_init(); G_WHERE_ASK.with(|c| *c.borrow_mut() = v); }
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
    let _ = py_str("The store's template pools: the words it reads and the words it says.\n\nEvery upper-case pool in `bot/modules/memory_store.py` whose value is a literal - a string, a\ntuple or a list of them, or a group of pools named from them - lives here, so the module that\nreads and answers with them stays a readable size. They are moved, not rewritten: the values are\nidentical, and the module imports every name back, so `memory_store.TOPIC_ASK` is the same object\nit always was. A pool built by calling something at import time (`tuple(_p(...) for ...)`) stays\nin the module, because moving it would move the words and the code that makes them apart.\n");
    s_PRIORITY(Py::Int(7000));
    s_ANSWERS(py_tuple(vec![py_str("pets"), py_str("family-and-parenting")]));
    s_RECALL_HINTS(py_tuple(vec![py_str("say"), py_str("said"), py_str("saying"), py_str("remember"), py_str("recall"), py_str("again"), py_str("told"), py_str("tell"), py_str("know"), py_str("what's"), py_str("whats"), py_str("what")]));
    s_SELF(py_tuple(vec![py_str("you"), py_str("your"), py_str("we"), py_str("i")]));
    s_NOT_THE_PLAN(py_str("(?!\\s+for\\b)"));
    s_ON_A_MEDIUM_OR_AT_A_VENUE(py_str("telly|tv|television|radio|bbc|itv|channel|netflix|amazon|prime|disney|iplayer|now\\s?tv|sky|spotify|youtube|cinema|cinemas|pictures|theatre|theater|matinee|gig|gigs|concert|concerts|arena|playhouse|phone|phones|mobile|screen|screens|monitor|laptop|laptops|computer|computers|pc|tablet|tablets|ipad|kindle|speaker|speakers|headphones|earphones|earbuds|device"));
    s_NOT_ON_A_MEDIUM(op_add(&op_add(&py_str("(?!\\s*(?:(?:at|in)\\s+)?(?:the\\s+|a\\s+|my\\s+|our\\s+)?(?:"), &g_ON_A_MEDIUM_OR_AT_A_VENUE()), &py_str(")\\b)")));
    s_REPEAT_ADVERB(py_str("(?:again|slower|slowly|step by step|once more|clearer|simply|for me|please|thanks|in order)"));
    s_LIMITS(py_tuple(vec![py_str("allergies"), py_str("diet"), py_str("condition")]));
    s_DENIAL_SAY(py_dict(vec![(py_str("allergies"), py_str("{Thing} back on the table, then.")), (py_str("condition"), py_str("Right, no {thing} — that makes things easier, then.")), (py_str("diet"), py_str("Right — meat is back on the table, then.")), (py_str("pets"), py_str("Right — no {thing}, then."))]));
    s_CORRECTED_SAY(py_dict(vec![(py_str("allergies"), py_str("{Thing}, not {old} — got it.")), (py_str("condition"), py_str("Right, {thing} now, not {old} — got it."))]));
    s_RETRACT_SAY(py_dict(vec![(py_str("allergies"), py_str("the allergy note")), (py_str("condition"), py_str("the health note")), (py_str("diet"), py_str("the diet note")), (py_str("budget"), py_str("the budget")), (py_str("job"), py_str("the job")), (py_str("city"), py_str("the place you live")), (py_str("from"), py_str("where you’re from")), (py_str("born"), py_str("where you were born")), (py_str("age"), py_str("your age")), (py_str("height"), py_str("your height")), (py_str("name"), py_str("your name")), (py_str("places"), py_str("that place"))]));
    s_NOT_RETRACTED(py_tuple(vec![py_str("taste"), py_str("other"), py_str("people"), py_str("pet"), Py::None]));
    s__FACT_SLOT(py_str("(?:job|work|name|budget|allerg\\w*|pets?|cats?|dogs?|kittens?|pupp(?:y|ies)|animals?|salary|birthday|age|partner|husband|wife|kids?|children|son|daughter|mum|mom|dad|sister|brother|car|address|postcode)"));
    s_PET_AGE_FIX_WORDS(Py::Int(8));
    s_OTHER_SAID(py_tuple(vec![py_tuple(vec![py_str("name"), py_str("is called")]), py_tuple(vec![py_str("allergies"), py_str("is allergic to")]), py_tuple(vec![py_str("condition"), py_str("has")]), py_tuple(vec![py_str("diet"), py_str("is")]), py_tuple(vec![py_str("diet"), py_str("eats")])]));
    s_OTHER_KIND_SAY(py_dict(vec![(py_str("name"), py_str("{v}, got it.")), (py_str("allergies"), py_str("{v} it is, then — I'll keep your {who} off those.")), (py_str("condition"), py_str("{v} it is, then — I'll keep that in mind for your {who}.")), (py_str("diet"), py_str("{v} it is, then — that's what I'll go with for your {who}."))]));
    s_AGE_FIX_WORDS(Py::Int(8));
    s_NAME_FIX_WORDS(Py::Int(9));
    s_NAME_GIVEN_WORDS(Py::Int(5));
    fn __data1() -> Py { py_dict(vec![(py_str("pound"), py_str("money")), (py_str("pounds"), py_str("money")), (py_str("quid"), py_str("money")), (py_str("euro"), py_str("money")), (py_str("euros"), py_str("money")), (py_str("dollar"), py_str("money")), (py_str("dollars"), py_str("money")), (py_str("k"), py_str("money")), (py_str("kilo"), py_str("weight")), (py_str("kilos"), py_str("weight")), (py_str("kg"), py_str("weight")), (py_str("kgs"), py_str("weight")), (py_str("kilogram"), py_str("weight")), (py_str("kilograms"), py_str("weight")), (py_str("stone"), py_str("weight")), (py_str("stones"), py_str("weight")), (py_str("st"), py_str("weight")), (py_str("lb"), py_str("weight")), (py_str("lbs"), py_str("weight")), (py_str("gram"), py_str("weight")), (py_str("grams"), py_str("weight")), (py_str("g"), py_str("weight")), (py_str("cm"), py_str("weight")), (py_str("centimetre"), py_str("weight")), (py_str("centimetres"), py_str("weight")), (py_str("centimeter"), py_str("weight")), (py_str("centimeters"), py_str("weight")), (py_str("foot"), py_str("weight")), (py_str("feet"), py_str("weight")), (py_str("ft"), py_str("weight")), (py_str("inch"), py_str("weight")), (py_str("inches"), py_str("weight")), (py_str("litre"), py_str("volume")), (py_str("litres"), py_str("volume")), (py_str("liter"), py_str("volume")), (py_str("liters"), py_str("volume")), (py_str("mile"), py_str("distance")), (py_str("miles"), py_str("distance")), (py_str("km"), py_str("distance")), (py_str("minute"), py_str("time")), (py_str("minutes"), py_str("time")), (py_str("min"), py_str("time")), (py_str("mins"), py_str("time")), (py_str("hour"), py_str("time")), (py_str("hours"), py_str("time")), (py_str("day"), py_str("time")), (py_str("days"), py_str("time")), (py_str("week"), py_str("time")), (py_str("weeks"), py_str("time")), (py_str("month"), py_str("time")), (py_str("months"), py_str("time")), (py_str("year"), py_str("time")), (py_str("years"), py_str("time"))]) }
    s_UNIT_FAMILY(__data1());
    s_FACT_FAMILY(py_dict(vec![(py_str("budget"), py_str("money")), (py_str("pay"), py_str("money")), (py_str("salary"), py_str("money")), (py_str("wage"), py_str("money")), (py_str("rent"), py_str("money")), (py_str("price"), py_str("money")), (py_str("spend"), py_str("money")), (py_str("weight"), py_str("weight")), (py_str("height"), py_str("weight")), (py_str("distance"), py_str("distance"))]));
    s_NOT_A_UNIT(py_str("in|on|at|for|by|before|after|and|then|this|next|of|to|from|until|till|or|but|so|with|without|the|a|an"));
    s_AMOUNT_WORD(py_str("(?:an?|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|half\\s+an?|half|a\\s+couple\\s+of|a\\s+few|a\\s+bit\\s+of|\\d+(?:[.,]\\d+)?)"));
    s_SPELLING_APART(Py::Int(3));
    s_CORRECTION_SAY_DROP(py_dict(vec![(py_str("allergies"), py_str("Got it — no {v}."))]));
    s_DROP_LISTS(py_tuple(vec![py_str("allergies")]));
    fn __data2() -> Py { py_dict(vec![(py_str("budget"), py_str("{v} — got it, I'll work to that.")), (py_str("pet"), py_str("{v} — got it, I'll call the {a} {v}.")), (py_str("pets_count"), py_str("{v} — got it.")), (py_str("kids"), py_str("{v} — got it.")), (py_str("name"), py_str("{v} — got it, I'll call you {v}.")), (py_str("person_name"), py_str("{v} — got it, I'll say {v}.")), (py_str("age"), py_str("{v} — got it, {v} it is.")), (py_str("weight"), py_str("{v} — got it, you weigh {v}.")), (py_str("city"), py_str("{v} — got it, I'll say {v}.")), (py_str("places"), py_str("{v} — got it, I'll say {v}.")), (py_str("job"), py_str("{v} — got it, I'll say {v}.")), (py_str("plan"), py_str("Got it — {v}.")), (py_str("allergies"), py_str("{v} — got it, I'll keep that out of anything I suggest.")), (py_str("birthday"), py_str("Got it — your birthday is {v}.")), (py_str("line"), py_str("{v} — got it."))]) }
    s_CORRECTION_SAY(__data2());
    s_CORRECTION_SAY_PAIR(py_dict(vec![(py_str("plan"), py_str("{v}, not {w} — got it."))]));
    s_PET_RENAMED_SAY(py_tuple(vec![py_str("{v}, not {w} — got it. How's {v} settling in?"), py_str("Right — {v}, not {w}. That's the name I'll use."), py_str("{v}, not {w} — noted. {v} it is from now on."), py_str("{v}, not {w}; glad you said. How's {v} getting on?")]));
    s_BARE_ECHO_ONLY(py_str("{v} — got it."));
    s_CORRECTION_BACK(Py::Int(4));
    s_AT_TIME_VALUE(py_str("(?:\\d{1,2}(?:[:.]\\d{2})?(?!\\d)|(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\b|noon|midday|midnight)(?:\\s*(?:am|pm|o'?clock))?"));
    s_KIN_ASK(py_str("[a-z]+(?:[ \\t]*-?[ \\t]*in[ \\t]*-?[ \\t]*laws?)?"));
    s_IN_LAW_AFTER_TEXT(py_str("[ \\t]*-?[ \\t]*in[ \\t]*-?[ \\t]*laws?\\b"));
    s_PRONOUN_FITS(py_dict(vec![(py_str("his"), py_tuple(vec![py_str("he")])), (py_str("him"), py_tuple(vec![py_str("he")])), (py_str("he"), py_tuple(vec![py_str("he")])), (py_str("her"), py_tuple(vec![py_str("she")])), (py_str("she"), py_tuple(vec![py_str("she")])), (py_str("their"), py_tuple(vec![py_str("he"), py_str("she")])), (py_str("them"), py_tuple(vec![py_str("he"), py_str("she")])), (py_str("they"), py_tuple(vec![py_str("he"), py_str("she")]))]));
    s_CHILD_RELATIONS(py_tuple(vec![py_str("son"), py_str("daughter"), py_str("child"), py_str("kid"), py_str("boy"), py_str("girl"), py_str("stepson"), py_str("stepdaughter"), py_str("twins")]));
    s__ABOUT_WHO(py_str("(?:my|our)\\s+[a-z]+\\b|the\\s+(?:kids?|children|child)\\b|her\\b|him\\b|them\\b|she\\b|he\\b|they\\b"));
    s_CHILD(py_tuple(vec![py_str("son"), py_str("daughter"), py_str("twins"), py_str("baby"), py_str("child"), py_str("kid"), py_str("boy"), py_str("girl"), py_str("youngest"), py_str("eldest"), py_str("oldest")]));
    s_CHILD_OF(py_dict(vec![(py_str("children"), py_str("child")), (py_str("kids"), py_str("kid")), (py_str("childs"), py_str("child")), (py_str("babies"), py_str("baby")), (py_str("boys"), py_str("boy")), (py_str("girls"), py_str("girl")), (py_str("sons"), py_str("son")), (py_str("daughters"), py_str("daughter"))]));
    fn __data4() -> Py { py_tuple(vec![py_str("parent"), py_str("mother"), py_str("mum"), py_str("mom"), py_str("mam"), py_str("mummy"), py_str("mommy"), py_str("father"), py_str("dad"), py_str("daddy"), py_str("papa"), py_str("stepmother"), py_str("stepfather")]) }
    fn __data3() -> Py { py_dict(vec![(py_str("parents"), __data4()), (py_str("grandparents"), py_tuple(vec![py_str("grandparent"), py_str("grandmother"), py_str("grandma"), py_str("gran"), py_str("nan"), py_str("nana"), py_str("granny"), py_str("grandfather"), py_str("grandad"), py_str("granddad"), py_str("grandpa")])), (py_str("siblings"), py_tuple(vec![py_str("sibling"), py_str("brother"), py_str("sister"), py_str("half brother"), py_str("half sister")])), (py_str("brothers"), py_tuple(vec![py_str("brother"), py_str("half brother")])), (py_str("sisters"), py_tuple(vec![py_str("sister"), py_str("half sister")])), (py_str("kids"), g_CHILD()), (py_str("children"), g_CHILD()), (py_str("twins"), py_tuple(vec![py_str("twin"), py_str("son"), py_str("daughter"), py_str("child"), py_str("kid")])), (py_str("boys"), py_tuple(vec![py_str("son"), py_str("boy"), py_str("child"), py_str("kid")])), (py_str("girls"), py_tuple(vec![py_str("daughter"), py_str("girl"), py_str("child"), py_str("kid")])), (py_str("grandchildren"), py_tuple(vec![py_str("grandson"), py_str("granddaughter"), py_str("grandchild")])), (py_str("uncles"), py_tuple(vec![py_str("uncle")])), (py_str("aunts"), py_tuple(vec![py_str("aunt")])), (py_str("cousins"), py_tuple(vec![py_str("cousin")]))]) }
    s_FAMILY_GROUP(__data3());
    s_FOOD_PHRASES(py_dict(vec![(py_str("nuts"), py_tuple(vec![py_str("peanut butter"), py_str("satay sauce"), py_str("nut butter"), py_str("nut roast"), py_str("almond milk")])), (py_str("peanuts"), py_tuple(vec![py_str("peanut butter"), py_str("satay sauce"), py_str("nut butter")])), (py_str("dairy"), py_tuple(vec![py_str("ice cream"), py_str("ice-cream"), py_str("milk chocolate"), py_str("double cream"), py_str("sour cream"), py_str("cream cheese")])), (py_str("gluten"), py_tuple(vec![py_str("soy sauce"), py_str("stock cube"), py_str("stock cubes")])), (py_str("wheat"), py_tuple(vec![py_str("soy sauce"), py_str("stock cube"), py_str("stock cubes")]))]));
    s_ADVICE_MODULES(py_tuple(vec![py_str("answers"), py_str("chat")]));
    s_SLOT_KEY(py_dict(vec![(py_str("name"), py_tuple(vec![py_str("name")])), (py_str("job"), py_tuple(vec![py_str("job")])), (py_str("city"), py_tuple(vec![py_str("city"), py_str("town"), py_str("from")])), (py_str("budget"), py_tuple(vec![py_str("budget")])), (py_str("allergy"), py_tuple(vec![py_str("allergies"), py_str("allergy")])), (py_str("pets"), py_tuple(vec![py_str("pets")])), (py_str("age"), py_tuple(vec![py_str("age")]))]));
    s_BARE_SLOTS(py_tuple(vec![py_tuple(vec![py_str("allergy"), py_str("\\ballerg")]), py_tuple(vec![py_str("budget"), py_str("\\bbudget")]), py_tuple(vec![py_str("pets"), py_str("\\b(?:dog|cat|pet|animal)s?\\b")]), py_tuple(vec![py_str("job"), py_str("\\b(?:job|work)\\b")]), py_tuple(vec![py_str("city"), py_str("\\b(?:city|place|town)\\b")]), py_tuple(vec![py_str("name"), py_str("\\bname\\b")]), py_tuple(vec![py_str("age"), py_str("\\bage\\b")])]));
    s_WHERE_ASK(py_str("where(?:abouts| abouts)?"));
    s_KIN(py_str("mum|mom|mother|dad|father|son|daughter|child|kid|wife|husband|partner|brother|sister|gran|granny|nan|nana|grandma|grandad|grandpa|grandmother|grandfather|friend|mate|baby|children|kids|nephew|niece|aunt|uncle|cousin|stepson|stepdaughter"));
    s_BAD_NEWS(py_str("(?:is|are|was|were|has been|have been|had been|got|has got|have got|got taken|has ended up|ended up)?\\s*(?:really\\s+|very\\s+|quite\\s+|so\\s+)?(?:in hospital|in the hospital|in a coma|on a ventilator|unwell|(?:ill|sick)(?!\\s+of\\b)|poorly|dying|struggling|getting worse|worse)|(?:was|got|has been|had been)?\\s*(?:diagnosed|taken into hospital)|has cancer|had cancer|has had a fall|had a fall|took a fall|is terminal"));
    s__UNIT_SAID(py_dict(vec![(py_str("minutes"), py_tuple(vec![py_str("minute"), py_str("minutes")])), (py_str("hours"), py_tuple(vec![py_str("hour"), py_str("hours")])), (py_str("steps"), py_tuple(vec![py_str("step"), py_str("steps")])), (py_str("litres"), py_tuple(vec![py_str("litre"), py_str("litres")])), (py_str("glasses"), py_tuple(vec![py_str("glass"), py_str("glasses")])), (py_str("km"), py_tuple(vec![py_str("km"), py_str("km")])), (py_str("miles"), py_tuple(vec![py_str("mile"), py_str("miles")])), (py_str("days"), py_tuple(vec![py_str("day"), py_str("days")])), (py_str("weeks"), py_tuple(vec![py_str("week"), py_str("weeks")])), (py_str("nights"), py_tuple(vec![py_str("night"), py_str("nights")])), (py_str("a week"), py_tuple(vec![py_str("session a week"), py_str("sessions a week")])), (py_str("a day"), py_tuple(vec![py_str("session a day"), py_str("sessions a day")]))]));
    fn __data5() -> Py { py_tuple(vec![py_tuple(vec![py_str("a week"), py_str("(?:(?:times?|sessions?|runs?|workouts?|walks?)\\s+)?(?:a|per|each)\\s+week")]), py_tuple(vec![py_str("a day"), py_str("(?:(?:times?|sessions?|glasses?)\\s+)?(?:a|per|each)\\s+day")]), py_tuple(vec![py_str("minutes"), py_str("minutes?|mins?")]), py_tuple(vec![py_str("hours"), py_str("hours?|hrs?")]), py_tuple(vec![py_str("litres"), py_str("litres?|liters?|l\\b")]), py_tuple(vec![py_str("glasses"), py_str("glasses?")]), py_tuple(vec![py_str("steps"), py_str("steps?")]), py_tuple(vec![py_str("km"), py_str("km\\b|kilometres?|kilometers?|k\\b")]), py_tuple(vec![py_str("miles"), py_str("miles?|mi\\b")]), py_tuple(vec![py_str("days"), py_str("days?")]), py_tuple(vec![py_str("weeks"), py_str("weeks?")]), py_tuple(vec![py_str("nights"), py_str("nights?")]), py_tuple(vec![py_str("grams"), py_str("grams?|grammes?|g\\b")]), py_tuple(vec![py_str("calories"), py_str("calories|cals?\\b|kilocalories|kcals?")]), py_tuple(vec![py_str("micrograms"), py_str("micrograms?|microgrammes?|mcgs?|\\u00b5g|ug\\b")]), py_tuple(vec![py_str("milligrams"), py_str("milligrams?|milligrammes?|mgs?")])]) }
    s__AMOUNTS(__data5());
    fn __data6() -> Py { py_dict(vec![(py_str("once"), Py::Int(1)), (py_str("twice"), Py::Int(2)), (py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12))]) }
    s__AMOUNT_WORDS(__data6());
    s__COUNT_WORD(py_str("\\b(?:\\d+(?:,\\d{3})*(?:\\.\\d+)?|(?:once|twice|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\b)"));
    s__CONVERT(py_dict(vec![(py_tuple(vec![py_str("minutes"), py_str("hours")]), Py::Float(60.0)), (py_tuple(vec![py_str("hours"), py_str("minutes")]), op_truediv(&Py::Int(1), &Py::Float(60.0))), (py_tuple(vec![py_str("km"), py_str("miles")]), Py::Float(0.621371)), (py_tuple(vec![py_str("miles"), py_str("km")]), Py::Float(1.609344))]));
    s__WEEKLY(py_str("a week"));
    s_FIGURE_UNITS(py_dict(vec![(py_str("week"), py_tuple(vec![py_str("week"), py_str("weeks"), py_str("fortnight"), py_str("fortnights")])), (py_str("day"), py_tuple(vec![py_str("day"), py_str("days")])), (py_str("month"), py_tuple(vec![py_str("month"), py_str("months")])), (py_str("year"), py_tuple(vec![py_str("year"), py_str("years")])), (py_str("hour"), py_tuple(vec![py_str("hour"), py_str("hours")])), (py_str("minute"), py_tuple(vec![py_str("minute"), py_str("minutes")])), (py_str("second"), py_tuple(vec![py_str("second"), py_str("seconds")])), (py_str("mile"), py_tuple(vec![py_str("mile"), py_str("miles")])), (py_str("km"), py_tuple(vec![py_str("km"), py_str("kilometre"), py_str("kilometres"), py_str("kilometer"), py_str("kilometers")])), (py_str("kilo"), py_tuple(vec![py_str("kilo"), py_str("kilos"), py_str("kg"), py_str("kilogram"), py_str("kilograms")])), (py_str("stone"), py_tuple(vec![py_str("stone"), py_str("stones")])), (py_str("pound"), py_tuple(vec![py_str("pound"), py_str("pounds"), py_str("lb"), py_str("lbs")]))]));
    s_FIGURE_TIME(py_tuple(vec![py_str("week"), py_str("day"), py_str("month"), py_str("year"), py_str("hour"), py_str("minute"), py_str("second")]));
    s_FIGURE_NUMBER(py_str("(?:\\d{1,3}(?:,\\d{3})+(?:\\.\\d+)?|\\d+(?:\\.\\d+)?|a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fifteen|twenty|thirty|forty|fifty|sixty|ninety|hundred)"));
    s_FIGURE_QUALIFIER(py_str("(?:about|around|roughly|nearly|almost|up to|at least|over|under|just|only|maybe|probably|another)"));
    s_LOOK_BACK_FIGURE(Py::Int(3));
    s_FIGURE_ANSWER_LEN(Py::Int(120));
    s_COUNT_WORD(py_str("(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|hundred|dozen|couple|few|several|lot|lots|plenty|number)"));
    s_PLACE_NOT(py_str("the|a|an|my|his|her|their|our|your|its|it|this|that|these|those|order|time|general|total|all|front|charge|public|private|fact|mind|particular|future|past|advance|june|july|may|march|april|august|september|october|november|december|january|february|monday|tuesday|wednesday|thursday|friday|saturday|sunday|home|work|bed|town|touch|least|once|twice|night|morning|evening|afternoon|weekend|month|year|between|case|some|any|no|one|two|first|last|next|same|other|else|person|people"));
    s_TURNED(py_dict(vec![(py_str("we"), py_str("you")), (py_str("our"), py_str("your")), (py_str("my"), py_str("your")), (py_str("us"), py_str("you")), (py_str("me"), py_str("you")), (py_str("im"), py_str("you are")), (py_str("i"), py_str("you")), (py_str("ive"), py_str("you've"))]));
    fn __data7() -> Py { py_dict(vec![(py_str("job"), py_str("job")), (py_str("jobs"), py_str("job")), (py_str("work"), py_str("job")), (py_str("role"), py_str("job")), (py_str("shift"), py_str("job")), (py_str("shifts"), py_str("job")), (py_str("hours"), py_str("job")), (py_str("pay"), py_str("job")), (py_str("salary"), py_str("job")), (py_str("boss"), py_str("job")), (py_str("colleagues"), py_str("job")), (py_str("son"), py_str("people")), (py_str("daughter"), py_str("people")), (py_str("kid"), py_str("kids")), (py_str("kids"), py_str("kids")), (py_str("child"), py_str("kids")), (py_str("children"), py_str("kids")), (py_str("mum"), py_str("people")), (py_str("dad"), py_str("people")), (py_str("mother"), py_str("people")), (py_str("father"), py_str("people")), (py_str("sister"), py_str("people")), (py_str("brother"), py_str("people")), (py_str("partner"), py_str("people")), (py_str("wife"), py_str("people")), (py_str("husband"), py_str("people")), (py_str("family"), py_str("people")), (py_str("parents"), py_str("people")), (py_str("dog"), py_str("pets")), (py_str("cat"), py_str("pets")), (py_str("puppy"), py_str("pets")), (py_str("kitten"), py_str("pets")), (py_str("pets"), py_str("pets")), (py_str("car"), py_str("owns")), (py_str("bike"), py_str("owns")), (py_str("phone"), py_str("owns")), (py_str("laptop"), py_str("owns")), (py_str("house"), py_str("city")), (py_str("flat"), py_str("city")), (py_str("home"), py_str("city")), (py_str("garden"), py_str("city")), (py_str("town"), py_str("city")), (py_str("area"), py_str("city")), (py_str("name"), py_str("name")), (py_str("age"), py_str("age")), (py_str("budget"), py_str("budget")), (py_str("diet"), py_str("diet"))]) }
    s_PART_HELD(__data7());
    s_PART_KEYS(py_tuple(vec![py_str("job"), py_str("people"), py_str("kids"), py_str("pets"), py_str("owns"), py_str("city"), py_str("places"), py_str("name"), py_str("age"), py_str("budget"), py_str("diet"), py_str("allergies")]));
    s_KEPT_THINGS(py_tuple(vec![py_str("job"), py_str("people"), py_str("owns"), py_str("kids"), py_str("pets"), py_str("name"), py_str("age"), py_str("budget"), py_str("diet")]));
    s_NAMED_BY(py_dict(vec![(py_str("name"), py_str("\\bname\\b|\\bcalled\\b")), (py_str("age"), py_str("\\bold\\b|\\bage\\b|\\d")), (py_str("job"), py_str("\\bjob\\b|\\bwork\\b|\\bliving\\b|\\bam i an?\\b|\\bwas i an?\\b")), (py_str("city"), py_str("\\bliv(?:e|es|ing)\\b|\\bhome\\b|\\bcity\\b|\\btown\\b")), (py_str("from"), py_str("\\bfrom\\b|\\bhometown\\b|\\bgrew up\\b|\\bborn\\b")), (py_str("allergies"), py_str("\\ballerg")), (py_str("budget"), py_str("\\bbudget\\b|\\bspend\\b|\\bafford\\b|\\d")), (py_str("diet"), py_str("\\bvegetarian\\b|\\bvegan\\b|\\bdiet\\b|\\beat\\b|\\bfood\\b")), (py_str("pets"), py_str("\\bdog\\b|\\bcat\\b|\\bpet\\b|\\bhamster\\b|\\brabbit\\b|\\bfish\\b|\\bbird\\b|\\bhorse\\b"))]));
    s_RESTATED_KEYS(py_tuple(vec![py_str("name"), py_str("age"), py_str("job"), py_str("city"), py_str("from"), py_str("diet"), py_str("budget"), py_str("allergies"), py_str("pets")]));
    s_SWAPPABLE(py_tuple(vec![py_str("city"), py_str("places"), py_str("job"), py_str("diet"), py_str("plan"), py_str("plans")]));
    s__TENS_WORDS(py_tuple(vec![py_str(""), py_str(""), py_str("twenty"), py_str("thirty"), py_str("forty"), py_str("fifty"), py_str("sixty"), py_str("seventy"), py_str("eighty"), py_str("ninety")]));
    s_CORRECTED_AGAIN(py_tuple(vec![py_str("{Value}, not {was} — got it."), py_str("{Value}, then — your {who} is {value} now."), py_str("Not {was} — {value}. Got it.")]));
    s_CORRECTED_AGAIN_WHO(py_tuple(vec![py_str("{Value} — got it, your {who} is {value} from now on."), py_str("{Value}, then — I have your {who} down as {value}.")]));
    s_CORRECTED_WHICH_PERSON(py_tuple(vec![py_str("{Value} — your {who}, not your {was_who}. Got it."), py_str("Ah, it's your {who} who is {value} — I had that with your {was_who}."), py_str("{Value} goes with your {who}, then, not your {was_who}. Noted."), py_str("Your {who}, not your {was_who} — I've moved that over.")]));
    s_CAT_NAMES(py_tuple(vec![py_str("Miso"), py_str("Pepper"), py_str("Juniper"), py_str("Biscuit"), py_str("Saffron"), py_str("Marmalade"), py_str("Clover"), py_str("Smudge"), py_str("Willow"), py_str("Pip")]));
    s_DOG_NAMES(py_tuple(vec![py_str("Finn"), py_str("Nell"), py_str("Bramble"), py_str("Otto"), py_str("Rufus"), py_str("Tilly"), py_str("Ziggy"), py_str("Juno"), py_str("Alfie"), py_str("Moss")]));
    s_SMALL_NAMES(py_tuple(vec![py_str("Pip"), py_str("Peanut"), py_str("Willow"), py_str("Nibbles"), py_str("Bramble"), py_str("Squeak"), py_str("Poppy"), py_str("Titch")]));
    s_NAME_ADVICE(py_tuple(vec![py_str("Pick the one you would not mind calling across a park at night - that rules more names out than anything else."), py_str("Say each one out loud first: a name you shorten anyway is the one you will end up using."), py_str("Two syllables carries best, and use only the chosen one for the first fortnight, so it learns its own name quickly."), py_str("The right answer is usually the one everybody in the house can spell and shout.")]));
    s_RESUME(py_tuple(vec![py_str("Back to {thing}, then. What did you want to know about it?"), py_str("OK, back to {thing}. Where were we?"), py_str("Right, {thing} again. Go on."), py_str("Back to {thing} it is. I'm still here."), py_str("Righto, {thing}. What's the question?"), py_str("Back to {thing}. I've still got what you told me about it.")]));
    s_NO_PLANS_REPLY(py_tuple(vec![py_str("You haven't told me about anything coming up yet. What's on?"), py_str("Nothing so far — you haven't mentioned anything coming up. What have you got?")]));
    s_WEEK(py_tuple(vec![py_str("monday"), py_str("tuesday"), py_str("wednesday"), py_str("thursday"), py_str("friday"), py_str("saturday"), py_str("sunday")]));
    s_TAIL_WINDOW(Py::Int(40));
    s_PLURAL_PEOPLE(py_tuple(vec![py_str("children"), py_str("people"), py_str("parents"), py_str("kids"), py_str("twins"), py_str("boys"), py_str("girls"), py_str("friends"), py_str("mates"), py_str("flatmates"), py_str("housemates"), py_str("colleagues")]));
    fn __data8() -> Py { py_dict(vec![(py_str("vegan"), py_tuple(vec![py_str("meat, fish, dairy or eggs"), py_str("")])), (py_str("plant-based"), py_tuple(vec![py_str("meat, fish, dairy or eggs"), py_str("")])), (py_str("plant based"), py_tuple(vec![py_str("meat, fish, dairy or eggs"), py_str("")])), (py_str("vegetarian"), py_tuple(vec![py_str("meat or fish"), py_str("")])), (py_str("veggie"), py_tuple(vec![py_str("meat or fish"), py_str("")])), (py_str("veg"), py_tuple(vec![py_str("meat or fish"), py_str("")])), (py_str("pescatarian"), py_tuple(vec![py_str("meat"), py_str(" (fish is fine)")])), (py_str("gluten-free"), py_tuple(vec![py_str("gluten"), py_str("")])), (py_str("dairy-free"), py_tuple(vec![py_str("dairy"), py_str("")])), (py_str("lactose-free"), py_tuple(vec![py_str("dairy"), py_str("")])), (py_str("wheat-free"), py_tuple(vec![py_str("wheat"), py_str("")])), (py_str("sugar-free"), py_tuple(vec![py_str("sugar"), py_str("")])), (py_str("low-carb"), py_tuple(vec![py_str("carbohydrates"), py_str("")]))]) }
    s_DIET_RULES_OUT(__data8());
    fn __data9() -> Py { py_tuple(vec![py_str("diet"), py_str("diets"), py_str("dieting"), py_str("eat"), py_str("eats"), py_str("eating"), py_str("food"), py_str("meal"), py_str("meals"), py_str("vegan"), py_str("vegetarian"), py_str("veggie"), py_str("pescatarian"), py_str("keto"), py_str("paleo")]) }
    fn __data10() -> Py { py_tuple(vec![py_str("budget"), py_str("budgets"), py_str("afford"), py_str("affording"), py_str("save"), py_str("saving"), py_str("savings"), py_str("wage"), py_str("wages"), py_str("salary"), py_str("income"), py_str("earn"), py_str("earns"), py_str("earning"), py_str("cash")]) }
    fn __data11() -> Py { py_tuple(vec![py_str("job"), py_str("jobs"), py_str("work"), py_str("works"), py_str("working"), py_str("worked"), py_str("role"), py_str("career"), py_str("employer"), py_str("employed"), py_str("office"), py_str("shift"), py_str("shifts"), py_str("boss"), py_str("manager"), py_str("colleagues")]) }
    s_DENIAL_WORDS(py_dict(vec![(py_str("diet"), __data9()), (py_str("budget"), __data10()), (py_str("allergy"), py_tuple(vec![py_str("intolerant"), py_str("intolerance"), py_str("react"), py_str("reaction"), py_str("epipen"), py_str("coeliac"), py_str("celiac"), py_str("anaphylaxis"), py_str("anaphylactic")])), (py_str("job"), __data11())]));
    s_DENIAL_STEMS(py_dict(vec![(py_str("allergy"), py_tuple(vec![py_str("allerg")]))]));
    fn __data12() -> Py { py_tuple(vec![py_str("son"), py_str("daughter"), py_str("child"), py_str("kid"), py_str("baby"), py_str("twin"), py_str("boy"), py_str("girl"), py_str("stepson"), py_str("stepdaughter"), py_str("teenager"), py_str("lad"), py_str("lass")]) }
    s_CHILD_RELATIONS(__data12());
    s_GROUP_OF(py_dict(vec![(py_str("children"), g_CHILD_RELATIONS()), (py_str("childs"), g_CHILD_RELATIONS()), (py_str("kids"), g_CHILD_RELATIONS()), (py_str("parents"), py_tuple(vec![py_str("mum"), py_str("dad"), py_str("parent"), py_str("stepmum"), py_str("stepdad")])), (py_str("siblings"), py_tuple(vec![py_str("brother"), py_str("sister"), py_str("half-brother"), py_str("half-sister")])), (py_str("brothers"), py_tuple(vec![py_str("brother")])), (py_str("sisters"), py_tuple(vec![py_str("sister")])), (py_str("grandparents"), py_tuple(vec![py_str("grandma"), py_str("grandad"), py_str("grandparent")])), (py_str("grandchildren"), py_tuple(vec![py_str("grandson"), py_str("granddaughter"), py_str("grandchild")])), (py_str("aunts"), py_tuple(vec![py_str("aunt")])), (py_str("uncles"), py_tuple(vec![py_str("uncle")])), (py_str("cousins"), py_tuple(vec![py_str("cousin")]))]));
    fn __data13() -> Py { py_dict(vec![(py_str("mother"), py_str("mum")), (py_str("mom"), py_str("mum")), (py_str("mam"), py_str("mum")), (py_str("mummy"), py_str("mum")), (py_str("father"), py_str("dad")), (py_str("daddy"), py_str("dad")), (py_str("nana"), py_str("grandma")), (py_str("nan"), py_str("grandma")), (py_str("granny"), py_str("grandma")), (py_str("gran"), py_str("grandma")), (py_str("grandmother"), py_str("grandma")), (py_str("grandpa"), py_str("grandad")), (py_str("grandfather"), py_str("grandad")), (py_str("auntie"), py_str("aunt")), (py_str("kid"), py_str("child")), (py_str("childs"), py_str("child")), (py_str("kids"), py_str("child"))]) }
    s_RELATION_ALIKE(__data13());
    s_MONTH_ORDER(py_tuple(vec![py_str("january"), py_str("february"), py_str("march"), py_str("april"), py_str("may"), py_str("june"), py_str("july"), py_str("august"), py_str("september"), py_str("october"), py_str("november"), py_str("december")]));
    fn __data14() -> Py { py_tuple(vec![py_str("brothers"), py_str("sisters"), py_str("siblings"), py_str("children"), py_str("kids"), py_str("childs"), py_str("parents"), py_str("grandchildren"), py_str("grandparents"), py_str("aunts"), py_str("uncles"), py_str("cousins"), py_str("family"), py_str("brother"), py_str("sister"), py_str("child"), py_str("kid"), py_str("sons"), py_str("daughters"), py_str("mums"), py_str("dads"), py_str("parents"), py_str("son"), py_str("daughter"), py_str("mum"), py_str("dad"), py_str("parent"), py_str("grandson"), py_str("granddaughter"), py_str("aunt"), py_str("uncle"), py_str("cousin")]) }
    s_HAVE_ANY_WORDS(__data14());
    s_HER_WORD(py_dict(vec![(py_str("sister"), py_str("her")), (py_str("daughter"), py_str("her")), (py_str("mum"), py_str("her")), (py_str("granddaughter"), py_str("her")), (py_str("aunt"), py_str("her")), (py_str("brother"), py_str("him")), (py_str("son"), py_str("him")), (py_str("dad"), py_str("him")), (py_str("grandson"), py_str("him")), (py_str("uncle"), py_str("him"))]));
    s_DIET_FOOD(py_dict(vec![(py_str("vegetarian"), py_tuple(vec![py_str("meat"), py_str("fish")])), (py_str("veggie"), py_tuple(vec![py_str("meat"), py_str("fish")])), (py_str("vego"), py_tuple(vec![py_str("meat"), py_str("fish")])), (py_str("pescatarian"), py_tuple(vec![py_str("meat")])), (py_str("pescetarian"), py_tuple(vec![py_str("meat")])), (py_str("vegan"), py_tuple(vec![py_str("meat"), py_str("fish"), py_str("dairy"), py_str("eggs")])), (py_str("plant based"), py_tuple(vec![py_str("meat"), py_str("fish"), py_str("dairy"), py_str("eggs")])), (py_str("plantbased"), py_tuple(vec![py_str("meat"), py_str("fish"), py_str("dairy"), py_str("eggs")]))]));
    s_PERSON_GROUP(py_dict(vec![(py_str("children"), g_CHILD_RELATIONS()), (py_str("childs"), g_CHILD_RELATIONS()), (py_str("kids"), g_CHILD_RELATIONS()), (py_str("brood"), g_CHILD_RELATIONS()), (py_str("boys"), py_tuple(vec![py_str("son"), py_str("boy"), py_str("stepson")])), (py_str("girls"), py_tuple(vec![py_str("daughter"), py_str("girl"), py_str("stepdaughter")])), (py_str("parents"), py_tuple(vec![py_str("mum"), py_str("mother"), py_str("dad"), py_str("father"), py_str("stepmum"), py_str("stepdad"), py_str("parent")])), (py_str("siblings"), py_tuple(vec![py_str("brother"), py_str("sister"), py_str("stepbrother"), py_str("stepsister")])), (py_str("grandchildren"), py_tuple(vec![py_str("grandson"), py_str("granddaughter"), py_str("grandchild")])), (py_str("grandkids"), py_tuple(vec![py_str("grandson"), py_str("granddaughter"), py_str("grandchild")])), (py_str("grandparents"), py_tuple(vec![py_str("nan"), py_str("nana"), py_str("gran"), py_str("granny"), py_str("grandma"), py_str("grandad"), py_str("grandpa")])), (py_str("twins"), py_tuple(vec![py_str("twins"), py_str("son"), py_str("daughter")]))]));
    s_PLURAL_PEOPLE(py_tuple(vec![py_str("children"), py_str("childs"), py_str("kids"), py_str("twins"), py_str("babies")]));
    s_OWNED_BY_THE_VERB(py_tuple(vec![py_tuple(vec![py_str("drive"), py_str("car")]), py_tuple(vec![py_str("ride"), py_str("bike")])]));
    s_PREGNANT_SAY(py_tuple(vec![py_str("Noted — your {who} is pregnant, so nothing alcoholic for her. Say if you want a hand with what to cook or what to plan."), py_str("Your {who} being pregnant is worth knowing — I'll keep drink out of anything I suggest.")]));
    s_PREGNANT_WEEKS_SAY(py_tuple(vec![py_str("Noted — your {who} is {weeks} weeks pregnant, so nothing alcoholic for her. Say if you want a hand with what to cook or what to plan."), py_str("Your {who} at {weeks} weeks — I'll keep drink out of anything I suggest, and say if a meal plan would help.")]));
    s_PREGNANT_OWN_SAY(py_tuple(vec![py_str("Lovely news — {weeks} weeks. Nothing alcoholic in anything I suggest, and say if you want a hand with meals or with what to plan."), py_str("That's lovely news — {weeks} weeks in, and I'll keep the drink out of anything I suggest. Say the word if a meal plan would help.")]));
    s_LIMIT_SAY(py_tuple(vec![py_str("Noted, your {who} doesn't drink — I'll not put wine in the plan."), py_str("Good to know your {who} isn't drinking; I'll leave it out of what I suggest.")]));
    s_FEEDING_SAY(py_tuple(vec![py_str("Noted — your {who} is breastfeeding, so I'll keep the drink out of what I suggest and keep the ideas to food."), py_str("Good to know your {who} is breastfeeding; I'll leave anything with drink in it off the list.")]));
    s_STATE_WAS_SAY(py_tuple(vec![py_str("Noted - I'll take that as something that was, not something now. Tell me what you'd like a hand with and I'll pick it up from there."), py_str("Thanks for telling me. I'll keep that in the past rather than reading it as news now.")]));
    s_PET_WORDS(py_str("dog|puppy|cat|kitten|rabbit|hamster|parrot|goldfish|fish|horse|pet"));
    s_PRONOUN_PEOPLE(py_tuple(vec![py_str("she"), py_str("he"), py_str("they"), py_str("him"), py_str("her"), py_str("them")]));
    s_SAME_FACT_THEIR_OWN(py_tuple(vec![py_str("allergies"), py_str("diet"), py_str("condition"), py_str("age"), py_str("job"), py_str("name"), py_str("budget"), py_str("pets"), py_str("places"), py_str("plan")]));
    s_CONDITION_RULES_OUT(py_dict(vec![(py_str("coeliac"), py_str("gluten")), (py_str("celiac"), py_str("gluten")), (py_str("gluten intolerant"), py_str("gluten")), (py_str("lactose intolerant"), py_str("dairy"))]));
    s_CONDITION_KEEPS_DOWN(py_dict(vec![(py_str("diabetic"), py_str("sugar and white carbs")), (py_str("high cholesterol"), py_str("saturated fat and fried food")), (py_str("high blood pressure"), py_str("salt"))]));
    s_CAR_CAPS(py_dict(vec![(py_str("bmw"), py_str("BMW")), (py_str("vw"), py_str("VW")), (py_str("mg"), py_str("MG")), (py_str("gti"), py_str("GTI")), (py_str("tdi"), py_str("TDI")), (py_str("mpv"), py_str("MPV"))]));
    s_NUMBER_WORDS(py_dict(vec![(Py::Int(2), py_str("two")), (Py::Int(3), py_str("three")), (Py::Int(4), py_str("four")), (Py::Int(5), py_str("five")), (Py::Int(6), py_str("six")), (Py::Int(7), py_str("seven")), (Py::Int(8), py_str("eight")), (Py::Int(9), py_str("nine")), (Py::Int(10), py_str("ten"))]));
    s__SUBJECT_KEYS(py_dict(vec![(py_str("pet"), py_str("pets")), (py_str("allergy"), py_str("allergies")), (py_str("budget"), py_str("budget")), (py_str("job"), py_str("job")), (py_str("name"), py_str("name")), (py_str("city"), py_str("city")), (py_str("limit"), py_str("limits"))]));
    fn __data15() -> Py { py_dict(vec![(py_str("cat"), py_str("cats")), (py_str("dog"), py_str("dogs")), (py_str("puppy"), py_str("puppies")), (py_str("kitten"), py_str("kittens")), (py_str("rabbit"), py_str("rabbits")), (py_str("bunny"), py_str("bunnies")), (py_str("hamster"), py_str("hamsters")), (py_str("fish"), py_str("fish")), (py_str("bird"), py_str("birds")), (py_str("horse"), py_str("horses")), (py_str("pony"), py_str("ponies")), (py_str("parrot"), py_str("parrots")), (py_str("mouse"), py_str("mice")), (py_str("guinea pig"), py_str("guinea pigs")), (py_str("tortoise"), py_str("tortoises"))]) }
    s_PLURAL_PETS(__data15());
    s_COUNT_WORDS(py_dict(vec![(Py::Int(2), py_str("two")), (Py::Int(3), py_str("three")), (Py::Int(4), py_str("four")), (Py::Int(5), py_str("five")), (Py::Int(6), py_str("six")), (Py::Int(7), py_str("seven")), (Py::Int(8), py_str("eight")), (Py::Int(9), py_str("nine")), (Py::Int(10), py_str("ten"))]));
    s_A_SHORT_ANSWER(Py::Int(14));
    fn __data16() -> Py { py_dict(vec![(py_str("name"), py_str("name")), (py_str("job"), py_str("job")), (py_str("city"), py_str("city")), (py_str("from"), py_str("from")), (py_str("diet"), py_str("diet")), (py_str("budget"), py_str("budget")), (py_str("allergy"), py_str("allergies")), (py_str("allergies"), py_str("allergies")), (py_str("age"), py_str("age")), (py_str("pet"), py_str("pet")), (py_str("pets"), py_str("pets")), (py_str("limit"), py_str("allergies")), (py_str("height"), py_str("height"))]) }
    s_KEY_SLOT(__data16());
    s_CONCEPTS(py_dict(vec![]));
    s_POSSESSIVE(py_str("(?:(?:'s|s'|s)(?!\\s+(?:the|a|an)\\b))?"));
    s_QUOTE_LEADS(py_tuple(vec![py_str("You said: "), py_str("Your own words were: "), py_str("This is what you told me: "), py_str("You put it like this: ")]));
    s_ASKED_LEADS(py_tuple(vec![py_str("You asked: "), py_str("Your question was: "), py_str("What you said was a question: ")]));
    s_NOTHING_STARTED(py_tuple(vec![py_str("We haven't really started yet — you go first."), py_str("Nothing yet: there's nothing of yours to look back on. Where would you like to begin?"), py_str("So far it's just been hellos, so there's nothing to go back over. What's on your mind?"), py_str("We're still at the beginning — you haven't given me anything to hold onto yet.")]));
    s_POINTS_BACK_NOTHING(py_tuple(vec![py_str("There's nothing before this chat for that to be, so I've no 'before' to go on. What is it you want?"), py_str("We've only just started, so there's nothing before this to point at — say the thing itself and I'll answer it."), py_str("Nothing said yet for that to be. Tell me the thing in a word or two and I'll take it from there."), py_str("You haven't said the thing itself yet — what is it?"), py_str("‘The usual’ is what you'd say to someone who already knows. I don't yet — what is it?")]));
    s_POINTING_WORD_NOTHING(py_tuple(vec![py_str("There's nothing before this chat for that to be. What are we working on?"), py_str("I haven't got anything for that to point at yet — give me the thing in a few words and I'll answer for it."), py_str("That's the first thing you've said, so I've nothing in hand for it to mean. What is it?"), py_str("Nothing said yet for that to be: tell me the thing itself in a word or two and I'll take it from there.")]));
    s_WHAT_I_SAID_NOTHING(py_tuple(vec![py_str("Nothing yet — you haven't told me anything, so there's nothing for that to be. What was on your mind?"), py_str("You haven't said yet, so I'd be making it up. Tell me what it was and I'll keep it.")]));
    fn __data17() -> Py { py_tuple(vec![py_str("cut"), py_str("reduce"), py_str("shorten"), py_str("lower"), py_str("drop"), py_str("halve"), py_str("taper"), py_str("ease"), py_str("ease off"), py_str("keep it"), py_str("keep to"), py_str("keep the"), py_str("keep your"), py_str("go easy"), py_str("take it"), py_str("skip"), py_str("swap"), py_str("switch"), py_str("replace"), py_str("rest"), py_str("ice"), py_str("warm up"), py_str("stretch"), py_str("cut back"), py_str("fewer"), py_str("less"), py_str("lighter"), py_str("gentler"), py_str("shorter"), py_str("slower")]) }
    s_RESTRICT_OPENS(__data17());
    s_CALL_LEADS(py_tuple(vec![py_str("Yes, but {limit}. And {condition}."), py_str("Keep going, but {limit}. And {condition}."), py_str("Yes — {limit}. And {condition}."), py_str("Go on, but {limit}. And {condition}.")]));
    s_LENGTH(py_str("(?:half\\s+an\\s+hour|an\\s+hour|(?:\\d+|an?|a\\s+couple\\s+of|a\\s+few|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fifteen|twenty|thirty|forty|forty-five|fifty|sixty|ninety)\\s*(?:minutes?|mins?|hours?|hrs?))"));
    fn __data18() -> Py { py_dict(vec![(py_str("a"), Py::Int(1)), (py_str("an"), Py::Int(1)), (py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("fifteen"), Py::Int(15)), (py_str("twenty"), Py::Int(20)), (py_str("thirty"), Py::Int(30)), (py_str("forty"), Py::Int(40)), (py_str("forty-five"), Py::Int(45)), (py_str("fifty"), Py::Int(50)), (py_str("sixty"), Py::Int(60)), (py_str("ninety"), Py::Int(90))]) }
    s_WORD_NUMBERS(__data18());
    s_UNIT_SECONDS(py_dict(vec![(py_str("second"), Py::Int(1)), (py_str("sec"), Py::Int(1)), (py_str("minute"), Py::Int(60)), (py_str("min"), Py::Int(60)), (py_str("hour"), Py::Int(3600)), (py_str("hr"), Py::Int(3600))]));
    s_BUDGET_FITS(py_tuple(vec![py_str("{part}. {budget} covers that."), py_str("{part}. That fits inside {budget}."), py_str("{part}. {budget} is enough for that.")]));
    s_BUDGET_MORE(py_tuple(vec![py_str("{part}. {budget} is more than enough for the steps of it."), py_str("{part}. That is more than enough time for the steps of it, one at a time."), py_str("{part}. Nothing in it needs longer than {budget}, so there is more than enough there.")]));
    s_ANY_FIGURE(stdlib_call("re", "compile", &[py_str("\\b(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fifteen|twenty|thirty|forty|fifty|sixty|ninety|hundred)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BUDGET_TIGHT(py_tuple(vec![py_str("{part}. That wants longer than {budget}, but it is still the answer."), py_str("{part}. It takes longer than {budget}, but this is still it.")]));
    s_HELD_THREAD(py_tuple(vec![py_str("crisis"), py_str("suicide"), py_str("overdose"), py_str("call an ambulance"), py_str("999 now"), py_str("111 now"), py_str("hospital now"), py_str("call someone with you")]));
    s_GRIEF_WORDS(py_tuple(vec![py_str("died"), py_str("passed away"), py_str("funeral"), py_str("sorry for your loss"), py_str("cancer"), py_str("hospice"), py_str("in hospital"), py_str("grief"), py_str("miss him"), py_str("miss her"), py_str("miss them")]));
    s_CLOSE_PLAIN(py_tuple(vec![py_str("Fair enough — we'll leave it there. I'm here if you want to come back to it."), py_str("Alright, dropped. Say the word if you pick it up again later."), py_str("That's fine by me. I'm not going anywhere if you change your mind.")]));
    s_CLOSE_HEAVY(py_tuple(vec![py_str("Of course. You don't have to talk about it with me — I'm here either way."), py_str("That's alright. It can wait, and so can I."), py_str("Understood. Nothing more from me on it unless you bring it up.")]));
    s_CHANGE_PLAIN(py_tuple(vec![py_str("Yes. Food, films, or something to do at the weekend — pick one and I'll run with it."), py_str("Sure. Give me a subject — cooking, telly, plans — and I'll take it from there."), py_str("Of course. Tell me what you'd rather chew on: dinner, a film, the weekend?")]));
    s_CHANGE_HEAVY(py_tuple(vec![py_str("Yes, gladly. Anything you'd rather think about — a film, dinner, the weekend?"), py_str("Of course. We can leave that where it is — something light instead, like dinner or a film?"), py_str("Happily. Something light, then — a film, dinner, or the weekend?")]));
    s_CHECK_NOTHING_SAID(py_tuple(vec![py_str("Nothing came through before that one — say it again and I'll take it from there."), py_str("I don't have a line from you before this one. Go on, and I'll pick it up.")]));
    s_CHECK_A_FEELING(py_tuple(vec![py_str("Yes — I've got it."), py_str("I heard you, and I'm still here."), py_str("Yes, that came through.")]));
    s_CHECK_A_QUESTION(py_tuple(vec![py_str("Yes — I've got the question."), py_str("Yes, I have it — anything else on it?"), py_str("Got it, I have your question.")]));
    s_CHECK_PLAIN(py_tuple(vec![py_str("Yes — {said}."), py_str("Got it — {said}."), py_str("Yes, that's what I have: {said}.")]));
    s_CLOCK_SAID(py_str("(?:half (?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)|quarter (?:past|to) (?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)|(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)(?: (?:thirty|fifteen|forty-five))?|(?:1[0-2]|0?[1-9])(?:[:. ][0-5]\\d)?\\s*(?:am|pm)?|noon|midday|midnight)"));
    s_ROUTINE_SUBJECT(py_str("(?:i|we|im|i'm|i am|we're|we are)"));
    s_ROUTINE_SAY_BACK(py_dict(vec![(py_str("get up"), py_str("You're up at {at}.")), (py_str("go to bed"), py_str("You go to bed at {at}.")), (py_str("start work"), py_str("You start work at {at}.")), (py_str("finish work"), py_str("You finish work at {at}.")), (py_str("leave"), py_str("You leave at {at}.")), (py_str("get home"), py_str("You get home at {at}."))]));
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.store_lines".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ADVICE_MODULES" => Some(g_ADVICE_MODULES()),
            "AGE_FIX_WORDS" => Some(g_AGE_FIX_WORDS()),
            "AMOUNT_WORD" => Some(g_AMOUNT_WORD()),
            "ANSWERS" => Some(g_ANSWERS()),
            "ANY_FIGURE" => Some(g_ANY_FIGURE()),
            "ASKED_LEADS" => Some(g_ASKED_LEADS()),
            "AT_TIME_VALUE" => Some(g_AT_TIME_VALUE()),
            "A_SHORT_ANSWER" => Some(g_A_SHORT_ANSWER()),
            "BAD_NEWS" => Some(g_BAD_NEWS()),
            "BARE_ECHO_ONLY" => Some(g_BARE_ECHO_ONLY()),
            "BARE_SLOTS" => Some(g_BARE_SLOTS()),
            "BUDGET_FITS" => Some(g_BUDGET_FITS()),
            "BUDGET_MORE" => Some(g_BUDGET_MORE()),
            "BUDGET_TIGHT" => Some(g_BUDGET_TIGHT()),
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
            "CORRECTED_WHICH_PERSON" => Some(g_CORRECTED_WHICH_PERSON()),
            "CORRECTION_BACK" => Some(g_CORRECTION_BACK()),
            "CORRECTION_SAY" => Some(g_CORRECTION_SAY()),
            "CORRECTION_SAY_DROP" => Some(g_CORRECTION_SAY_DROP()),
            "CORRECTION_SAY_PAIR" => Some(g_CORRECTION_SAY_PAIR()),
            "COUNT_WORD" => Some(g_COUNT_WORD()),
            "COUNT_WORDS" => Some(g_COUNT_WORDS()),
            "DENIAL_SAY" => Some(g_DENIAL_SAY()),
            "DENIAL_STEMS" => Some(g_DENIAL_STEMS()),
            "DENIAL_WORDS" => Some(g_DENIAL_WORDS()),
            "DIET_FOOD" => Some(g_DIET_FOOD()),
            "DIET_RULES_OUT" => Some(g_DIET_RULES_OUT()),
            "DOG_NAMES" => Some(g_DOG_NAMES()),
            "DROP_LISTS" => Some(g_DROP_LISTS()),
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
            "IN_LAW_AFTER_TEXT" => Some(g_IN_LAW_AFTER_TEXT()),
            "KEPT_THINGS" => Some(g_KEPT_THINGS()),
            "KEY_SLOT" => Some(g_KEY_SLOT()),
            "KIN" => Some(g_KIN()),
            "KIN_ASK" => Some(g_KIN_ASK()),
            "LENGTH" => Some(g_LENGTH()),
            "LIMITS" => Some(g_LIMITS()),
            "LIMIT_SAY" => Some(g_LIMIT_SAY()),
            "LOOK_BACK_FIGURE" => Some(g_LOOK_BACK_FIGURE()),
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
            "ON_A_MEDIUM_OR_AT_A_VENUE" => Some(g_ON_A_MEDIUM_OR_AT_A_VENUE()),
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
            "PLURAL_PEOPLE" => Some(g_PLURAL_PEOPLE()),
            "PLURAL_PETS" => Some(g_PLURAL_PETS()),
            "POINTING_WORD_NOTHING" => Some(g_POINTING_WORD_NOTHING()),
            "POINTS_BACK_NOTHING" => Some(g_POINTS_BACK_NOTHING()),
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
            "SAME_FACT_THEIR_OWN" => Some(g_SAME_FACT_THEIR_OWN()),
            "SELF" => Some(g_SELF()),
            "SLOT_KEY" => Some(g_SLOT_KEY()),
            "SMALL_NAMES" => Some(g_SMALL_NAMES()),
            "SPELLING_APART" => Some(g_SPELLING_APART()),
            "STATE_WAS_SAY" => Some(g_STATE_WAS_SAY()),
            "SWAPPABLE" => Some(g_SWAPPABLE()),
            "TAIL_WINDOW" => Some(g_TAIL_WINDOW()),
            "TURNED" => Some(g_TURNED()),
            "UNIT_FAMILY" => Some(g_UNIT_FAMILY()),
            "UNIT_SECONDS" => Some(g_UNIT_SECONDS()),
            "WEEK" => Some(g_WEEK()),
            "WHAT_I_SAID_NOTHING" => Some(g_WHAT_I_SAID_NOTHING()),
            "WHERE_ASK" => Some(g_WHERE_ASK()),
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
            "AT_TIME_VALUE" => s_AT_TIME_VALUE(v),
            "A_SHORT_ANSWER" => s_A_SHORT_ANSWER(v),
            "BAD_NEWS" => s_BAD_NEWS(v),
            "BARE_ECHO_ONLY" => s_BARE_ECHO_ONLY(v),
            "BARE_SLOTS" => s_BARE_SLOTS(v),
            "BUDGET_FITS" => s_BUDGET_FITS(v),
            "BUDGET_MORE" => s_BUDGET_MORE(v),
            "BUDGET_TIGHT" => s_BUDGET_TIGHT(v),
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
            "CORRECTED_WHICH_PERSON" => s_CORRECTED_WHICH_PERSON(v),
            "CORRECTION_BACK" => s_CORRECTION_BACK(v),
            "CORRECTION_SAY" => s_CORRECTION_SAY(v),
            "CORRECTION_SAY_DROP" => s_CORRECTION_SAY_DROP(v),
            "CORRECTION_SAY_PAIR" => s_CORRECTION_SAY_PAIR(v),
            "COUNT_WORD" => s_COUNT_WORD(v),
            "COUNT_WORDS" => s_COUNT_WORDS(v),
            "DENIAL_SAY" => s_DENIAL_SAY(v),
            "DENIAL_STEMS" => s_DENIAL_STEMS(v),
            "DENIAL_WORDS" => s_DENIAL_WORDS(v),
            "DIET_FOOD" => s_DIET_FOOD(v),
            "DIET_RULES_OUT" => s_DIET_RULES_OUT(v),
            "DOG_NAMES" => s_DOG_NAMES(v),
            "DROP_LISTS" => s_DROP_LISTS(v),
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
            "IN_LAW_AFTER_TEXT" => s_IN_LAW_AFTER_TEXT(v),
            "KEPT_THINGS" => s_KEPT_THINGS(v),
            "KEY_SLOT" => s_KEY_SLOT(v),
            "KIN" => s_KIN(v),
            "KIN_ASK" => s_KIN_ASK(v),
            "LENGTH" => s_LENGTH(v),
            "LIMITS" => s_LIMITS(v),
            "LIMIT_SAY" => s_LIMIT_SAY(v),
            "LOOK_BACK_FIGURE" => s_LOOK_BACK_FIGURE(v),
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
            "ON_A_MEDIUM_OR_AT_A_VENUE" => s_ON_A_MEDIUM_OR_AT_A_VENUE(v),
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
            "PLURAL_PEOPLE" => s_PLURAL_PEOPLE(v),
            "PLURAL_PETS" => s_PLURAL_PETS(v),
            "POINTING_WORD_NOTHING" => s_POINTING_WORD_NOTHING(v),
            "POINTS_BACK_NOTHING" => s_POINTS_BACK_NOTHING(v),
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
            "SAME_FACT_THEIR_OWN" => s_SAME_FACT_THEIR_OWN(v),
            "SELF" => s_SELF(v),
            "SLOT_KEY" => s_SLOT_KEY(v),
            "SMALL_NAMES" => s_SMALL_NAMES(v),
            "SPELLING_APART" => s_SPELLING_APART(v),
            "STATE_WAS_SAY" => s_STATE_WAS_SAY(v),
            "SWAPPABLE" => s_SWAPPABLE(v),
            "TAIL_WINDOW" => s_TAIL_WINDOW(v),
            "TURNED" => s_TURNED(v),
            "UNIT_FAMILY" => s_UNIT_FAMILY(v),
            "UNIT_SECONDS" => s_UNIT_SECONDS(v),
            "WEEK" => s_WEEK(v),
            "WHAT_I_SAID_NOTHING" => s_WHAT_I_SAID_NOTHING(v),
            "WHERE_ASK" => s_WHERE_ASK(v),
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
    pyrt::xrefs::reg_module("memory.store_lines", module_obj);
    pyrt::xrefs::reg_access("memory.store_lines", "ADVICE_MODULES", g_ADVICE_MODULES, s_ADVICE_MODULES);
    pyrt::xrefs::reg_access("memory.store_lines", "AGE_FIX_WORDS", g_AGE_FIX_WORDS, s_AGE_FIX_WORDS);
    pyrt::xrefs::reg_access("memory.store_lines", "AMOUNT_WORD", g_AMOUNT_WORD, s_AMOUNT_WORD);
    pyrt::xrefs::reg_access("memory.store_lines", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("memory.store_lines", "ANY_FIGURE", g_ANY_FIGURE, s_ANY_FIGURE);
    pyrt::xrefs::reg_access("memory.store_lines", "ASKED_LEADS", g_ASKED_LEADS, s_ASKED_LEADS);
    pyrt::xrefs::reg_access("memory.store_lines", "AT_TIME_VALUE", g_AT_TIME_VALUE, s_AT_TIME_VALUE);
    pyrt::xrefs::reg_access("memory.store_lines", "A_SHORT_ANSWER", g_A_SHORT_ANSWER, s_A_SHORT_ANSWER);
    pyrt::xrefs::reg_access("memory.store_lines", "BAD_NEWS", g_BAD_NEWS, s_BAD_NEWS);
    pyrt::xrefs::reg_access("memory.store_lines", "BARE_ECHO_ONLY", g_BARE_ECHO_ONLY, s_BARE_ECHO_ONLY);
    pyrt::xrefs::reg_access("memory.store_lines", "BARE_SLOTS", g_BARE_SLOTS, s_BARE_SLOTS);
    pyrt::xrefs::reg_access("memory.store_lines", "BUDGET_FITS", g_BUDGET_FITS, s_BUDGET_FITS);
    pyrt::xrefs::reg_access("memory.store_lines", "BUDGET_MORE", g_BUDGET_MORE, s_BUDGET_MORE);
    pyrt::xrefs::reg_access("memory.store_lines", "BUDGET_TIGHT", g_BUDGET_TIGHT, s_BUDGET_TIGHT);
    pyrt::xrefs::reg_access("memory.store_lines", "CALL_LEADS", g_CALL_LEADS, s_CALL_LEADS);
    pyrt::xrefs::reg_access("memory.store_lines", "CAR_CAPS", g_CAR_CAPS, s_CAR_CAPS);
    pyrt::xrefs::reg_access("memory.store_lines", "CAT_NAMES", g_CAT_NAMES, s_CAT_NAMES);
    pyrt::xrefs::reg_access("memory.store_lines", "CHANGE_HEAVY", g_CHANGE_HEAVY, s_CHANGE_HEAVY);
    pyrt::xrefs::reg_access("memory.store_lines", "CHANGE_PLAIN", g_CHANGE_PLAIN, s_CHANGE_PLAIN);
    pyrt::xrefs::reg_access("memory.store_lines", "CHECK_A_FEELING", g_CHECK_A_FEELING, s_CHECK_A_FEELING);
    pyrt::xrefs::reg_access("memory.store_lines", "CHECK_A_QUESTION", g_CHECK_A_QUESTION, s_CHECK_A_QUESTION);
    pyrt::xrefs::reg_access("memory.store_lines", "CHECK_NOTHING_SAID", g_CHECK_NOTHING_SAID, s_CHECK_NOTHING_SAID);
    pyrt::xrefs::reg_access("memory.store_lines", "CHECK_PLAIN", g_CHECK_PLAIN, s_CHECK_PLAIN);
    pyrt::xrefs::reg_access("memory.store_lines", "CHILD", g_CHILD, s_CHILD);
    pyrt::xrefs::reg_access("memory.store_lines", "CHILD_OF", g_CHILD_OF, s_CHILD_OF);
    pyrt::xrefs::reg_access("memory.store_lines", "CHILD_RELATIONS", g_CHILD_RELATIONS, s_CHILD_RELATIONS);
    pyrt::xrefs::reg_access("memory.store_lines", "CLOCK_SAID", g_CLOCK_SAID, s_CLOCK_SAID);
    pyrt::xrefs::reg_access("memory.store_lines", "CLOSE_HEAVY", g_CLOSE_HEAVY, s_CLOSE_HEAVY);
    pyrt::xrefs::reg_access("memory.store_lines", "CLOSE_PLAIN", g_CLOSE_PLAIN, s_CLOSE_PLAIN);
    pyrt::xrefs::reg_access("memory.store_lines", "CONCEPTS", g_CONCEPTS, s_CONCEPTS);
    pyrt::xrefs::reg_access("memory.store_lines", "CONDITION_KEEPS_DOWN", g_CONDITION_KEEPS_DOWN, s_CONDITION_KEEPS_DOWN);
    pyrt::xrefs::reg_access("memory.store_lines", "CONDITION_RULES_OUT", g_CONDITION_RULES_OUT, s_CONDITION_RULES_OUT);
    pyrt::xrefs::reg_access("memory.store_lines", "CORRECTED_AGAIN", g_CORRECTED_AGAIN, s_CORRECTED_AGAIN);
    pyrt::xrefs::reg_access("memory.store_lines", "CORRECTED_AGAIN_WHO", g_CORRECTED_AGAIN_WHO, s_CORRECTED_AGAIN_WHO);
    pyrt::xrefs::reg_access("memory.store_lines", "CORRECTED_SAY", g_CORRECTED_SAY, s_CORRECTED_SAY);
    pyrt::xrefs::reg_access("memory.store_lines", "CORRECTED_WHICH_PERSON", g_CORRECTED_WHICH_PERSON, s_CORRECTED_WHICH_PERSON);
    pyrt::xrefs::reg_access("memory.store_lines", "CORRECTION_BACK", g_CORRECTION_BACK, s_CORRECTION_BACK);
    pyrt::xrefs::reg_access("memory.store_lines", "CORRECTION_SAY", g_CORRECTION_SAY, s_CORRECTION_SAY);
    pyrt::xrefs::reg_access("memory.store_lines", "CORRECTION_SAY_DROP", g_CORRECTION_SAY_DROP, s_CORRECTION_SAY_DROP);
    pyrt::xrefs::reg_access("memory.store_lines", "CORRECTION_SAY_PAIR", g_CORRECTION_SAY_PAIR, s_CORRECTION_SAY_PAIR);
    pyrt::xrefs::reg_access("memory.store_lines", "COUNT_WORD", g_COUNT_WORD, s_COUNT_WORD);
    pyrt::xrefs::reg_access("memory.store_lines", "COUNT_WORDS", g_COUNT_WORDS, s_COUNT_WORDS);
    pyrt::xrefs::reg_access("memory.store_lines", "DENIAL_SAY", g_DENIAL_SAY, s_DENIAL_SAY);
    pyrt::xrefs::reg_access("memory.store_lines", "DENIAL_STEMS", g_DENIAL_STEMS, s_DENIAL_STEMS);
    pyrt::xrefs::reg_access("memory.store_lines", "DENIAL_WORDS", g_DENIAL_WORDS, s_DENIAL_WORDS);
    pyrt::xrefs::reg_access("memory.store_lines", "DIET_FOOD", g_DIET_FOOD, s_DIET_FOOD);
    pyrt::xrefs::reg_access("memory.store_lines", "DIET_RULES_OUT", g_DIET_RULES_OUT, s_DIET_RULES_OUT);
    pyrt::xrefs::reg_access("memory.store_lines", "DOG_NAMES", g_DOG_NAMES, s_DOG_NAMES);
    pyrt::xrefs::reg_access("memory.store_lines", "DROP_LISTS", g_DROP_LISTS, s_DROP_LISTS);
    pyrt::xrefs::reg_access("memory.store_lines", "FACT_FAMILY", g_FACT_FAMILY, s_FACT_FAMILY);
    pyrt::xrefs::reg_access("memory.store_lines", "FAMILY_GROUP", g_FAMILY_GROUP, s_FAMILY_GROUP);
    pyrt::xrefs::reg_access("memory.store_lines", "FEEDING_SAY", g_FEEDING_SAY, s_FEEDING_SAY);
    pyrt::xrefs::reg_access("memory.store_lines", "FIGURE_ANSWER_LEN", g_FIGURE_ANSWER_LEN, s_FIGURE_ANSWER_LEN);
    pyrt::xrefs::reg_access("memory.store_lines", "FIGURE_NUMBER", g_FIGURE_NUMBER, s_FIGURE_NUMBER);
    pyrt::xrefs::reg_access("memory.store_lines", "FIGURE_QUALIFIER", g_FIGURE_QUALIFIER, s_FIGURE_QUALIFIER);
    pyrt::xrefs::reg_access("memory.store_lines", "FIGURE_TIME", g_FIGURE_TIME, s_FIGURE_TIME);
    pyrt::xrefs::reg_access("memory.store_lines", "FIGURE_UNITS", g_FIGURE_UNITS, s_FIGURE_UNITS);
    pyrt::xrefs::reg_access("memory.store_lines", "FOOD_PHRASES", g_FOOD_PHRASES, s_FOOD_PHRASES);
    pyrt::xrefs::reg_access("memory.store_lines", "GRIEF_WORDS", g_GRIEF_WORDS, s_GRIEF_WORDS);
    pyrt::xrefs::reg_access("memory.store_lines", "GROUP_OF", g_GROUP_OF, s_GROUP_OF);
    pyrt::xrefs::reg_access("memory.store_lines", "HAVE_ANY_WORDS", g_HAVE_ANY_WORDS, s_HAVE_ANY_WORDS);
    pyrt::xrefs::reg_access("memory.store_lines", "HELD_THREAD", g_HELD_THREAD, s_HELD_THREAD);
    pyrt::xrefs::reg_access("memory.store_lines", "HER_WORD", g_HER_WORD, s_HER_WORD);
    pyrt::xrefs::reg_access("memory.store_lines", "IN_LAW_AFTER_TEXT", g_IN_LAW_AFTER_TEXT, s_IN_LAW_AFTER_TEXT);
    pyrt::xrefs::reg_access("memory.store_lines", "KEPT_THINGS", g_KEPT_THINGS, s_KEPT_THINGS);
    pyrt::xrefs::reg_access("memory.store_lines", "KEY_SLOT", g_KEY_SLOT, s_KEY_SLOT);
    pyrt::xrefs::reg_access("memory.store_lines", "KIN", g_KIN, s_KIN);
    pyrt::xrefs::reg_access("memory.store_lines", "KIN_ASK", g_KIN_ASK, s_KIN_ASK);
    pyrt::xrefs::reg_access("memory.store_lines", "LENGTH", g_LENGTH, s_LENGTH);
    pyrt::xrefs::reg_access("memory.store_lines", "LIMITS", g_LIMITS, s_LIMITS);
    pyrt::xrefs::reg_access("memory.store_lines", "LIMIT_SAY", g_LIMIT_SAY, s_LIMIT_SAY);
    pyrt::xrefs::reg_access("memory.store_lines", "LOOK_BACK_FIGURE", g_LOOK_BACK_FIGURE, s_LOOK_BACK_FIGURE);
    pyrt::xrefs::reg_access("memory.store_lines", "MONTH_ORDER", g_MONTH_ORDER, s_MONTH_ORDER);
    pyrt::xrefs::reg_access("memory.store_lines", "NAMED_BY", g_NAMED_BY, s_NAMED_BY);
    pyrt::xrefs::reg_access("memory.store_lines", "NAME_ADVICE", g_NAME_ADVICE, s_NAME_ADVICE);
    pyrt::xrefs::reg_access("memory.store_lines", "NAME_FIX_WORDS", g_NAME_FIX_WORDS, s_NAME_FIX_WORDS);
    pyrt::xrefs::reg_access("memory.store_lines", "NAME_GIVEN_WORDS", g_NAME_GIVEN_WORDS, s_NAME_GIVEN_WORDS);
    pyrt::xrefs::reg_access("memory.store_lines", "NOTHING_STARTED", g_NOTHING_STARTED, s_NOTHING_STARTED);
    pyrt::xrefs::reg_access("memory.store_lines", "NOT_A_UNIT", g_NOT_A_UNIT, s_NOT_A_UNIT);
    pyrt::xrefs::reg_access("memory.store_lines", "NOT_ON_A_MEDIUM", g_NOT_ON_A_MEDIUM, s_NOT_ON_A_MEDIUM);
    pyrt::xrefs::reg_access("memory.store_lines", "NOT_RETRACTED", g_NOT_RETRACTED, s_NOT_RETRACTED);
    pyrt::xrefs::reg_access("memory.store_lines", "NOT_THE_PLAN", g_NOT_THE_PLAN, s_NOT_THE_PLAN);
    pyrt::xrefs::reg_access("memory.store_lines", "NO_PLANS_REPLY", g_NO_PLANS_REPLY, s_NO_PLANS_REPLY);
    pyrt::xrefs::reg_access("memory.store_lines", "NUMBER_WORDS", g_NUMBER_WORDS, s_NUMBER_WORDS);
    pyrt::xrefs::reg_access("memory.store_lines", "ON_A_MEDIUM_OR_AT_A_VENUE", g_ON_A_MEDIUM_OR_AT_A_VENUE, s_ON_A_MEDIUM_OR_AT_A_VENUE);
    pyrt::xrefs::reg_access("memory.store_lines", "OTHER_KIND_SAY", g_OTHER_KIND_SAY, s_OTHER_KIND_SAY);
    pyrt::xrefs::reg_access("memory.store_lines", "OTHER_SAID", g_OTHER_SAID, s_OTHER_SAID);
    pyrt::xrefs::reg_access("memory.store_lines", "OWNED_BY_THE_VERB", g_OWNED_BY_THE_VERB, s_OWNED_BY_THE_VERB);
    pyrt::xrefs::reg_access("memory.store_lines", "PART_HELD", g_PART_HELD, s_PART_HELD);
    pyrt::xrefs::reg_access("memory.store_lines", "PART_KEYS", g_PART_KEYS, s_PART_KEYS);
    pyrt::xrefs::reg_access("memory.store_lines", "PERSON_GROUP", g_PERSON_GROUP, s_PERSON_GROUP);
    pyrt::xrefs::reg_access("memory.store_lines", "PET_AGE_FIX_WORDS", g_PET_AGE_FIX_WORDS, s_PET_AGE_FIX_WORDS);
    pyrt::xrefs::reg_access("memory.store_lines", "PET_RENAMED_SAY", g_PET_RENAMED_SAY, s_PET_RENAMED_SAY);
    pyrt::xrefs::reg_access("memory.store_lines", "PET_WORDS", g_PET_WORDS, s_PET_WORDS);
    pyrt::xrefs::reg_access("memory.store_lines", "PLACE_NOT", g_PLACE_NOT, s_PLACE_NOT);
    pyrt::xrefs::reg_access("memory.store_lines", "PLURAL_PEOPLE", g_PLURAL_PEOPLE, s_PLURAL_PEOPLE);
    pyrt::xrefs::reg_access("memory.store_lines", "PLURAL_PETS", g_PLURAL_PETS, s_PLURAL_PETS);
    pyrt::xrefs::reg_access("memory.store_lines", "POINTING_WORD_NOTHING", g_POINTING_WORD_NOTHING, s_POINTING_WORD_NOTHING);
    pyrt::xrefs::reg_access("memory.store_lines", "POINTS_BACK_NOTHING", g_POINTS_BACK_NOTHING, s_POINTS_BACK_NOTHING);
    pyrt::xrefs::reg_access("memory.store_lines", "POSSESSIVE", g_POSSESSIVE, s_POSSESSIVE);
    pyrt::xrefs::reg_access("memory.store_lines", "PREGNANT_OWN_SAY", g_PREGNANT_OWN_SAY, s_PREGNANT_OWN_SAY);
    pyrt::xrefs::reg_access("memory.store_lines", "PREGNANT_SAY", g_PREGNANT_SAY, s_PREGNANT_SAY);
    pyrt::xrefs::reg_access("memory.store_lines", "PREGNANT_WEEKS_SAY", g_PREGNANT_WEEKS_SAY, s_PREGNANT_WEEKS_SAY);
    pyrt::xrefs::reg_access("memory.store_lines", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("memory.store_lines", "PRONOUN_FITS", g_PRONOUN_FITS, s_PRONOUN_FITS);
    pyrt::xrefs::reg_access("memory.store_lines", "PRONOUN_PEOPLE", g_PRONOUN_PEOPLE, s_PRONOUN_PEOPLE);
    pyrt::xrefs::reg_access("memory.store_lines", "QUOTE_LEADS", g_QUOTE_LEADS, s_QUOTE_LEADS);
    pyrt::xrefs::reg_access("memory.store_lines", "RECALL_HINTS", g_RECALL_HINTS, s_RECALL_HINTS);
    pyrt::xrefs::reg_access("memory.store_lines", "RELATION_ALIKE", g_RELATION_ALIKE, s_RELATION_ALIKE);
    pyrt::xrefs::reg_access("memory.store_lines", "REPEAT_ADVERB", g_REPEAT_ADVERB, s_REPEAT_ADVERB);
    pyrt::xrefs::reg_access("memory.store_lines", "RESTATED_KEYS", g_RESTATED_KEYS, s_RESTATED_KEYS);
    pyrt::xrefs::reg_access("memory.store_lines", "RESTRICT_OPENS", g_RESTRICT_OPENS, s_RESTRICT_OPENS);
    pyrt::xrefs::reg_access("memory.store_lines", "RESUME", g_RESUME, s_RESUME);
    pyrt::xrefs::reg_access("memory.store_lines", "RETRACT_SAY", g_RETRACT_SAY, s_RETRACT_SAY);
    pyrt::xrefs::reg_access("memory.store_lines", "ROUTINE_SAY_BACK", g_ROUTINE_SAY_BACK, s_ROUTINE_SAY_BACK);
    pyrt::xrefs::reg_access("memory.store_lines", "ROUTINE_SUBJECT", g_ROUTINE_SUBJECT, s_ROUTINE_SUBJECT);
    pyrt::xrefs::reg_access("memory.store_lines", "SAME_FACT_THEIR_OWN", g_SAME_FACT_THEIR_OWN, s_SAME_FACT_THEIR_OWN);
    pyrt::xrefs::reg_access("memory.store_lines", "SELF", g_SELF, s_SELF);
    pyrt::xrefs::reg_access("memory.store_lines", "SLOT_KEY", g_SLOT_KEY, s_SLOT_KEY);
    pyrt::xrefs::reg_access("memory.store_lines", "SMALL_NAMES", g_SMALL_NAMES, s_SMALL_NAMES);
    pyrt::xrefs::reg_access("memory.store_lines", "SPELLING_APART", g_SPELLING_APART, s_SPELLING_APART);
    pyrt::xrefs::reg_access("memory.store_lines", "STATE_WAS_SAY", g_STATE_WAS_SAY, s_STATE_WAS_SAY);
    pyrt::xrefs::reg_access("memory.store_lines", "SWAPPABLE", g_SWAPPABLE, s_SWAPPABLE);
    pyrt::xrefs::reg_access("memory.store_lines", "TAIL_WINDOW", g_TAIL_WINDOW, s_TAIL_WINDOW);
    pyrt::xrefs::reg_access("memory.store_lines", "TURNED", g_TURNED, s_TURNED);
    pyrt::xrefs::reg_access("memory.store_lines", "UNIT_FAMILY", g_UNIT_FAMILY, s_UNIT_FAMILY);
    pyrt::xrefs::reg_access("memory.store_lines", "UNIT_SECONDS", g_UNIT_SECONDS, s_UNIT_SECONDS);
    pyrt::xrefs::reg_access("memory.store_lines", "WEEK", g_WEEK, s_WEEK);
    pyrt::xrefs::reg_access("memory.store_lines", "WHAT_I_SAID_NOTHING", g_WHAT_I_SAID_NOTHING, s_WHAT_I_SAID_NOTHING);
    pyrt::xrefs::reg_access("memory.store_lines", "WHERE_ASK", g_WHERE_ASK, s_WHERE_ASK);
    pyrt::xrefs::reg_access("memory.store_lines", "WORD_NUMBERS", g_WORD_NUMBERS, s_WORD_NUMBERS);
    pyrt::xrefs::reg_access("memory.store_lines", "_ABOUT_WHO", g__ABOUT_WHO, s__ABOUT_WHO);
    pyrt::xrefs::reg_access("memory.store_lines", "_AMOUNTS", g__AMOUNTS, s__AMOUNTS);
    pyrt::xrefs::reg_access("memory.store_lines", "_AMOUNT_WORDS", g__AMOUNT_WORDS, s__AMOUNT_WORDS);
    pyrt::xrefs::reg_access("memory.store_lines", "_CONVERT", g__CONVERT, s__CONVERT);
    pyrt::xrefs::reg_access("memory.store_lines", "_COUNT_WORD", g__COUNT_WORD, s__COUNT_WORD);
    pyrt::xrefs::reg_access("memory.store_lines", "_FACT_SLOT", g__FACT_SLOT, s__FACT_SLOT);
    pyrt::xrefs::reg_access("memory.store_lines", "_SUBJECT_KEYS", g__SUBJECT_KEYS, s__SUBJECT_KEYS);
    pyrt::xrefs::reg_access("memory.store_lines", "_TENS_WORDS", g__TENS_WORDS, s__TENS_WORDS);
    pyrt::xrefs::reg_access("memory.store_lines", "_UNIT_SAID", g__UNIT_SAID, s__UNIT_SAID);
    pyrt::xrefs::reg_access("memory.store_lines", "_WEEKLY", g__WEEKLY, s__WEEKLY);
}

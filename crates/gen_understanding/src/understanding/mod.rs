#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
pub mod attempts;
pub mod body;
pub mod breeds;
pub mod car_glass;
pub mod cars;
pub mod confusion;
pub mod kinds;
pub mod lexicon;
pub mod normalize;
pub mod references;
pub mod senses;
pub mod topics;
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


thread_local! { static G_ABOUT_THE_BOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AFTER_BOT_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALL_ON_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANY_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_WHY_IT_IS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_FREE_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_FREE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_PERMISSION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AT_IT_STILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CLAUSE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CONDITION_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DESCRIBING_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FAULT_TO_LEND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STAND_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VALUE_IN_FRONT_OF_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOT_ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOT_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOT_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BROKEN_OF_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARRY_PASS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARRY_TOPIC: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARRY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHAT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONSTRAINT_FACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COPULA_AUX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTION_HEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTION_MARKER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTION_OPENER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DETERMINER_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DID_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOES_ALL_THEY_CAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOES_IT_ALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOES_IT_RIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EDGE_PUNCTUATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EVERYTHING_AT_ONCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_PERSON_ANYWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOLLOW_OPENER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOOD_TIME_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GREETING_PHRASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELPS_ME_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOME_LOOKBACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOME_SCOPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HYPOTHETICAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INTRODUCING_A_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEANS_ON_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOOKBACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MANNER_TALK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MAX_KEYWORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEANING_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEANING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MODAL_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MODAL_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOMENT_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOMENT_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOMENT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MORE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_NAMED_ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_NAMED_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_NAMED_VIEW_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_MANNER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_ONE_HELPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_TIME_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_PRONOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_QUESTION_TOPIC: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERMISSION_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_HEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POSSESSIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRAISED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRESENCE_ASKER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRESENCE_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRESENCE_GREETING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRESENCE_MAX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRESENCE_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRESENCE_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRESENCE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PROPERTY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PROPERTY_SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REACHING_KEYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RUNS_MYSELF_RAGGED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RUNS_THE_HOUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAME_KIND_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SECOND_JOIN_HEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SECOND_JOIN_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SECOND_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHIFTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHIFT_ACTUALLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPEAKER_SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STAND_IN_NOT_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STAND_IN_SELECTORS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STAND_IN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STOPWORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUBJECT_TYPES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TALK_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TALK_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TALK_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THANKED_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_ONE_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THIRD_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHO_ENDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORK_SCOPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORN_OUT_OF_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CLAUSE_WITH_NO_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_THING_THAT_CHANGES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_ABOUT_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SEARCH_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ABOUT_THE_BOT() -> Py { ensure_init(); G_ABOUT_THE_BOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THE_BOT(v: Py) { ensure_init(); G_ABOUT_THE_BOT.with(|c| *c.borrow_mut() = v); }
pub fn g_AFTER_BOT_FILLER() -> Py { ensure_init(); G_AFTER_BOT_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFTER_BOT_FILLER(v: Py) { ensure_init(); G_AFTER_BOT_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_ALL_ON_THEM() -> Py { ensure_init(); G_ALL_ON_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALL_ON_THEM(v: Py) { ensure_init(); G_ALL_ON_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_ANY_PRONOUN() -> Py { ensure_init(); G_ANY_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANY_PRONOUN(v: Py) { ensure_init(); G_ANY_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKING_WORDS() -> Py { ensure_init(); G_ASKING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING_WORDS(v: Py) { ensure_init(); G_ASKING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_WHY_IT_IS() -> Py { ensure_init(); G_ASKS_WHY_IT_IS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_WHY_IT_IS(v: Py) { ensure_init(); G_ASKS_WHY_IT_IS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_FREE_FILLER() -> Py { ensure_init(); G_ASK_FREE_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_FREE_FILLER(v: Py) { ensure_init(); G_ASK_FREE_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_FREE_WORDS() -> Py { ensure_init(); G_ASK_FREE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_FREE_WORDS(v: Py) { ensure_init(); G_ASK_FREE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_PERMISSION() -> Py { ensure_init(); G_ASK_PERMISSION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_PERMISSION(v: Py) { ensure_init(); G_ASK_PERMISSION.with(|c| *c.borrow_mut() = v); }
pub fn g_AT_IT_STILL() -> Py { ensure_init(); G_AT_IT_STILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AT_IT_STILL(v: Py) { ensure_init(); G_AT_IT_STILL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CLAUSE_VERB() -> Py { ensure_init(); G_A_CLAUSE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CLAUSE_VERB(v: Py) { ensure_init(); G_A_CLAUSE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CONDITION_AFTER() -> Py { ensure_init(); G_A_CONDITION_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CONDITION_AFTER(v: Py) { ensure_init(); G_A_CONDITION_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DESCRIBING_SUBJECT() -> Py { ensure_init(); G_A_DESCRIBING_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DESCRIBING_SUBJECT(v: Py) { ensure_init(); G_A_DESCRIBING_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FAULT_TO_LEND() -> Py { ensure_init(); G_A_FAULT_TO_LEND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FAULT_TO_LEND(v: Py) { ensure_init(); G_A_FAULT_TO_LEND.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STAND_IN() -> Py { ensure_init(); G_A_STAND_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STAND_IN(v: Py) { ensure_init(); G_A_STAND_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VALUE_IN_FRONT_OF_NOT() -> Py { ensure_init(); G_A_VALUE_IN_FRONT_OF_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VALUE_IN_FRONT_OF_NOT(v: Py) { ensure_init(); G_A_VALUE_IN_FRONT_OF_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_BE_VERB() -> Py { ensure_init(); G_BE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BE_VERB(v: Py) { ensure_init(); G_BE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_BOT_ASK_WORDS() -> Py { ensure_init(); G_BOT_ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOT_ASK_WORDS(v: Py) { ensure_init(); G_BOT_ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_BOT_NOUNS() -> Py { ensure_init(); G_BOT_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOT_NOUNS(v: Py) { ensure_init(); G_BOT_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_BOT_STATE() -> Py { ensure_init(); G_BOT_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOT_STATE(v: Py) { ensure_init(); G_BOT_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_BROKEN_OF_A_THING() -> Py { ensure_init(); G_BROKEN_OF_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BROKEN_OF_A_THING(v: Py) { ensure_init(); G_BROKEN_OF_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_CARRY_PASS() -> Py { ensure_init(); G_CARRY_PASS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARRY_PASS(v: Py) { ensure_init(); G_CARRY_PASS.with(|c| *c.borrow_mut() = v); }
pub fn g_CARRY_TOPIC() -> Py { ensure_init(); G_CARRY_TOPIC.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARRY_TOPIC(v: Py) { ensure_init(); G_CARRY_TOPIC.with(|c| *c.borrow_mut() = v); }
pub fn g_CARRY_WORDS() -> Py { ensure_init(); G_CARRY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARRY_WORDS(v: Py) { ensure_init(); G_CARRY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_CHAT_WORDS() -> Py { ensure_init(); G_CHAT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHAT_WORDS(v: Py) { ensure_init(); G_CHAT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_CONSTRAINT_FACT() -> Py { ensure_init(); G_CONSTRAINT_FACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONSTRAINT_FACT(v: Py) { ensure_init(); G_CONSTRAINT_FACT.with(|c| *c.borrow_mut() = v); }
pub fn g_COPULA_AUX() -> Py { ensure_init(); G_COPULA_AUX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COPULA_AUX(v: Py) { ensure_init(); G_COPULA_AUX.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTION_HEAD() -> Py { ensure_init(); G_CORRECTION_HEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTION_HEAD(v: Py) { ensure_init(); G_CORRECTION_HEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTION_MARKER() -> Py { ensure_init(); G_CORRECTION_MARKER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTION_MARKER(v: Py) { ensure_init(); G_CORRECTION_MARKER.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTION_OPENER() -> Py { ensure_init(); G_CORRECTION_OPENER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTION_OPENER(v: Py) { ensure_init(); G_CORRECTION_OPENER.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_VERBS() -> Py { ensure_init(); G_DAY_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_VERBS(v: Py) { ensure_init(); G_DAY_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_WORDS() -> Py { ensure_init(); G_DAY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_WORDS(v: Py) { ensure_init(); G_DAY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_DETERMINER_TAIL() -> Py { ensure_init(); G_DETERMINER_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DETERMINER_TAIL(v: Py) { ensure_init(); G_DETERMINER_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_DID_IT() -> Py { ensure_init(); G_DID_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DID_IT(v: Py) { ensure_init(); G_DID_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_DOES_ALL_THEY_CAN() -> Py { ensure_init(); G_DOES_ALL_THEY_CAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOES_ALL_THEY_CAN(v: Py) { ensure_init(); G_DOES_ALL_THEY_CAN.with(|c| *c.borrow_mut() = v); }
pub fn g_DOES_IT_ALL() -> Py { ensure_init(); G_DOES_IT_ALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOES_IT_ALL(v: Py) { ensure_init(); G_DOES_IT_ALL.with(|c| *c.borrow_mut() = v); }
pub fn g_DOES_IT_RIGHT() -> Py { ensure_init(); G_DOES_IT_RIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOES_IT_RIGHT(v: Py) { ensure_init(); G_DOES_IT_RIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_EDGE_PUNCTUATION() -> Py { ensure_init(); G_EDGE_PUNCTUATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EDGE_PUNCTUATION(v: Py) { ensure_init(); G_EDGE_PUNCTUATION.with(|c| *c.borrow_mut() = v); }
pub fn g_EVERYTHING_AT_ONCE() -> Py { ensure_init(); G_EVERYTHING_AT_ONCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EVERYTHING_AT_ONCE(v: Py) { ensure_init(); G_EVERYTHING_AT_ONCE.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_PERSON_ANYWHERE() -> Py { ensure_init(); G_FIRST_PERSON_ANYWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_PERSON_ANYWHERE(v: Py) { ensure_init(); G_FIRST_PERSON_ANYWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g_FOLLOW_OPENER() -> Py { ensure_init(); G_FOLLOW_OPENER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOLLOW_OPENER(v: Py) { ensure_init(); G_FOLLOW_OPENER.with(|c| *c.borrow_mut() = v); }
pub fn g_GOOD_TIME_WORDS() -> Py { ensure_init(); G_GOOD_TIME_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOOD_TIME_WORDS(v: Py) { ensure_init(); G_GOOD_TIME_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_GREETING_PHRASE() -> Py { ensure_init(); G_GREETING_PHRASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GREETING_PHRASE(v: Py) { ensure_init(); G_GREETING_PHRASE.with(|c| *c.borrow_mut() = v); }
pub fn g_HELPS_ME_NOT() -> Py { ensure_init(); G_HELPS_ME_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELPS_ME_NOT(v: Py) { ensure_init(); G_HELPS_ME_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_HOME_LOOKBACK() -> Py { ensure_init(); G_HOME_LOOKBACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOME_LOOKBACK(v: Py) { ensure_init(); G_HOME_LOOKBACK.with(|c| *c.borrow_mut() = v); }
pub fn g_HOME_SCOPE() -> Py { ensure_init(); G_HOME_SCOPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOME_SCOPE(v: Py) { ensure_init(); G_HOME_SCOPE.with(|c| *c.borrow_mut() = v); }
pub fn g_HYPOTHETICAL() -> Py { ensure_init(); G_HYPOTHETICAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HYPOTHETICAL(v: Py) { ensure_init(); G_HYPOTHETICAL.with(|c| *c.borrow_mut() = v); }
pub fn g_INTRODUCING_A_STATE() -> Py { ensure_init(); G_INTRODUCING_A_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INTRODUCING_A_STATE(v: Py) { ensure_init(); G_INTRODUCING_A_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_LEANS_ON_THEM() -> Py { ensure_init(); G_LEANS_ON_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEANS_ON_THEM(v: Py) { ensure_init(); G_LEANS_ON_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_LOOKBACK() -> Py { ensure_init(); G_LOOKBACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOOKBACK(v: Py) { ensure_init(); G_LOOKBACK.with(|c| *c.borrow_mut() = v); }
pub fn g_MANNER_TALK() -> Py { ensure_init(); G_MANNER_TALK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MANNER_TALK(v: Py) { ensure_init(); G_MANNER_TALK.with(|c| *c.borrow_mut() = v); }
pub fn g_MAX_KEYWORDS() -> Py { ensure_init(); G_MAX_KEYWORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MAX_KEYWORDS(v: Py) { ensure_init(); G_MAX_KEYWORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MEANING_TAIL() -> Py { ensure_init(); G_MEANING_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEANING_TAIL(v: Py) { ensure_init(); G_MEANING_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_MEANING_WORDS() -> Py { ensure_init(); G_MEANING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEANING_WORDS(v: Py) { ensure_init(); G_MEANING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MODAL_STATE() -> Py { ensure_init(); G_MODAL_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MODAL_STATE(v: Py) { ensure_init(); G_MODAL_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_MODAL_WORDS() -> Py { ensure_init(); G_MODAL_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MODAL_WORDS(v: Py) { ensure_init(); G_MODAL_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MOMENT_FILLER() -> Py { ensure_init(); G_MOMENT_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOMENT_FILLER(v: Py) { ensure_init(); G_MOMENT_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_MOMENT_VERBS() -> Py { ensure_init(); G_MOMENT_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOMENT_VERBS(v: Py) { ensure_init(); G_MOMENT_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_MOMENT_WORDS() -> Py { ensure_init(); G_MOMENT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOMENT_WORDS(v: Py) { ensure_init(); G_MOMENT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MORE_WORDS() -> Py { ensure_init(); G_MORE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MORE_WORDS(v: Py) { ensure_init(); G_MORE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_NAMED_ASK_WORDS() -> Py { ensure_init(); G_NOTHING_NAMED_ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_NAMED_ASK_WORDS(v: Py) { ensure_init(); G_NOTHING_NAMED_ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_NAMED_FILLER() -> Py { ensure_init(); G_NOTHING_NAMED_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_NAMED_FILLER(v: Py) { ensure_init(); G_NOTHING_NAMED_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_NAMED_VIEW_WORDS() -> Py { ensure_init(); G_NOTHING_NAMED_VIEW_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_NAMED_VIEW_WORDS(v: Py) { ensure_init(); G_NOTHING_NAMED_VIEW_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_SUBJECT() -> Py { ensure_init(); G_NOT_A_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_SUBJECT(v: Py) { ensure_init(); G_NOT_A_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_MANNER() -> Py { ensure_init(); G_NOT_MANNER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_MANNER(v: Py) { ensure_init(); G_NOT_MANNER.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_ONE_HELPS() -> Py { ensure_init(); G_NO_ONE_HELPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_ONE_HELPS(v: Py) { ensure_init(); G_NO_ONE_HELPS.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_TIME_OFF() -> Py { ensure_init(); G_NO_TIME_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_TIME_OFF(v: Py) { ensure_init(); G_NO_TIME_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_PRONOUNS() -> Py { ensure_init(); G_OTHER_PRONOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_PRONOUNS(v: Py) { ensure_init(); G_OTHER_PRONOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_QUESTION_TOPIC() -> Py { ensure_init(); G_OWN_QUESTION_TOPIC.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_QUESTION_TOPIC(v: Py) { ensure_init(); G_OWN_QUESTION_TOPIC.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_VERB() -> Py { ensure_init(); G_OWN_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_VERB(v: Py) { ensure_init(); G_OWN_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_PERMISSION_FILLER() -> Py { ensure_init(); G_PERMISSION_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERMISSION_FILLER(v: Py) { ensure_init(); G_PERMISSION_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_HEADS() -> Py { ensure_init(); G_PERSON_HEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_HEADS(v: Py) { ensure_init(); G_PERSON_HEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_STOP() -> Py { ensure_init(); G_PERSON_STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_STOP(v: Py) { ensure_init(); G_PERSON_STOP.with(|c| *c.borrow_mut() = v); }
pub fn g_POSSESSIVE() -> Py { ensure_init(); G_POSSESSIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POSSESSIVE(v: Py) { ensure_init(); G_POSSESSIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_PRAISED() -> Py { ensure_init(); G_PRAISED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRAISED(v: Py) { ensure_init(); G_PRAISED.with(|c| *c.borrow_mut() = v); }
pub fn g_PRESENCE_ASKER() -> Py { ensure_init(); G_PRESENCE_ASKER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRESENCE_ASKER(v: Py) { ensure_init(); G_PRESENCE_ASKER.with(|c| *c.borrow_mut() = v); }
pub fn g_PRESENCE_FILLER() -> Py { ensure_init(); G_PRESENCE_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRESENCE_FILLER(v: Py) { ensure_init(); G_PRESENCE_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_PRESENCE_GREETING() -> Py { ensure_init(); G_PRESENCE_GREETING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRESENCE_GREETING(v: Py) { ensure_init(); G_PRESENCE_GREETING.with(|c| *c.borrow_mut() = v); }
pub fn g_PRESENCE_MAX() -> Py { ensure_init(); G_PRESENCE_MAX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRESENCE_MAX(v: Py) { ensure_init(); G_PRESENCE_MAX.with(|c| *c.borrow_mut() = v); }
pub fn g_PRESENCE_NOT() -> Py { ensure_init(); G_PRESENCE_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRESENCE_NOT(v: Py) { ensure_init(); G_PRESENCE_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_PRESENCE_SUBJECT() -> Py { ensure_init(); G_PRESENCE_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRESENCE_SUBJECT(v: Py) { ensure_init(); G_PRESENCE_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_PRESENCE_WORD() -> Py { ensure_init(); G_PRESENCE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRESENCE_WORD(v: Py) { ensure_init(); G_PRESENCE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_PROPERTY_ASK() -> Py { ensure_init(); G_PROPERTY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PROPERTY_ASK(v: Py) { ensure_init(); G_PROPERTY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PROPERTY_SUBJECTS() -> Py { ensure_init(); G_PROPERTY_SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PROPERTY_SUBJECTS(v: Py) { ensure_init(); G_PROPERTY_SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g_REACHING_KEYS() -> Py { ensure_init(); G_REACHING_KEYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REACHING_KEYS(v: Py) { ensure_init(); G_REACHING_KEYS.with(|c| *c.borrow_mut() = v); }
pub fn g_RUNS_MYSELF_RAGGED() -> Py { ensure_init(); G_RUNS_MYSELF_RAGGED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RUNS_MYSELF_RAGGED(v: Py) { ensure_init(); G_RUNS_MYSELF_RAGGED.with(|c| *c.borrow_mut() = v); }
pub fn g_RUNS_THE_HOUSE() -> Py { ensure_init(); G_RUNS_THE_HOUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RUNS_THE_HOUSE(v: Py) { ensure_init(); G_RUNS_THE_HOUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_SAME_KIND_WORDS() -> Py { ensure_init(); G_SAME_KIND_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAME_KIND_WORDS(v: Py) { ensure_init(); G_SAME_KIND_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SECOND_JOIN_HEAD() -> Py { ensure_init(); G_SECOND_JOIN_HEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SECOND_JOIN_HEAD(v: Py) { ensure_init(); G_SECOND_JOIN_HEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_SECOND_JOIN_TAIL() -> Py { ensure_init(); G_SECOND_JOIN_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SECOND_JOIN_TAIL(v: Py) { ensure_init(); G_SECOND_JOIN_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_SECOND_PERSON() -> Py { ensure_init(); G_SECOND_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SECOND_PERSON(v: Py) { ensure_init(); G_SECOND_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_SHIFTS() -> Py { ensure_init(); G_SHIFTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHIFTS(v: Py) { ensure_init(); G_SHIFTS.with(|c| *c.borrow_mut() = v); }
pub fn g_SHIFT_ACTUALLY() -> Py { ensure_init(); G_SHIFT_ACTUALLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHIFT_ACTUALLY(v: Py) { ensure_init(); G_SHIFT_ACTUALLY.with(|c| *c.borrow_mut() = v); }
pub fn g_SPEAKER_SUBJECTS() -> Py { ensure_init(); G_SPEAKER_SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPEAKER_SUBJECTS(v: Py) { ensure_init(); G_SPEAKER_SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g_STAND_IN_NOT_THINGS() -> Py { ensure_init(); G_STAND_IN_NOT_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STAND_IN_NOT_THINGS(v: Py) { ensure_init(); G_STAND_IN_NOT_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_STAND_IN_SELECTORS() -> Py { ensure_init(); G_STAND_IN_SELECTORS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STAND_IN_SELECTORS(v: Py) { ensure_init(); G_STAND_IN_SELECTORS.with(|c| *c.borrow_mut() = v); }
pub fn g_STAND_IN_WORDS() -> Py { ensure_init(); G_STAND_IN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STAND_IN_WORDS(v: Py) { ensure_init(); G_STAND_IN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_STOPWORDS() -> Py { ensure_init(); G_STOPWORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STOPWORDS(v: Py) { ensure_init(); G_STOPWORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SUBJECT_TYPES() -> Py { ensure_init(); G_SUBJECT_TYPES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUBJECT_TYPES(v: Py) { ensure_init(); G_SUBJECT_TYPES.with(|c| *c.borrow_mut() = v); }
pub fn g_TALK_ASK() -> Py { ensure_init(); G_TALK_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TALK_ASK(v: Py) { ensure_init(); G_TALK_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_TALK_FILLER() -> Py { ensure_init(); G_TALK_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TALK_FILLER(v: Py) { ensure_init(); G_TALK_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_TALK_VERBS() -> Py { ensure_init(); G_TALK_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TALK_VERBS(v: Py) { ensure_init(); G_TALK_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_THANKED_FOR() -> Py { ensure_init(); G_THANKED_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THANKED_FOR(v: Py) { ensure_init(); G_THANKED_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_ONE_WHO() -> Py { ensure_init(); G_THE_ONE_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_ONE_WHO(v: Py) { ensure_init(); G_THE_ONE_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_THIRD_PERSON() -> Py { ensure_init(); G_THIRD_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THIRD_PERSON(v: Py) { ensure_init(); G_THIRD_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_WHO_ENDS() -> Py { ensure_init(); G_WHO_ENDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHO_ENDS(v: Py) { ensure_init(); G_WHO_ENDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WORK_SCOPE() -> Py { ensure_init(); G_WORK_SCOPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORK_SCOPE(v: Py) { ensure_init(); G_WORK_SCOPE.with(|c| *c.borrow_mut() = v); }
pub fn g_WORN_OUT_OF_A_PERSON() -> Py { ensure_init(); G_WORN_OUT_OF_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORN_OUT_OF_A_PERSON(v: Py) { ensure_init(); G_WORN_OUT_OF_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CLAUSE_WITH_NO_SUBJECT() -> Py { ensure_init(); G__A_CLAUSE_WITH_NO_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CLAUSE_WITH_NO_SUBJECT(v: Py) { ensure_init(); G__A_CLAUSE_WITH_NO_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_THING_THAT_CHANGES() -> Py { ensure_init(); G__A_THING_THAT_CHANGES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_THING_THAT_CHANGES(v: Py) { ensure_init(); G__A_THING_THAT_CHANGES.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_ABOUT_THE_THING() -> Py { ensure_init(); G__NOT_ABOUT_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_ABOUT_THE_THING(v: Py) { ensure_init(); G__NOT_ABOUT_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__SEARCH_FRAME() -> Py { ensure_init(); G__SEARCH_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SEARCH_FRAME(v: Py) { ensure_init(); G__SEARCH_FRAME.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The understanding engine: what the user said and meant. The understanding lane owns it.\n\nbot/modules/understand.py stores understand(text, memory) in memory[\"understood\"] before any\nmodule proposes a reply, so every module reads one reading of the line instead of matching its\nown keywords.\n\nThe reading is built in four steps, each in its own module:\n\n- `normalize.py`: messy typing becomes ordinary words (chat-speak, missing apostrophes,\n  obvious typos, stretched letters), and run-on lines are split into clauses.\n- `kinds.py`: what kind of line it is, the finer intent, and what it asks for.\n- `topics.py`: the topic, the things it names, and the numbers with their units.\n- `references.py`: what \"it\", \"that\", \"they\" and \"there\" point at, and whether the line\n  follows the last one.\n\n`understand()` returns a plain dict of plain data (README.md lists every key). Nothing here\nanswers; it only reads. `kind` keeps its four original values and `clean`/`words` keep their\nmeaning, so a module written against the seed's reading keeps working.\n");
    s_STOPWORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\na an the and or but if then than that this these those there here it its i me my myself\nwe our us you your yours he him his she her hers they them their theirs am is are was were be\nbeen being do does did doing done have has had having will would shall should can could may\nmight must not no nor so too very just really quite only also as at by for from in into of\noff on onto out over under up with within without about above after again against all any\nboth each few more most other some such own same what which who whom whose when where why how\nplease thanks thank hi hello hey ok okay yeah yes sure sorry oh um uh like get got to want\nneed help tell give show find know think guess\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MAX_KEYWORDS(Py::Int(14));
    s_LOOKBACK(Py::Int(4));
    s_CARRY_TOPIC(Py::Int(2));
    s_CARRY_WORDS(Py::Int(8));
    s_CARRY_PASS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("empty greeting farewell thanks ack smalltalk laugh about_bot"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THANKED_FOR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("for about of so again"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DID_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("set put turned used tried made did got added took left changed lowered\n    raised cleaned fixed swapped bought watched read told asked moved ran went came filled\n    drained switched adjusted threw cleaned"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PRAISED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("perfect perfectly great brilliant sorted solved fixed works working\n    lovely gorgeous spot marvellous marvelous amazing"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ABOUT_THE_BOT(py_str("about-the-bot"));
    s_SECOND_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("you your yours yourself u ur youre ya ye yous"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKING_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("what which who whom whose where when why"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_COPULA_AUX({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("do does did are is was were have has will would"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MODAL_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("can cant cannot could"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MODAL_STATE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("see hear listen think feel remember understand know learn sleep dream sing draw read write talk speak taste smell count help cook make"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHO_ENDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("from here there"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AFTER_BOT_FILLER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("ever always still actually really even also just then now never a an the"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BOT_STATE(op_bitor(&g_MODAL_STATE(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("have has had got get make makes think thinks know knows\n    remember remembers understand see hear listen feel feel help helps do does done\n    bot bots robot robots llm ai chatgpt gpt claude human person people real alive awake\n    busy smart clever intelligent right wrong sure ok okay there here about up online around\n    ready name named personality hobbies hobby fun weekend weekends day days week evening\n    morning night thing things lot bit much many answers answer mean means meant"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    s_BOT_NOUNS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("model models llm llms ai bot bots robot robots chatgpt gpt claude name names personality hobby hobbies"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HYPOTHETICAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("would wouldnt wouldve"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DAY_VERBS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("was is are did has have were s"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MEANING_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("mean means meant"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MEANING_TAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("by that this it there here then exactly you i"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DAY_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("day week weekend morning night evening afternoon"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SECOND_JOIN_HEAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:(?:and|also|plus|but|so|then|yeah|yes|and\\s+then|oh|and\\s+yeah)\\s+)+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SECOND_JOIN_TAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[\\s,]*(?:\\btoo\\b|\\bas\\s+well\\b|\\balso\\b|\\ban\\s+all\\b|\\band\\s+all\\b|\\band\\s+that\\b|\\baswell\\b)[\\s.!?]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CORRECTION_HEAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:(?:no|nah|sorry|actually|wait|no\\s+sorry|sorry\\s+no)[\\s,]+)?(?:(?:i\\s+mean|i\\s+meant|i\\s+did\\s+mean|mean)|(?:it'?s|it\\s+is))\\s+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CORRECTION_MARKER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:(?:no|nah|sorry|actually|wait|no\\s+sorry|sorry\\s+no)[\\s,]+)|^(?:i\\s+mean|i\\s+meant|i\\s+did\\s+mean|mean)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_VALUE_IN_FRONT_OF_NOT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\S.*\\bnot\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OWN_VERB({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("is are was were am be been being do does did done have has had having\n    will would can could should shall may might must went go goes gone going came come comes\n    coming get gets got getting make makes made making want wants wanted like likes liked need\n    needs needed think thinks thought know knows knew say says said tell tells told try tries\n    tried use uses used eat eats ate eating drink drinks drank cook cooks cooked buy buys bought\n    sell sells sold work works worked play plays played watch watches watched read reads run\n    runs ran walk walks walked live lives lived feel feels felt hurt hurts ache aches"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_FAULT_TO_LEND({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(is\\s+killing\\s+me|kills\\s+me|is\\s+playing\\s+up|is\\s+on\\s+the\\s+blink|(?:is|are)\\s+not\\s+working|(?:isnt|aren't|aint|isn't)\\s+working|has\\s+stopped|have\\s+stopped|keeps\\s+stopping|wont\\s+start|won't\\s+start|doesnt\\s+start|doesn't\\s+start|wont\\s+turn\\s+on|won't\\s+turn\\s+on|doesnt\\s+turn\\s+on|doesn't\\s+turn\\s+on|wont\\s+come\\s+on|isnt\\s+working|isn't\\s+working|is\\s+blocked|is\\s+blocked\\s+up|is\\s+leaking|is\\s+flat|is\\s+dead|is\\s+sore|is\\s+stiff|is\\s+swollen|is\\s+broken|has\\s+gone|is\\s+bad|itches|itch|hurts|hurting|hurt|aches|aching|ache)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_STAND_IN_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("cold hot warm cool freezing big small little large tiny huge\n    old new broken faulty dodgy leaky cracked chipped rusty worn dead slow fast noisy loud\n    quiet heavy light cheap expensive dear long short tall wide narrow thick thin hard soft wet\n    dry clean dirty full empty spare extra front back top bottom middle inner outer upstairs\n    downstairs left right wrong loose tight green red blue black white dark pale stained\n    missing"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_STAND_IN_SELECTORS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("other first second third last next same former latter"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_DESCRIBING_SUBJECT(op_bitor(&op_bitor(&op_bitor(&g_STAND_IN_WORDS(), &crate::understanding::lexicon::g_A_STATE()), &crate::understanding::lexicon::g_FEELINGS()), &crate::understanding::lexicon::g_A_DESCRIBING()));
    s_A_STAND_IN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(the|these|those|my|our|your|his|her|their|both)\\s+(?:([a-z][a-z'-]*)\\s+)?(ones|one)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_STAND_IN_NOT_THINGS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("winter summer spring autumn morning afternoon evening night\n    week weekend month year hour minute day today tomorrow yesterday christmas birthday\n    holiday weekend weeknight fortnight decade moment time while"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HOME_LOOKBACK(Py::Int(8));
    s_ANY_PRONOUN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::understanding::references::g_PRONOUNS(), "values", args))(&__args) }) { let mut aliases: Py = __item.clone(); for __item in py_iter_or_raise(&aliases.clone()) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SHIFTS(py_tuple(vec![py_str("never mind"), py_str("nevermind"), py_str("forget that"), py_str("forget it"), py_str("different question"), py_str("new question"), py_str("new topic"), py_str("changing the subject"), py_str("change of subject"), py_str("on another note"), py_str("moving on"), py_str("back to"), py_str("anyway"), py_str("anyways"), py_str("instead"), py_str("another thing"), py_str("unrelated"), py_str("off topic")]));
    s_SHIFT_ACTUALLY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:^|\\|)\\s*actually\\b|\\b(?:no|oh|well|sorry|and|but),? actually\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CHAT_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("talk talking talks chat chatting chats message messages conversation\nramble rambling wittering waffling going typing write writing"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TALK_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:bor(?:ing|ed)|annoy(?:ing|ed)|wast(?:e|ing)\\s+(?:your|my)\\s+time|keeping\\s+you|still\\s+(?:with|there|following|listening)|too\\s+(?:long|much)|going\\s+on|ramble|rambling|wittering|waffling)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PERSON_STOP({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:so|and|but|ok|okay|well|then|hmm|erm|um|oh|ah|right|alright|just|also)?[\\s,]*(?:should|shall|can|could)\\s+i\\s+(?:stop|quit|shut\\s+up|leave\\s+it)(?:\\s+(?:now|then|talking|chatting|going\\s+on))?[\\s.!?]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PRESENCE_SUBJECT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("you u ya your yourself me"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PRESENCE_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("up there around about here online awake alive ready with\n    free"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PRESENCE_FILLER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("still ok okay good right fine then now anyone anybody someone\n    hello hey hi hiya heya howdy yo morning evenin again mate pal buddy all are is r else\n    later soon l8r"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PRESENCE_ASKER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("anyone anybody someone"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PRESENCE_NOT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("greeting thanks farewell empty"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PRESENCE_MAX(Py::Int(5));
    s_PRESENCE_GREETING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::understanding::kinds::g_GREETINGS()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&word.clone(), &py_str(" ")))) {  __out.push(word.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GREETING_PHRASE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::understanding::kinds::g_GREETINGS()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&word.clone(), &py_str(" ")))) {  __out.push(word.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OWN_QUESTION_TOPIC(py_str("nature-and-weather"));
    s_NOTHING_NAMED_ASK_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("what which who how why when where do does did would should\n    will can could is are any anything something else more that this it for me i we to about of\n    in on with"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOTHING_NAMED_VIEW_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("think say suggest recommend advise reckon opinion advice\n    thoughts thought view take recommendation suggestion ideas idea"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOTHING_NAMED_FILLER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("so and ok okay right well but then now here first next\n    actually please pls"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BOT_ASK_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("you u ya your yours yourself"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BROKEN_OF_A_THING(py_tuple(vec![py_str("knackered"), py_str("buggered")]));
    s_WORN_OUT_OF_A_PERSON(py_dict(vec![(py_str("knackered"), py_str("knackered")), (py_str("buggered"), py_str("exhausted"))]));
    s_SPEAKER_SUBJECTS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("i"), py_str("im"), py_str("ive"), py_str("me")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BE_VERB({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("am is are was were be been being"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_CONDITION_AFTER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("if"), py_str("unless"), py_str("when"), py_str("till"), py_str("until")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_A_SUBJECT(op_bitor(&g_BE_VERB(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("a an the and but so really very totally completely\n    absolutely utterly quite pretty dead proper just now too also always still only even"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    s_OTHER_PRONOUNS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("we us our ours they them their theirs he him his she her hers\n    it its you your yours"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_EDGE_PUNCTUATION(py_str(".,!?;:'\"()"));
    s_A_CLAUSE_VERB({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("have has had having do does did done get gets got getting\n    go goes went gone make makes made making take takes took taken come comes came bet bet\n    felt feel feels leave leaves left keep keeps kept put puts"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MOMENT_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("sec secs second seconds min mins minute minutes mo moment\n    time"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MOMENT_VERBS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("got have get spare"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MOMENT_FILLER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("a an the to for chat talk and i you u me ask something question\n    quick please ta thanks can could would may might will just wanted like us we mind if\n    wondering favour favor do does did hey hi hello so well um er there"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_PERMISSION(py_tuple(vec![py_str("ask you something"), py_str("ask you a question"), py_str("ask you one"), py_str("ask a question"), py_str("ask something"), py_str("ask you quick question"), py_str("ask a quick question"), py_str("bend your ear"), py_str("pick your brains"), py_str("ask your advice"), py_str("talk to you"), py_str("talk with you"), py_str("chat to you"), py_str("chat with you"), py_str("can we talk"), py_str("could we talk"), py_str("can we chat"), py_str("have a word"), py_str("quick word"), py_str("quick question"), py_str("quick one"), py_str("ask a favour"), py_str("ask you a favour")]));
    s_PERMISSION_FILLER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("you u ya your yourself need needed want wanted mind about\n    something then now bit wee little one just really actually\n    ok okay hey hi hello so well oh ah and but or yeah yes please ta thanks love mate"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_FREE_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("free around available busy"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_FREE_FILLER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("are is you u ya to for a chat talk then now still or and quick\n    moment min minute sec second right mate please"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_MANNER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("silly friendly lovely lonely ugly early daily weekly monthly yearly\n    likely only family reply supply apply july italy belly jelly rally fly"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TALK_VERBS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("talk talking speak speaking chat chatting discuss discussing"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MORE_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("more much really very"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MANNER_TALK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("properly privately seriously openly honestly frankly quietly calmly\n    briefly quickly plainly directly bluntly gently carefully sensibly reasonably"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GOOD_TIME_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("a an the it this that is s good bad ok okay fine right wrong\n    better best time moment now for to then or"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TALK_FILLER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("can could would will may might shall should we i you u ya us to for a\n    about with now just please then"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LEANS_ON_THEM({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:everyone|everybody|they all|all of them)\\s+(?:leans?|rel(?:y|ies)|depends?|counts?|comes?|come)\\s+(?:on|to)\\s+(?:me|us)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DOES_IT_ALL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bi\\s+(?:do|handle|manage|carry|hold|sort|run|organise|organize|deal\\s+with)\\s+(?:it\\s+all|everything|the\\s+lot|all\\s+of\\s+it|the\\s+whole\\s+lot)\\b|\\bi\\s+(?:carry|hold)\\s+(?:the\\s+)?(?:whole\\s+)?(?:house|family)\\b|\\bi\\s+do\\s+everything\\s+for\\s+(?:everyone|everybody|them|the\\s+kids)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NO_TIME_OFF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bi\\s+(?:never|hardly\\s+ever|rarely)\\s+get\\s+(?:a\\s+break|five\\s+minutes)\\b|\\bi\\s+(?:never|hardly\\s+ever|rarely)\\s+get\\s+a\\s+(?:minute|moment|second)\\s+(?:to\\s+myself|for\\s+myself|off)\\b|\\bi\\s+(?:never|dont|do\\s+not)\\s+get\\s+any\\s+time\\s+(?:to\\s+myself|off|for\\s+myself)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ALL_ON_THEM({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:it'?s|its|it\\s+is|everything\\s+is|everything'?s)\\s+(?:all\\s+)?on\\s+me\\b|\\bit'?s\\s+all\\s+(?:down\\s+)?to\\s+me\\b|\\ball\\s+on\\s+my\\s+(?:own\\s+)?(?:plate|shoulders|back)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_ONE_WHO({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bi'?m\\s+(?:always\\s+|usually\\s+|often\\s+)?(?:the\\s+|the\\s+only\\s+)?one\\s+(?:who|that)\\b|\\bi\\s+am\\s+(?:always\\s+)?(?:the\\s+|the\\s+only\\s+)?one\\s+(?:who|that)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NO_ONE_HELPS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:no ?one|nobody)\\s+(?:else\\s+)?(?:helps?|helped|helps\\s+out|does\\s+anything|lifts\\s+a\\s+finger)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_EVERYTHING_AT_ONCE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:everything|everyone|everybody|all\\s+of\\s+it|it\\s+all|nothing|nobody|no ?one|the\\s+lot|the\\s+whole)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DOES_ALL_THEY_CAN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\beverything\\s+(?:i|we)\\s+(?:can|could)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THIRD_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:she|he|they|her|his|their|them|him|mum|mom|mother|dad|father|parents|sister|brother|son|daughter|partner|husband|wife|friend|colleague|boss)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FIRST_PERSON_ANYWHERE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|me|my|mine|myself|we|us|our)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS_WHY_IT_IS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:why|how\\s+come|what\\s+makes|whats\\s+making|what'?s\\s+making)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WORK_SCOPE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:at|from|in|to)\\s+work\\b|\\bwork(?:ing)?\\s+(?:all|every)\\s+(?:day|night)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HOME_SCOPE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:at|in)\\s+(?:home|the\\s+house|our\\s+house|this\\s+house)\\b|\\bhome\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DOES_IT_RIGHT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:it|everything|things)\\s+(?:all\\s+)?(?:right|perfectly|correctly)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RUNS_THE_HOUSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bi\\s+(?:run|runs|running|manage|managing)\\s+(?:the|this|our)\\s+(?:house|home|family)\\b|\\bi'?m\\s+(?:the\\s+)?only\\s+one\\s+(?:who\\s+)?(?:holds?|holding|keeps?|keeping|runs|running)\\b[^.]*\\btogether\\b|\\b(?:i'?m|i\\s+am)\\s+(?:always\\s+|usually\\s+)?(?:the\\s+)?one\\s+who\\s+(?:runs?|keeps?|holds?)\\s+(?:the|this|our)\\s+(?:house|home|family)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RUNS_MYSELF_RAGGED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bi\\s+(?:run|runs|running|work|working|push|pushing|drive|driving)\\s+myself\\s+(?:ragged|into\\s+the\\s+ground|to\\s+the\\s+bone)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HELPS_ME_NOT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:no ?one|nobody)\\s+(?:else\\s+)?(?:helps?|helped)\\s+me\\s+(?:with|at|around)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PROPERTY_SUBJECTS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\ntemperature temp temps degrees centigrade fahrenheit heat time times price cost size\nsetting weight length height age speed amount number\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SUBJECT_TYPES(py_tuple(vec![py_str("title"), py_str("name"), py_str("person"), py_str("place"), py_str("thing")]));
    s_PROPERTY_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:and|so|ok|okay|also|but)?\\s*[,:]?\\s*(?:what|which|how)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PERSON_HEADS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("i im ive ill id youre you he hes she shes they theyre we were weve\n    weve us"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_POSSESSIVE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("my his her their our your"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AT_IT_STILL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("still again now"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FOLLOW_OPENER(py_tuple(vec![py_tuple(vec![py_str("and"), py_str("what"), py_str("about")]), py_tuple(vec![py_str("what"), py_str("about")]), py_tuple(vec![py_str("how"), py_str("about")]), py_tuple(vec![py_str("and")]), py_tuple(vec![py_str("or")]), py_tuple(vec![py_str("also")]), py_tuple(vec![py_str("what")]), py_tuple(vec![py_str("how")])]));
    s_NOT_A_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("i im ive me we us you your youre u he she they theyre it its is are\n    was were am be been being do does did can could should would will shall may might have has\n    had if when while because since until though although than that this then there here and or\n    but not no"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CORRECTION_OPENER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("no nah sorry actually oops wait nope"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DETERMINER_TAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("the a an my your our their his her each every"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SAME_KIND_WORDS(py_tuple(vec![py_tuple(vec![py_str("person"), crate::understanding::topics::g_RELATIONS()]), py_tuple(vec![py_str("name"), crate::understanding::lexicon::g_COMMON_NAMES()]), py_tuple(vec![py_str("part"), crate::understanding::lexicon::g_BODY_PARTS()]), py_tuple(vec![py_str("time"), crate::understanding::normalize::g_COUNT_UNITS()])]));
    s_SUBJECT_TYPES(py_tuple(vec![py_str("title"), py_str("name"), py_str("person"), py_str("place"), py_str("thing")]));
    s_REACHING_KEYS(py_tuple(vec![py_str("topic"), py_str("emotion"), py_str("body"), py_str("numbers"), py_str("summons"), py_str("presence")]));
    s__SEARCH_FRAME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?P<polite>\\s*(?:(?:can|could|would|will)\\s+you\\s+|please\\s+|pls\\s+|just\\s+|ok(?:ay)?\\s+|so\\s+|and\\s+|go\\s+|quick\\s+)*)(?:search\\s+for|search\\s+up|look\\s+up)\\s+|^(?P<polite2>\\s*(?:(?:can|could|would|will)\\s+you\\s+|please\\s+|pls\\s+|just\\s+)*)google\\s+")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_CLAUSE_WITH_NO_SUBJECT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:whether|if|that)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_THING_THAT_CHANGES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:what\\s+time|when|hours?|opening|opens?|clos(?:es|ing)|shut|open\\s+(?:today|tonight|now)|price|prices|cost|much|fee|weather|forecast|rain|sun|snow|wind|temperature|warm|cold\\s+tomorrow|today|tonight|tomorrow|weekend|sunday|saturday|monday|holiday|tide|traffic|delays?|available|availability|in\\s+stock|stock|timetable|timetables|times|schedule|next|score|scores|result|results|kick\\s+off|live)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__NOT_ABOUT_THE_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+(?:for|on)\\s+(?:me|us)\\b.*$|\\s+(?:online|on the internet|on the web)\\b.*$|,?\\s+(?:please|thanks|ta|for me|for us)\\s*$")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_INTRODUCING_A_STATE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:(?:you can|please|just|ok|okay)\\s+)*(?:call me|my names?|my name['\\u2019]s|my name is|im called|i am called|i['\\u2019]m called|known as)\\s+(?:(?:actually|honestly|properly|officially|tbh|really|very|so|quite|a bit|a little)\\s+)*([a-z][a-z'-]{1,19})(?:\\s*[,.!]*\\s*(?:please|lol|lols|thanks|ta|cheers|then|btw|tbh|lmao|ha))?\\s*[.!]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CONSTRAINT_FACT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:my|our|the)\\s+(?:budget|price|cost|age|salary|wage|wages|income|savings|time|spending|tariff|allowance)\\s+(?:is|was|'s)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn _the_second_thing(args: &[Py]) -> Py {
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
    let mut core: Py = Py::None;
    let mut correction: Py = Py::None;
    let mut head: Py = Py::None;
    let mut line: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The name a second-thing line keeps once the join is taken off, or \"\".");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return py_str("");
    }
    correction = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CORRECTION_HEAD(), "match", args))(&__args) };
    if py_truthy(&{ let __b = correction.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _says_which_value_is_meant(args))(&__args) })) } }) {
    correction = Py::None;
    }
    head = { let __b = correction.clone(); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SECOND_JOIN_HEAD(), "match", args))(&__args) } } };
    rest = if py_truthy(&head.clone()) { slice_or_raise(&line.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&head.clone(), "end", args))(&__args) }, &Py::None, &Py::None) } else { line.clone() };
    tail = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SECOND_JOIN_TAIL(), "search", args))(&__args) };
    core = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&if py_truthy(&tail.clone()) { slice_or_raise(&rest.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&tail.clone(), "start", args))(&__args) }, &Py::None) } else { rest.clone() }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&core.clone()))) {
    return py_str("");
    }
    if py_truthy(&correction.clone()) {
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[A-Za-z']+")); __args.push(core.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }); } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(5)); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_OWN_VERB(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } }) {
    return py_str("");
    }
    return core.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&head.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&tail.clone())) } }) {
    return py_str("");
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[A-Za-z']+")); __args.push(core.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }); } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(5)) } }) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_OWN_VERB(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_str("");
    }
    return core.clone();
    Py::None
}
pub fn _says_which_value_is_meant(args: &[Py]) -> Py {
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
    let mut rest: Py = Py::None;
    let _ = py_str("Whether a correction line names the value it means, or marks itself as a correction.\n\n    \"no i mean my hip\", \"sorry, it's the 3rd\" and \"i meant the back one\" mark it themselves.\n    The bare \"its X\" does not: it is the commonest way to start the next thing said, and the\n    \"not\" with a value in front of it (\"its Preeya not Priya\") is the one markerless shape that\n    can only be a correction.\n    ");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CORRECTION_MARKER(), "match", args))(&__args) }) {
    return Py::Bool(true);
    }
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CORRECTION_HEAD(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&line.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rest.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_VALUE_IN_FRONT_OF_NOT(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_fault_to_lend(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut previous: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "previous") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: previous"))) };
    let mut found: Py = Py::None;
    let _ = py_str("What the earlier line says is wrong with its thing, or \"\".");
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = previous.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_FAULT_TO_LEND(), "search", args))(&__args) };
    return if py_truthy(&found.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) } } else { py_str("") };
    Py::None
}
pub fn _a_stand_in(args: &[Py]) -> Py {
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
    let mut lead: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("The determiner and adjective standing where a noun should be, or None.\n\n    \"the cold ones\", \"the broken one\", \"the ones at the front\": the noun is not missing from\n    the language, it is missing from this line, so nothing in it is a subject of its own.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_STAND_IN(), "finditer", args))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    lead = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&lead.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_STAND_IN_WORDS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lead.clone(), "lower", args))(&__args) })) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&lead.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_STAND_IN_SELECTORS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lead.clone(), "lower", args))(&__args) })) } }) {
    continue;
    }
    return py_dict(vec![(py_str("word"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(3)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) }), (py_str("at"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(3)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }), (py_str("end"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(3)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }), (py_str("lead"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = lead.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", args))(&__args) })]);
    }
    return Py::None;
    Py::None
}
pub fn _plural_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrase: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut _u: Py = Py::None;
    let mut head: Py = Py::None;
    let mut last: Py = Py::None;
    let mut many: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The many of a thing's name: a stand-in for several of them reads as them.");
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&phrase.clone(), "rpartition", args))(&__args) }, 3, None);
    head = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    last = __unpk1[2].clone();
    word = { let __b = last.clone(); if py_truthy(&__b) { __b } else { phrase.clone() } };
    many = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_plural_word(args))(&__args) };
    return if py_truthy(&head.clone()) { op_add(&op_add(&head.clone(), &py_str(" ")), &many.clone()) } else { many.clone() };
    Py::None
}
pub fn _a_plural_word(args: &[Py]) -> Py {
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
    let mut lower: Py = Py::None;
    lower = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ss")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lower.clone(), "endswith", args))(&__args) }) {
    return op_add(&word.clone(), &py_str("es"));
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lower.clone(), "endswith", args))(&__args) }) {
    return word.clone();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("ch"), py_str("sh"), py_str("x"), py_str("z")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lower.clone(), "endswith", args))(&__args) }) {
    return op_add(&word.clone(), &py_str("es"));
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("y")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lower.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lower.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("aeiou"), &geti(&lower.clone(), &op_neg(&Py::Int(2))))) } } } }) {
    return op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ies"));
    }
    return op_add(&word.clone(), &py_str("s"));
    Py::None
}
pub fn _singular_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrase: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut _u: Py = Py::None;
    let mut head: Py = Py::None;
    let mut last: Py = Py::None;
    let mut one: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("One of a thing the person named in the many: \"the small one\" after \"two kittens\".");
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&phrase.clone(), "rpartition", args))(&__args) }, 3, None);
    head = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    last = __unpk1[2].clone();
    word = { let __b = last.clone(); if py_truthy(&__b) { __b } else { phrase.clone() } };
    one = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_singular_word(args))(&__args) };
    return if py_truthy(&head.clone()) { op_add(&op_add(&head.clone(), &py_str(" ")), &one.clone()) } else { one.clone() };
    Py::None
}
pub fn _a_singular_word(args: &[Py]) -> Py {
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
    let mut lower: Py = Py::None;
    lower = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ies")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lower.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lower.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)) } }) {
    return op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None), &py_str("y"));
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("ses"), py_str("xes"), py_str("zes"), py_str("ches"), py_str("shes")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lower.clone(), "endswith", args))(&__args) }) {
    return slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lower.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ss")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lower.clone(), "endswith", args))(&__args) })) } }) {
    return slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    return word.clone();
    Py::None
}
pub fn _the_noun_to_lend(args: &[Py]) -> Py {
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
    let mut plural: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plural") { v.clone() }
    else { Py::Bool(false) };
    let mut earlier: Py = Py::None;
    let mut entities: Py = Py::None;
    let mut entity: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut name: Py = Py::None;
    let _ = py_str("The last thing the person named that a bare \"one\"/\"ones\" could stand for, or \"\".\n\n    A plural stand-in (\"the cold ones\") needs a thing the line said in the many, so a later line\n    that named one thing does not take the walk back off the line that named several.\n    ");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(Py::Int(3)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _user_lines(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    earlier = __item.clone();
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::normalize::normalise(args))(&__args) };
    entities = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::detect_entities(args))(&__args) }) { let mut entity: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(entity.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(entity.clone()); } } py_list(__out) })() };
    let __unpk2 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(entities.clone()); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::references::_target(args))(&__args) }, 2, None);
    name = __unpk2[0].clone();
    kind = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&kind.clone(), &py_str("thing"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(name.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_STAND_IN_NOT_THINGS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&name.clone(), "lower", args))(&__args) }))) {
    continue;
    }
    if py_truthy(&{ let __b = plural.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&name.clone(), "lower", args))(&__args) }, "split", args))(&__args) }, &op_neg(&Py::Int(1))), "endswith", args))(&__args) })) } }) {
    continue;
    }
    return name.clone();
    }
    return py_str("");
    Py::None
}
pub fn _with_the_noun_lent(args: &[Py]) -> Py {
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut noun: Py = Py::None;
    let mut stand: Py = Py::None;
    let _ = py_str("The line with the earlier line's thing standing where \"one\"/\"ones\" is, and that noun.\n\n    \"\" and \"\" when there is no stand-in or nothing of the person's own to lend it.\n    ");
    stand = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_stand_in(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&stand.clone()))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    noun = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __kws.extend(vec![("plural".to_string(), Py::Bool(py_eq(&geti(&stand.clone(), &py_str("word")), &py_str("ones"))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_noun_to_lend(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&noun.clone()))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    if py_truthy(&Py::Bool(py_eq(&geti(&stand.clone(), &py_str("word")), &py_str("ones")))) {
    noun = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(noun.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _plural_of(args))(&__args) };
    } else {
    noun = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(noun.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _singular_of(args))(&__args) };
    }
    return py_tuple(vec![op_add(&op_add(&slice_or_raise(&text.clone(), &Py::None, &geti(&stand.clone(), &py_str("at")), &Py::None), &noun.clone()), &slice_or_raise(&text.clone(), &geti(&stand.clone(), &py_str("end")), &Py::None, &Py::None)), noun.clone()]);
    Py::None
}
pub fn _a_describing_word_is_the_subject(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut read: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let mut entity: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut things: Py = Py::None;
    let mut weight: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when what the line was read as naming is a word for how a thing is, not a thing.\n\n    \"the cold ones are only the two at the front\" read as health on the word \"cold\" alone: the\n    adjective had become the subject, and a card answered the head cold it named. A word the line\n    only brushed against (\"the new one is a bit too wide\" reads on \"door\") is not a subject, and\n    the line is left as the follow-up it already was.\n    ");
    let __iter1 = py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("topic_from")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    word = __unpk2[0].clone();
    weight = __unpk2[1].clone();
    if py_truthy(&{ let __b = op_ge(&weight.clone(), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_DESCRIBING_SUBJECT(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) })) } }) {
    return Py::Bool(true);
    }
    }
    things = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("entities")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut entity: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("type")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entity.clone(), "get", args))(&__args) }, &py_str("thing")))) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entity.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); } } py_list(__out) })() };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(things.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&things.clone()) { let mut thing: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_A_DESCRIBING_SUBJECT(), &thing.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    Py::None
}
pub fn _stand_in_here(args: &[Py]) -> Py {
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
    let mut stand: Py = Py::None;
    let _ = py_str("{\"word\", \"noun\"} for a stand-in in this line, or None when the line has none.\n\n    `noun` is the thing lent to the line, so it is only ever set on the reading that took one.\n    ");
    stand = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_stand_in(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&stand.clone()))) {
    return Py::None;
    }
    return py_dict(vec![(py_str("word"), geti(&stand.clone(), &py_str("word"))), (py_str("noun"), Py::None)]);
    Py::None
}
pub fn _with_the_fault_lent(args: &[Py]) -> Py {
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut core: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut fault: Py = Py::None;
    let mut found: Py = Py::None;
    let _ = py_str("The line with the earlier line's fault on the end of its own name, or \"\".\n\n    Only the line right after the one that said the fault, only the person's own earlier line,\n    and only when this line would otherwise name a thing with nothing said of it.\n    ");
    core = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_second_thing(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&core.clone()))) {
    return py_str("");
    }
    fault = py_str("");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(Py::Int(3)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _user_lines(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    earlier = __item.clone();
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_fault_to_lend(args))(&__args) };
    if py_truthy(&found.clone()) {
    fault = found.clone();
    break;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_second_thing(args))(&__args) }))) {
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&fault.clone()))) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(core.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_FAULT_TO_LEND(), "search", args))(&__args) }) {
    return py_str("");
    }
    return op_add(&op_add(&core.clone(), &py_str(" ")), &fault.clone());
    Py::None
}
pub fn _turns(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { turns.clone() } else { py_list(vec![]) };
    Py::None
}
pub fn _user_lines(args: &[Py]) -> Py {
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
    let mut limit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "limit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: limit"))) };
    let mut lines: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The user's last `limit` lines, oldest first.");
    lines = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _turns(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    text = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "strip", args))(&__args) } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lines.clone(), "append", args))(&__args) };
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lines.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &limit.clone())) {
    break;
    }
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lines.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _pointable(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut entities: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entities") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entities"))) };
    let mut entity: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The entities a pronoun can point at.\n\n    A thing, a place, a person or a title always counts. A cap from the start of a line counts\n    only when it reads as a name: \"Two weeks in September\" starts with a cap that is not a\n    name, and \"We've got 12,000 so far\" must not become the thing a later \"it\" points at.\n    ");
    kept = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&entities.clone());
    for __item in __iter1 {
    entity = __item.clone();
    let __unpk2 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("type")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entity.clone(), "get", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entity.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }]), 2, None);
    kind = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("thing"), py_str("place"), py_str("person"), py_str("title")]), &kind.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(entity.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&kept.clone(), "append", args))(&__args) };
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("name"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&word.clone(), &py_str("'"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&crate::understanding::topics::g_COMMON_WORDS(), &word.clone())) } } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(entity.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&kept.clone(), "append", args))(&__args) };
    }
    }
    }
    return kept.clone();
    Py::None
}
pub fn _pointed_at(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut history: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "history") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: history"))) };
    let mut entities: Py = Py::None;
    let mut read: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The earlier line a pronoun in the next turn points at: the nearest one that names\n    something. A line that only leans on the line before it (\"its worse when i lie down\",\n    \"what should i do?\") does not take the thing being talked about away, so the search\n    carries on past it: \"should i see someone about it\" still means the heartburn.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(history.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    text = __item.clone();
    read = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::normalize::normalise(args))(&__args) };
    entities = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::detect_entities(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pointable(args))(&__args) };
    if py_truthy(&entities.clone()) {
    return py_tuple(vec![read.clone(), entities.clone()]);
    }
    }
    return py_tuple(vec![Py::None, py_list(vec![])]);
    Py::None
}
pub fn _the_recorded_subject(args: &[Py]) -> Py {
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
    let mut entities: Py = Py::None;
    let mut key: Py = Py::None;
    let mut line: Py = Py::None;
    let mut read: Py = Py::None;
    let mut record: Py = Py::None;
    let _ = py_str("The line the conversation's stored topic came from, when the last few name nothing.\n\n    A follow-up that points at something (\"will they survive that\") looks past the last line\n    when that line names nothing. The search only reads the last few lines, so in a long chat\n    the subject itself can have aged out of it: after five turns on tomatoes, \"im away for 2\n    weeks in august\" named nothing and \"will they survive that\" pointed at the August line\n    instead of the tomatoes. The store keeps the line the topic came from (`of`), which is\n    what a person means when nothing nearer names a thing.\n    ");
    record = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("topic")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(record.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return py_tuple(vec![Py::None, py_list(vec![])]);
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("of"), py_str("text")]));
    for __item in __iter1 {
    key = __item.clone();
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&record.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "strip", args))(&__args) })) } }) {
    continue;
    }
    read = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::normalize::normalise(args))(&__args) };
    entities = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::detect_entities(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pointable(args))(&__args) };
    if py_truthy(&entities.clone()) {
    return py_tuple(vec![read.clone(), entities.clone()]);
    }
    }
    return py_tuple(vec![Py::None, py_list(vec![])]);
    Py::None
}
pub fn _recent_topic(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut lines: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut best: Py = Py::None;
    let mut index: Py = Py::None;
    let mut key: Py = Py::None;
    let mut read: Py = Py::None;
    let mut score: Py = Py::None;
    let mut text: Py = Py::None;
    let mut topic: Py = Py::None;
    let _ = py_str("The topic the conversation has been on: the topic of the most recent line that names\n    one. \"is that enough\" names nothing, so it is still about the fitness plan two lines up;\n    a line that merely mentions something at the edge does not take the conversation over.");
    best = Py::None;
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lines.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    text = __unpk2[1].clone();
    read = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::normalize::normalise(args))(&__args) };
    let __unpk3 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); __args.push(Py::None); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::detect_entities(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::detect_topic(args))(&__args) }, 2, None);
    topic = __unpk3[0].clone();
    score = __unpk3[1].clone();
    if py_truthy(&{ let __b = topic.clone(); if !py_truthy(&__b) { __b } else { op_ge(&score.clone(), &g_CARRY_TOPIC()) } }) {
    key = py_tuple(vec![score.clone(), op_neg(&index.clone())]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&key.clone(), &geti(&best.clone(), &Py::Int(0))) } }) {
    best = py_tuple(vec![key.clone(), topic.clone()]);
    }
    }
    }
    return if py_truthy(&best.clone()) { geti(&best.clone(), &Py::Int(1)) } else { Py::None };
    Py::None
}
pub fn _changes_subject(args: &[Py]) -> Py {
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
    let mut cue: Py = Py::None;
    let mut padded: Py = Py::None;
    padded = op_add(&op_add(&py_str(" "), &geti(&line.clone(), &py_str("norm_text"))), &py_str(" "));
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SHIFTS()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&padded.clone(), &cue.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm_text"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SHIFT_ACTUALLY(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _names_a_bot_noun(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line says one of the words only the bot has: \"model\", \"hobbies\", \"bot\".");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_BOT_NOUNS(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _a_stop_in_a_chat_about_something(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut before: Py = Py::None;
    let mut last: Py = Py::None;
    let mut said: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a bare \"should i stop\" is about the chat's own thing, not about the talk.\n\n    The kinds read a stop as a question about this chat only when the whole line is the stop\n    (\"should i stop for a bit\" and \"the oil light came on, should i stop\" are already left\n    alone), and in a chat a follow-up *is* the whole line: after \"my knees hurt when i run\" the\n    line is whether to stop the running, and the bot answered \"You're not boring me. Take as\n    long as it needs.\" The chat before the line is what says which it is - the last turn this\n    reading already made of a stop, or a chat whose subject is the talk itself.\n    ");
    said = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { norm.clone() } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = norm.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) } };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PERSON_STOP(), "fullmatch", args))(&__args) }))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&memory.clone())) } }) {
    return Py::Bool(false);
    }
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    last = if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { turns.clone() } }) { geti(&turns.clone(), &op_neg(&Py::Int(1))) } else { Py::None };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("module")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &py_str("keeping"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    before = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TALK_ASK(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&before.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PERSON_STOP(), "match", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    }
    topic = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("topic")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    said = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&topic.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&said.clone(), &g_CHAT_WORDS()))) } };
    Py::None
}
pub fn _about_the_bot(args: &[Py]) -> Py {
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
    let mut read: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let mut entities: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entities") { v.clone() }
    else { py_tuple(vec![]) };
    let mut bot_at: Py = Py::None;
    let mut index: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether a line that named no topic of its own is asking the bot about itself.\n\n    Two shapes ask it. The kinds already read one of them (`about_bot`: \"do you remember what I\n    said\", \"can you see me\"), and the plainest ones - a question whose first words are about the\n    bot - read as naming nothing at all: \"do you have a name\", \"are you smart\", \"where are you\",\n    \"hows u\", \"what model are u\".\n\n    Only a question counts, and only one that opens on the bot (\"do you ...\", \"are your ...\",\n    \"where are you\") rather than naming a thing first: \"is that your coat\" is a coat, and a line\n    that names a place or a person is about what it named (\"what do you know about lisbon\").\n    It is also only ever asked once the line has named no topic of its own, so \"are you happy\"\n    keeps its feelings and \"can you help me with my tax\" keeps the tax.\n    ");
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("intent")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }, &py_str("about_bot")))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("kind")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }, &py_str("question"))); if py_truthy(&__b) { __b } else { entities.clone() } }) {
    return Py::Bool(false);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&word.clone(), &py_str("|")))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&words.clone(), &Py::Int(0)), &py_str("how"))); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_SECOND_PERSON(), &geti(&words.clone(), &Py::Int(2)))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DAY_VERBS(), &geti(&words.clone(), &Py::Int(1)))) } } } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::Int(3), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_DAY_WORDS(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    if py_truthy(&Py::Bool(py_contains(&g_SECOND_PERSON(), &geti(&words.clone(), &Py::Int(1))))) {
    if py_truthy(&Py::Bool(py_contains(&g_COPULA_AUX(), &geti(&words.clone(), &Py::Int(0))))) {
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::Int(2), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_AFTER_BOT_FILLER(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    return { let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_BOT_STATE(), &geti(&rest.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_a_bot_noun(args))(&__args) } } } } };
    }
    return { let __b = Py::Bool(py_contains(&g_MODAL_WORDS(), &geti(&words.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { { let __b = { let __cmp11: Py = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&op_lt(&Py::Int(2), &__cmp11)) { op_le(&__cmp11, &Py::Int(4)) } else { Py::Bool(false) } }; if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BOT_STATE(), &geti(&words.clone(), &Py::Int(2)))) } } } };
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_contains(&g_ASKING_WORDS(), &geti(&words.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::None, &Py::Int(4), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_SECOND_PERSON(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&geti(&words.clone(), &Py::Int(0)), &py_str("who")))) {
    return { let __b = Py::Bool(py_contains(&g_SECOND_PERSON(), &geti(&words.clone(), &op_neg(&Py::Int(1))))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WHO_ENDS(), &geti(&words.clone(), &op_neg(&Py::Int(1))))) } };
    }
    bot_at = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk3085 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk3085[0].clone(); let mut word: Py = __unpk3085[1].clone(); if py_truthy(&Py::Bool(py_contains(&g_SECOND_PERSON(), &word.clone()))) {  __out.push(index.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("next", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::Int(1), &bot_at.clone(), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_HYPOTHETICAL(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(false);
    }
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &op_add(&bot_at.clone(), &Py::Int(1)), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_AFTER_BOT_FILLER(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_MEANING_WORDS(), &geti(&rest.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&rest.clone(), &Py::Int(1), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_MEANING_TAIL(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(5))) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_BOT_STATE(), &geti(&rest.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_a_bot_noun(args))(&__args) } } } } };
    Py::None
}
pub fn _the_speaker_is_the_subject(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut between: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "between") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: between"))) };
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the words in front of a worn-out word leave the speaker as its subject.\n\n    \"im buggered\", \"i am totally buggered\" and \"i really am buggered\" are the person speaking;\n    \"we are buggered\" is not, and \"the kids have knackered me\" has the kids for its subject.\n    ");
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&between.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "isalnum", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_A_SUBJECT(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    return { let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SPEAKER_SUBJECTS(), &geti(&rest.clone(), &Py::Int(0)))) } };
    Py::None
}
pub fn _a_word_that_depends_on_the_owner(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut plain: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plain") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plain"))) };
    let mut entities: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entities") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entities"))) };
    let mut state_owner: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "state_owner") { v.clone() }
    else { Py::None };
    let mut after: Py = Py::None;
    let mut at: Py = Py::None;
    let mut back: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut between: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut entity: Py = Py::None;
    let mut low: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut out: Py = Py::None;
    let mut people: Py = Py::None;
    let mut things: Py = Py::None;
    let mut whose: Py = Py::None;
    let mut word: Py = Py::None;
    let mut word_: Py = Py::None;
    let mut words: Py = Py::None;
    let mut words_of: Py = Py::None;
    let _ = py_str("The sense of \"knackered\" that the owner in the line gives it.\n\n    The owner is the thing or person the state is said *about*: \"my washing machine is\n    knackered\" is a machine that is broken, and \"im knackered\" names nothing and is a person\n    who is worn out. A person named first (\"the kids have knackered me\") is the worn-out one:\n    nothing is broken.\n\n    It is not enough for a thing to be named somewhere earlier in the line. \"just got back from\n    a night shift, knackered\" is the person worn out, and the shift is not being called broken;\n    so the owner has to be the subject of the clause the word sits in - between the owner and\n    the word there may be nothing but the verb that carries the state and a measure or two, and\n    a comma in between ends the clause and the claim with it.\n\n    The worn-out sense is handed on as the word the rest of the bot knows for it\n    (`WORN_OUT_OF_A_PERSON`): \"buggered\" is not on any answers module's list of tired words, so\n    left as typed it reached the cheerful pool instead of the tiredness card.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&plain.clone()))) {
    return plain.clone();
    }
    things = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    people = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let __b = entities.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    entity = __item.clone();
    words_of = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entity.clone(), "get", args))(&__args) }, "split", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }); } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("type")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entity.clone(), "get", args))(&__args) }, &py_str("thing")))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words_of.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&things.clone(), "update", args))(&__args) };
    } else {
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("type")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entity.clone(), "get", args))(&__args) }, &py_str("person")))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words_of.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&people.clone(), "update", args))(&__args) };
    }
    }
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&plain.clone(), "split", args))(&__args) };
    out = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    at = __unpk3[0].clone();
    word = __unpk3[1].clone();
    bare = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_EDGE_PUNCTUATION()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "strip", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(!py_contains(&g_BROKEN_OF_A_THING(), &bare.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    continue;
    }
    between = py_list(vec![]);
    whose = Py::None;
    let __iter4 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&at.clone(), &Py::Int(1))); __args.push(op_neg(&Py::Int(1))); __args.push(op_neg(&Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter4 {
    back = __item.clone();
    earlier = geti(&words.clone(), &back.clone());
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_str(",;:?!.")) { let mut mark: Py = __item.clone(); __out.push(Py::Bool(py_contains(&earlier.clone(), &mark.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    break;
    }
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_EDGE_PUNCTUATION()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&earlier.clone(), "strip", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(py_contains(&things.clone(), &low.clone()))) {
    whose = py_str("thing");
    break;
    }
    if py_truthy(&Py::Bool(py_contains(&people.clone(), &low.clone()))) {
    whose = py_str("person");
    break;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&between.clone(), "append", args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&between.clone()) { let mut word_: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_OTHER_PRONOUNS(), &word_.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    whose = Py::None;
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_is(&whose.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(between.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_speaker_is_the_subject(args))(&__args) } } }) {
    whose = py_str("person");
    }
    }
    after = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &op_add(&at.clone(), &Py::Int(1)), &op_add(&at.clone(), &Py::Int(3)), &Py::None)) { let mut word_: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_EDGE_PUNCTUATION()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word_.clone(), "strip", args))(&__args) }, "lower", args))(&__args) }); } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(py_is(&whose.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&after.clone()) { let mut word_: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_A_CONDITION_AFTER(), &word_.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    } else {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&between.clone()) { let mut word_: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_A_CLAUSE_VERB(), &word_.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    } else {
    if py_truthy(&Py::Bool(py_eq(&whose.clone(), &py_str("thing")))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_EDGE_PUNCTUATION()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "strip", args))(&__args) }); __args.push(py_str("broken")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "replace", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    } else {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_EDGE_PUNCTUATION()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "strip", args))(&__args) }); __args.push(geti(&g_WORN_OUT_OF_A_PERSON(), &bare.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "replace", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    }
    }
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(out.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    Py::None
}
pub fn _opens_the_conversation(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut previous: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "previous") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: previous"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when there is nothing before the line to have a view about.\n\n    The first line of a conversation is one thing; a line that follows \"hi\" or \"thanks\" is the\n    same. A line that follows anything with something in it is a follow-up whatever it looks\n    like - \"im looking at two bikes\" / \"what would you suggest\" is a question about the bikes -\n    so only a person's opening line is read as an opening.\n    ");
    if py_truthy(&Py::Bool(py_is(&previous.clone(), &Py::None))) {
    return Py::Bool(true);
    }
    return op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("norm")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&previous.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&word.clone(), &py_str("|"))) } }) {  __out.push(word.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3));
    Py::None
}
pub fn asks_with_nothing_named(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line asks the bot's view of something it has not named yet.\n\n    \"what's your advice\" and \"anything you'd suggest\" were search hedges: there is nothing named\n    to advise on, and the reply that helps is to ask what it is about. \"what's your advice on my\n    cv\" names the cv and keeps its own reading.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&word.clone(), &py_str("|"))) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_BOT_ASK_WORDS(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NOTHING_NAMED_VIEW_WORDS(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    rest = op_sub(&op_sub(&op_sub(&op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_NOTHING_NAMED_ASK_WORDS()), &g_BOT_ASK_WORDS()), &g_NOTHING_NAMED_VIEW_WORDS()), &g_NOTHING_NAMED_FILLER());
    if py_truthy(&rest.clone()) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("can"), py_str("could")]), &geti(&words.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BOT_ASK_WORDS(), &geti(&words.clone(), &Py::Int(1)))) } } } }));
    Py::None
}
pub fn _presence(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut dropped_subject: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the whole line asks whether the bot is there, e.g. \"u up?\" or \"you around?\".\n\n    A greeting (\"hey there\") is a greeting, not a presence check, so the line also has to\n    address the bot (you/u) or ask after anyone at all, or leave the person out entirely the\n    way people do when they are not sure anyone is reading: \"still there?\", \"still up?\".\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&line.clone(), &py_str("norm"))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&word.clone(), &py_str("|"))) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PRESENCE_GREETING(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    words = slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_PRESENCE_MAX()) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_PRESENCE_WORD(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    asked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_PRESENCE_SUBJECT(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PRESENCE_ASKER(), &word.clone())) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    dropped_subject = op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asked.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&dropped_subject.clone())) } }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_PRESENCE_WORD(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PRESENCE_SUBJECT(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PRESENCE_ASKER(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PRESENCE_FILLER(), &word.clone())) } } } } } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _a_manner(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut back: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the word here says how the talk should go rather than naming anything.");
    word = geti(&norm.clone(), &at.clone());
    if py_truthy(&{ let __b = op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ly")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_MANNER(), &word.clone())) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g_MANNER_TALK(), &word.clone()))) {
    return Py::Bool(true);
    }
    back = op_sub(&at.clone(), &Py::Int(1));
    if py_truthy(&{ let __b = op_ge(&back.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_MORE_WORDS(), &geti(&norm.clone(), &back.clone()))) } }) {
    let __aug1 = op_sub(&back.clone(), &Py::Int(1));
    back = __aug1;
    }
    return { let __b = op_ge(&back.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_TALK_VERBS(), &geti(&norm.clone(), &back.clone()))) } };
    Py::None
}
pub fn _not_manner(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut at: Py = Py::None;
    let mut left: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The words of a line that say something, with the words of manner taken out.\n\n    An intensifier goes with the manner word after it: \"can we talk more privately\" says nothing\n    about the world either.\n    ");
    left = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(at.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_manner(args))(&__args) }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_MORE_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_lt(&op_add(&at.clone(), &Py::Int(1)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(op_add(&at.clone(), &Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_manner(args))(&__args) } } } } }) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&left.clone(), "add", args))(&__args) };
    }
    return left.clone();
    Py::None
}
pub fn a_load_carried(args: &[Py]) -> Py {
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
    let mut head: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut said: Py = Py::None;
    let mut unhelped: Py = Py::None;
    let _ = py_str("True when the line says the person is the one holding it all up for everybody else.\n\n    A reading, not a reply: `memory[\"understood\"][\"a_load_carried\"]` carries it, so one module\n    decides what a line like this is and every reader of the reading agrees.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'\" ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "strip", args))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[.!?,;:]+|\\bbut\\b|\\bthough\\b")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    piece = __item.clone();
    piece = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&piece.clone(), "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&piece.clone())); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_WHY_IT_IS(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DOES_ALL_THEY_CAN(), "search", args))(&__args) } } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WORK_SCOPE(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HOME_SCOPE(), "search", args))(&__args) })) } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DOES_IT_RIGHT(), "search", args))(&__args) }) {
    continue;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RUNS_MYSELF_RAGGED(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RUNS_THE_HOUSE(), "search", args))(&__args) } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HELPS_ME_NOT(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LEANS_ON_THEM(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DOES_IT_ALL(), "search", args))(&__args) } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NO_TIME_OFF(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ALL_ON_THEM(), "search", args))(&__args) } } }) {
    return Py::Bool(true);
    }
    unhelped = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NO_ONE_HELPS(), "search", args))(&__args) };
    if py_truthy(&unhelped.clone()) {
    head = slice_or_raise(&piece.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&unhelped.clone(), "start", args))(&__args) }, &Py::None);
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(head.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FIRST_PERSON_ANYWHERE(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(head.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THIRD_PERSON(), "search", args))(&__args) })) } }) {
    return Py::Bool(true);
    }
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_ONE_WHO(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_EVERYTHING_AT_ONCE(), "search", args))(&__args) } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _asks_for_a_moment(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut last: Py = Py::None;
    let mut norm: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line only asks whether the bot has a moment or whether they may ask.");
    norm = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&line.clone(), &py_str("norm"))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&word.clone(), &py_str("|"))) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&norm.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(9)) } }) {
    return Py::Bool(false);
    }
    bare = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    let __iter1 = py_iter_or_raise(&g_ASK_PERMISSION());
    for __item in __iter1 {
    phrase = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&bare.clone(), &phrase.clone()))) {
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrase.clone()); __args.push(py_str(" ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&bare.clone(), "replace", args))(&__args) }, "split", args))(&__args) };
    last = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&phrase.clone(), "split", args))(&__args) }, &op_neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(py_contains(&g_TALK_VERBS(), &last.clone()))) {
    rest = op_add(&py_list(vec![last.clone()]), &rest.clone());
    }
    if py_truthy(&Py::Bool(!py_truthy(&op_sub(&op_sub(&op_sub(&op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _not_manner(args))(&__args) }, &g_MOMENT_FILLER()), &g_MOMENT_WORDS()), &g_MOMENT_VERBS()), &g_PERMISSION_FILLER())))) {
    return Py::Bool(true);
    }
    continue;
    }
    }
    while py_truthy(&{ let __b = norm.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PRESENCE_GREETING(), &geti(&norm.clone(), &Py::Int(0)))) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&norm.clone(), "pop", args))(&__args) };
    }
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_GREETING_PHRASE(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&norm.clone(), &Py::None, &Py::Int(2), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) })) } }) {
    norm = slice_or_raise(&norm.clone(), &Py::Int(2), &Py::None, &Py::None);
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &g_ASK_FREE_WORDS()); if !py_truthy(&__b) { __b } else { { let __b = op_bitand(&words.clone(), &g_PRESENCE_SUBJECT()); if py_truthy(&__b) { __b } else { op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)) } } } }) {
    return Py::Bool(!py_truthy(&op_sub(&op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _not_manner(args))(&__args) }, &g_ASK_FREE_FILLER()), &g_ASK_FREE_WORDS())));
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, &py_str("the time"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = op_bitand(&op_sub(&words.clone(), &py_set(vec![py_str("time")])), &g_MOMENT_WORDS()); if py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &g_TALK_VERBS()) } })) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &g_MOMENT_WORDS()); if !py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &g_MOMENT_VERBS()) } }) {
    return Py::Bool(!py_truthy(&op_sub(&op_sub(&op_sub(&op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _not_manner(args))(&__args) }, &g_MOMENT_FILLER()), &g_MOMENT_WORDS()), &g_MOMENT_VERBS()), &py_set(vec![py_str("you"), py_str("u"), py_str("ya")]))));
    }
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &py_set(vec![py_str("time"), py_str("moment")])); if !py_truthy(&__b) { __b } else { op_le(&op_sub(&words.clone(), &g_TALK_VERBS()), &g_GOOD_TIME_WORDS()) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(6)); if !py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &g_TALK_VERBS()) } }) {
    return Py::Bool(!py_truthy(&op_sub(&op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _not_manner(args))(&__args) }, &g_TALK_FILLER()), &g_TALK_VERBS())));
    }
    return Py::Bool(false);
    Py::None
}
pub fn _keywords(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut word: Py = Py::None;
    return slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_STOPWORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "isdigit", args))(&__args) } } } } }) {  __out.push(word.clone()); } } py_list(__out) })() }, &Py::None, &g_MAX_KEYWORDS(), &Py::None);
    Py::None
}
pub fn _asks_a_property_and_nothing_else(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut low: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("\"what temperature\" and \"and how hot\", and nothing else: a property asked of the thing.\n\n    A line that states one is not this (\"he has a temperature\" is a thing about him, and the line\n    after it is still about that), so the shape of the ask is what decides it, not the words.\n    `bot/memory/topics.py` asks the same question of its own topic with its own word list, so\n    that neither package has to import the other.\n    ");
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PROPERTY_ASK(), "match", args))(&__args) }))) {
    return Py::Bool(false);
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&low.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _keywords(args))(&__args) };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(found.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_PROPERTY_SUBJECTS(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    Py::None
}
pub fn _the_chat_subject(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut chat: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "chat") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: chat"))) };
    let mut chosen: Py = Py::None;
    let mut entity: Py = Py::None;
    let mut named: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing the conversation is on, off the record the memory store keeps.\n\n    The record holds the line that started the subject and the words the chat has added to it\n    since (\"im thinking of getting a dog\", then \"cost\" and \"month\"). Its last named thing is the\n    subject, the same rule this carry uses on the earlier line.\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = chat.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return py_tuple(vec![Py::None, Py::None, py_list(vec![])]);
    }
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::normalize::normalise(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::detect_entities(args))(&__args) }) { let mut entity: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_SUBJECT_TYPES(), &geti(&entity.clone(), &py_str("type"))))) {  __out.push(entity.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return py_tuple(vec![Py::None, Py::None, py_list(vec![])]);
    }
    plain = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut entity: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("name"), py_str("title")]), &geti(&entity.clone(), &py_str("type"))))) {  __out.push(entity.clone()); } } py_list(__out) })() };
    chosen = geti(&{ let __b = plain.clone(); if py_truthy(&__b) { __b } else { named.clone() } }, &op_neg(&Py::Int(1)));
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = chat.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); } py_list(__out) })() };
    return py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&chosen.clone(), &py_str("text")), "lower", args))(&__args) }, geti(&chosen.clone(), &py_str("type")), words.clone()]);
    Py::None
}
pub fn _the_meaning_is_applied_to_what_was_said(args: &[Py]) -> Py {
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
    let mut leans_on: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "leans_on") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: leans_on"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut applied: Py = Py::None;
    let mut blob: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a \"what does that mean for my X\" is still about the line just given.\n\n    \"so what does that mean for my pay\" after an answer about a payslip is this question: the\n    thing the meaning is applied to has just been talked about, so the store says the bit of the\n    answer again. \"what does that mean for my mortgage\" after \"my back hurts\" is a question about\n    a mortgage, and a sentence about back pain applied to it would be a guess dressed as an\n    answer - so the reading does not claim it and the thing's own card keeps the turn.\n    ");
    applied = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::understanding::kinds::g_MEANT_APPLIED_TO(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&applied.clone()))) {
    return Py::Bool(true);
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&applied.clone(), "group", args))(&__args) }, "lower", args))(&__args) };
    said = py_list(vec![]);
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(leans_on.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&leans_on.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("subject")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&leans_on.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&leans_on.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(word.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "extend", args))(&__args) };
    }
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = turns.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&turns.clone(), &op_neg(&Py::Int(1)))); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "append", args))(&__args) };
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&said.clone());
    for __item in __iter1 {
    blob = __item.clone();
    let __aug2 = op_bitor(&words.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z]+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = blob.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    words = __aug2;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {  __out.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "startswith", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "startswith", args))(&__args) } } }); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _leans_on(args: &[Py]) -> Py {
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
    let mut read: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let mut ref_: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ref") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ref"))) };
    let mut carried_text: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "carried_text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: carried_text"))) };
    let mut carried: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "carried") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: carried"))) };
    let mut previous: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "previous") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: previous"))) };
    let mut own_topic: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "own_topic") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: own_topic"))) };
    let mut previous_topic: Py = if __pos.len() > 7 { __pos[7].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "previous_topic") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: previous_topic"))) };
    let mut previous_entities: Py = if __pos.len() > 8 { __pos[8].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "previous_entities") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: previous_entities"))) };
    let mut carried_is_older: Py = if __pos.len() > 9 { __pos[9].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "carried_is_older") { v.clone() }
    else { Py::Bool(false) };
    let mut chat: Py = if __pos.len() > 10 { __pos[10].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "chat") { v.clone() }
    else { Py::None };
    let mut asked_only: Py = Py::None;
    let mut chat_subject: Py = Py::None;
    let mut chat_type: Py = Py::None;
    let mut chat_words: Py = Py::None;
    let mut chosen: Py = Py::None;
    let mut entity: Py = Py::None;
    let mut named: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut subject_type: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The earlier line this one leans on, or None.\n\n    \"How long in the oven?\" and \"And if I stuff it?\" are about the last thing the person\n    said; on its own the line names no subject, so a knowledge lookup or a reply has nothing\n    to hold on to. The same is true of a line that says more about the thing already on the\n    table without naming anything new (\"He's scared of men and hides under the table\" after a\n    rescue dog). Both are read against that earlier line. A line that points at something of\n    its own (`topic_own`) is left alone: it has started something new.\n\n    The words of the earlier line are handed on as plain data, so any module can read the line\n    in context without keeping its own history (README.md, `leans_on`).\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&carried_text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CARRY_PASS(), &geti(&read.clone(), &py_str("intent")))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&geti(&ref_.clone(), &py_str("followup")))); if !py_truthy(&__b) { __b } else { own_topic.clone() } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _changes_subject(args))(&__args) }) {
    return Py::None;
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&carried.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _keywords(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::breeds::the_animal_a_breed_is(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&carried.clone(), &previous.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&carried_is_older.clone())) } }) {
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&previous.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _keywords(args))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&words.clone(), &word.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "append", args))(&__args) };
    }
    }
    }
    let __unpk2 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(chat.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_chat_subject(args))(&__args) }, 3, None);
    chat_subject = __unpk2[0].clone();
    chat_type = __unpk2[1].clone();
    chat_words = __unpk2[2].clone();
    let __iter3 = py_iter_or_raise(&chat_words.clone());
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&words.clone(), &word.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "append", args))(&__args) };
    }
    }
    words = slice_or_raise(&words.clone(), &Py::None, &g_CARRY_WORDS(), &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::None;
    }
    asked_only = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(carried_text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asks_a_property_and_nothing_else(args))(&__args) };
    words = if py_truthy(&asked_only.clone()) { { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_PROPERTY_SUBJECTS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() } } else { words.clone() };
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&previous_entities.clone()) { let mut entity: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&g_SUBJECT_TYPES(), &geti(&entity.clone(), &py_str("type")))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = asked_only.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PROPERTY_SUBJECTS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&entity.clone(), &py_str("text")), "lower", args))(&__args) })) } })) } }) {  __out.push(entity.clone()); } } py_list(__out) })() };
    plain = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut entity: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("name"), py_str("title")]), &geti(&entity.clone(), &py_str("type"))))) {  __out.push(entity.clone()); } } py_list(__out) })() };
    chosen = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = plain.clone(); if py_truthy(&__b) { __b } else { named.clone() } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut entity: Py = __item.clone(); __out.push(entity.clone()); } py_list(__out) })() }); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("next", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    subject = if py_truthy(&chosen.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&chosen.clone(), &py_str("text")), "lower", args))(&__args) } } else { Py::None };
    subject_type = if py_truthy(&chosen.clone()) { geti(&chosen.clone(), &py_str("type")) } else { Py::None };
    if py_truthy(&{ let __b = chat_subject.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&subject.clone(), &chat_subject.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&chat_words.clone(), &subject.clone())) } } } }) {
    let __unpk4 = py_unpack(&py_tuple(vec![chat_subject.clone(), chat_type.clone()]), 2, None);
    subject = __unpk4[0].clone();
    subject_type = __unpk4[1].clone();
    }
    return py_dict(vec![(py_str("text"), carried_text.clone()), (py_str("words"), words.clone()), (py_str("subject"), subject.clone()), (py_str("subject_type"), subject_type.clone()), (py_str("topic"), previous_topic.clone())]);
    Py::None
}
pub fn _about_a_person_or_a_trouble(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut after: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let _ = py_str("True when the words after a courtesy are about a person or name a trouble.");
    if py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_bitor(&crate::understanding::kinds::g_NEEDS_ANSWER(), &g_AT_IT_STILL()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::understanding::kinds::g_STILL_WRONG(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&g_PERSON_HEADS(), &geti(&after.clone(), &Py::Int(0))))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_POSSESSIVE(), &geti(&after.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&op_bitor(&op_bitor(&crate::understanding::kinds::g_OTHER_PEOPLE(), &crate::understanding::kinds::g_PERSON_NOUNS()), &crate::understanding::kinds::g_MOOD_NOUNS()), &geti(&after.clone(), &Py::Int(1)))) } } } }) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _about_something_of_its_own(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut after: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let mut entities: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut score: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the words after a thank-you are a line about something, not a sign-off.\n\n    \"my boss is a nightmare\" and \"the tap is still dripping\" name a thing and say something\n    about it, so the thank-you is a wrapper and the line inside needs the answer. \"that\n    worked\", \"that helps a lot\" and \"love\" say nothing of their own, and \"that explains the\n    boiler\" comments on the answer just given rather than asking again, so the bar is the one\n    a line's own topic has to clear.\n    ");
    if py_truthy(&{ let __b = op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::understanding::kinds::g_ADDRESS_WORDS(), &geti(&after.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&after.clone(), &Py::Int(0)), &py_str("i"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DID_IT(), &geti(&after.clone(), &Py::Int(1)))) } }) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &crate::understanding::kinds::g_NEEDS_ANSWER()))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::understanding::kinds::g_STILL_WRONG(), "search", args))(&__args) })) } }) {
    return Py::Bool(false);
    }
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&op_bitand(&words.clone(), &g_PRAISED())) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::_a_step_done(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::_the_step_went_wrong(args))(&__args) })) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &crate::understanding::kinds::g_PRAISING_VERB()); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::understanding::kinds::g_PRAISE_SAID(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::_a_fall_that_is_not_one(args))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&after.clone(), &py_str("temperature"))); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::understanding::kinds::g_SIGNING_OFF(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _about_a_person_or_a_trouble(args))(&__args) }))) {
    return Py::Bool(false);
    }
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::normalize::normalise(args))(&__args) };
    entities = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::detect_entities(args))(&__args) };
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); __args.push(Py::None); __args.push(entities.clone()); __args.push(Py::Bool(false)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::detect_topic(args))(&__args) }, 2, None);
    topic = __unpk1[0].clone();
    score = __unpk1[1].clone();
    return { let __b = Py::Bool(!py_is(&topic.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_ge(&score.clone(), &op_add(&g_CARRY_TOPIC(), &Py::Int(1))) } };
    Py::None
}
pub fn _the_new_thing(args: &[Py]) -> Py {
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
    let mut dropped: Py = Py::None;
    let mut opener: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The thing a bare follow-up names, as its words, or None.\n\n    \"what about my neck\" -> [\"my\", \"neck\"], \"and jo\" -> [\"jo\"], and None for \"what about the\n    board i chop it on\", which says a whole clause rather than a thing.\n    ");
    phrase = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = words.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    dropped = Py::Bool(true);
    while py_truthy(&{ let __b = dropped.clone(); if !py_truthy(&__b) { __b } else { phrase.clone() } }) {
    dropped = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&g_FOLLOW_OPENER());
    for __item in __iter1 {
    opener = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&phrase.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(opener.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &opener.clone()))) {
    phrase = slice_or_raise(&phrase.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(opener.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &Py::None);
    dropped = Py::Bool(true);
    break;
    }
    }
    }
    while py_truthy(&{ let __b = phrase.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CORRECTION_OPENER(), &geti(&phrase.clone(), &Py::Int(0)))) } }) {
    phrase = slice_or_raise(&phrase.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&phrase.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(5)) } }) {
    return Py::None;
    }
    let __iter2 = py_iter_or_raise(&phrase.clone());
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_NOT_A_THING(), &word.clone()))) {
    return Py::None;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrase.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::understanding::lexicon::g_COMMON_NAMES(), &word.clone())) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    return phrase.clone();
    Py::None
}
pub fn _the_same_kind(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut a: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "a") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: a"))) };
    let mut b: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "b") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: b"))) };
    let mut ch: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The kind of thing these two words are, when they are one, or None.");
    let __iter1 = py_iter_or_raise(&g_SAME_KIND_WORDS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    kind = __unpk2[0].clone();
    words = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&words.clone(), &a.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &b.clone())) } }) {
    return kind.clone();
    }
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&a.clone()) { let mut ch: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&ch.clone(), "isdigit", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&b.clone()) { let mut ch: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&ch.clone(), "isdigit", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return py_str("number");
    }
    return Py::None;
    Py::None
}
pub fn _where_the_swap_goes(args: &[Py]) -> Py {
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
    let mut phrase: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut at: Py = Py::None;
    let mut ch: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut end: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut places: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Where in the earlier line this line's own thing goes, as a slice of its words, or None.\n\n    The place is chosen by what the thing is, not by position: a number goes where the earlier\n    number is (\"180c\" -> \"200c\" in \"what is 180c in fahrenheit\"), a person where the earlier\n    person is (\"my son\" -> \"my mum\" in \"my son has a cough\"), a part of the body where the\n    earlier part is (\"my back hurts\" -> \"my neck hurts\"), a stretch of time where the earlier\n    one is (\"each week\" -> \"each month\"). Nothing else is put in place of anything: a line\n    that brings a thing of another kind is read as the line it is, which is how \"what about\n    the weather\" stays a question about the weather.\n    ");
    places = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = before.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&places.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&phrase.clone())) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&phrase.clone());
    for __item in __iter1 {
    word = __item.clone();
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(places.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    at = __unpk3[0].clone();
    earlier = __unpk3[1].clone();
    kind = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(earlier.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_same_kind(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    continue;
    }
    end = op_add(&at.clone(), &Py::Int(1));
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&earlier.clone()) { let mut ch: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&ch.clone(), "isdigit", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    while py_truthy(&{ let __b = op_lt(&end.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(places.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::understanding::normalize::g_COUNT_UNITS(), &geti(&places.clone(), &end.clone()))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("each"), py_str("per"), py_str("every")]), &geti(&places.clone(), &end.clone()))) } } } }) {
    let __aug4 = op_add(&end.clone(), &Py::Int(1));
    end = __aug4;
    }
    }
    return py_tuple(vec![at.clone(), end.clone(), kind.clone()]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_swap_is_said(args: &[Py]) -> Py {
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
    let mut phrase: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut place: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "place") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: place"))) };
    let mut _kind: Py = Py::None;
    let mut end: Py = Py::None;
    let mut places: Py = Py::None;
    let mut said: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("The earlier line with this line's own thing in it, or None when that is the same line.\n\n    The \"my\" in front of the place stays: \"what about my neck\" after \"my back hurts\" is \"my\n    neck hurts\", not \"my my neck hurts\".\n    ");
    places = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = before.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = phrase.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&places.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&place.clone(), &Py::None)) } } } }) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&place.clone(), 3, None);
    start = __unpk1[0].clone();
    end = __unpk1[1].clone();
    _kind = __unpk1[2].clone();
    if py_truthy(&{ let __b = op_gt(&start.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_DETERMINER_TAIL(), &geti(&places.clone(), &op_sub(&start.clone(), &Py::Int(1))))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DETERMINER_TAIL(), &geti(&said.clone(), &Py::Int(0)))) } } } }) {
    said = slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&slice_or_raise(&places.clone(), &Py::None, &start.clone(), &Py::None), &said.clone()), &slice_or_raise(&places.clone(), &end.clone(), &Py::None, &Py::None))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    Py::None
}
pub fn _the_line_before(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut read: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut leans: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The last line the person sent, which is the ask a bare follow-up is asking again.\n\n    It is the last turn of the memory rather than `leans_on`, because a follow-up can lean on a\n    line further back: \"im saving for a deposit\" / \"i can put by 400 a month\" / \"what about 500\n    a month\" leans on the deposit line, and the thing to swap is in the one before it.\n    ");
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) } } else { Py::None };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turn.clone(), &py_str("user")), "strip", args))(&__args) } } } } }) {
    return geti(&turn.clone(), &py_str("user"));
    }
    }
    leans = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("leans_on")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = read.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(leans.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&leans.clone(), "get", args))(&__args) }); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return geti(&leans.clone(), &py_str("text"));
    }
    return Py::None;
    Py::None
}
pub fn _asked_again(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut read: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The last ask with this line's own thing in it, or None.\n\n    Only a bare follow-up is read this way: one that leans on an earlier line and names no\n    subject of its own. The earlier line comes from the memory as plain data, so nothing here\n    keeps a history of its own.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("leans_on")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    before = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_line_before(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&before.clone(), "strip", args))(&__args) })) } }) {
    return Py::None;
    }
    phrase = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("norm")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_new_thing(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&phrase.clone()))) {
    return Py::None;
    }
    earlier = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::normalize::normalise(args))(&__args) };
    at = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&earlier.clone(), &py_str("norm"))); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _where_the_swap_goes(args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&at.clone(), &Py::None))) {
    return Py::None;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&earlier.clone(), &py_str("norm"))); __args.push(phrase.clone()); __args.push(at.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_swap_is_said(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&said.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&earlier.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) })) } }) {
    return Py::None;
    }
    return py_tuple(vec![said.clone(), geti(&at.clone(), &Py::Int(2))]);
    Py::None
}
pub fn _the_spans_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clauses: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clauses") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clauses"))) };
    let mut at: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut spans: Py = Py::None;
    let _ = py_str("Where each clause of the line sits in the whole line, as (start, end) word numbers.\n\n    The clauses are the line cut at its joins and they run end to end, so their lengths add\n    up to the line: \"my boiler is old | should i replace it\" starts at 0 and 5. A reader that\n    wants to know which noun belongs to which clause needs this, because a word's place is\n    the only thing that says which half of the line it was said in.\n    ");
    spans = py_list(vec![]);
    at = Py::Int(0);
    let __iter1 = py_iter_or_raise(&{ let __b = clauses.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    clause = __item.clone();
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![at.clone(), op_add(&at.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&spans.clone(), "append", args))(&__args) };
    let __aug2 = op_add(&at.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    at = __aug2;
    }
    return spans.clone();
    Py::None
}
pub fn _the_thing_the_ask_names(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clauses: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clauses") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clauses"))) };
    let mut focus: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "focus") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: focus"))) };
    let mut entities: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entities") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entities"))) };
    let mut ask_span: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut end: Py = Py::None;
    let mut entity: Py = Py::None;
    let mut inside: Py = Py::None;
    let mut named: Py = Py::None;
    let mut outside: Py = Py::None;
    let mut span: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("Which thing the line asks about, told apart from the scene it sets first.\n\n    One line often says two things: \"we have a cat and a baby, what should we feed the dog\"\n    sets a scene (a cat, a baby) and then asks about a dog, and \"my mum is 80 next week and we\n    want a cake, what should i bake\" sets a birthday and asks about baking. The reading already\n    cuts such a line into its clauses and marks the one that carries the ask (`focus`), but it\n    kept no word for which clause a named thing came from: `subject` is the first thing of the\n    whole line, which is the scene's thing whenever the line starts by setting one.\n\n    `subject` stays as it was, because the memory, the follow-ups and the cards that answer a\n    person's own thing are all built on it, and moving it was measured worse more often than\n    better on the deck. What is added is the half the reading was missing:\n\n    - `ask_subject`: the first thing named in the clause that carries the ask, or None.\n    - `setting`: the first thing named anywhere else in the line, or None - the scene.\n    - `subject_from`: \"ask\" when `subject` is named in the asking clause, \"setting\" when it is\n      only in the scene, None when the line names nothing.\n\n    So a module can tell \"the dog\" from \"the cat\", and a card that matched a scene word can be\n    stood down when the ask names a thing of its own. Returns (ask_subject, setting,\n    subject_from) for the `subject` it is given.\n    ");
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = entities.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut entity: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_SUBJECT_TYPES(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("type")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entity.clone(), "get", args))(&__args) }))) {  __out.push(entity.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return py_tuple(vec![Py::None, Py::None, Py::None]);
    }
    ask_span = Py::None;
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clauses.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clauses.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_spans_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("zip", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    clause = __unpk2[0].clone();
    span = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = focus.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    ask_span = span.clone();
    }
    }
    if py_truthy(&Py::Bool(py_is(&ask_span.clone(), &Py::None))) {
    return py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&geti(&named.clone(), &Py::Int(0)), &py_str("text")), "lower", args))(&__args) }, Py::None, py_str("ask")]);
    }
    let __unpk3 = py_unpack(&ask_span.clone(), 2, None);
    start = __unpk3[0].clone();
    end = __unpk3[1].clone();
    inside = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut entity: Py = __item.clone(); if py_truthy(&{ let __cmp11: Py = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("at")); __args.push(op_neg(&Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entity.clone(), "get", args))(&__args) }; if py_truthy(&op_le(&start.clone(), &__cmp11)) { op_lt(&__cmp11, &end.clone()) } else { Py::Bool(false) } }) {  __out.push(entity.clone()); } } py_list(__out) })() }); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("next", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    outside = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut entity: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp11: Py = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("at")); __args.push(op_neg(&Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entity.clone(), "get", args))(&__args) }; if py_truthy(&op_le(&start.clone(), &__cmp11)) { op_lt(&__cmp11, &end.clone()) } else { Py::Bool(false) } }))) {  __out.push(entity.clone()); } } py_list(__out) })() }); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("next", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return py_tuple(vec![if py_truthy(&inside.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&inside.clone(), &py_str("text")), "lower", args))(&__args) } } else { Py::None }, if py_truthy(&outside.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&outside.clone(), &py_str("text")), "lower", args))(&__args) } } else { Py::None }, if py_truthy(&inside.clone()) { py_str("ask") } else { py_str("setting") }]);
    Py::None
}
pub fn understand(args: &[Py]) -> Py {
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
    let mut alt: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut at_the_head: Py = Py::None;
    let mut errand: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut lent: Py = Py::None;
    let mut lent_noun: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut pairs: Py = Py::None;
    let mut read: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stand: Py = Py::None;
    let mut stated: Py = Py::None;
    let mut swapped: Py = Py::None;
    let _ = py_str("One reading of the user's line, as documented in README.md (memory[\"understood\"]).");
    read = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _read_line(args))(&__args) };
    stated = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_introduction_that_says_how_they_are(args))(&__args) };
    if py_truthy(&stated.clone()) {
    alt = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(stated.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _read_line(args))(&__args) };
    let _ = py_setitem(&alt.clone(), &py_str("text"), geti(&read.clone(), &py_str("text")));
    return alt.clone();
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_ask_inside_an_errand(args))(&__args) }, 2, None);
    errand = __unpk1[0].clone();
    at_the_head = __unpk1[1].clone();
    if py_truthy(&errand.clone()) {
    alt = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(errand.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _read_line(args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(alt.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reaches(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reaches(args))(&__args) })); if py_truthy(&__b) { __b } else { { let __b = at_the_head.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&alt.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = errand.clone(); if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_THING_THAT_CHANGES(), "search", args))(&__args) } } } } } } }) {
    let _ = py_setitem(&alt.clone(), &py_str("text"), if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) } } else { py_str("") });
    let _ = py_setitem(&alt.clone(), &py_str("fixes"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fixes")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) });
    let _ = py_setitem(&alt.clone(), &py_str("errand"), errand.clone());
    return alt.clone();
    }
    if py_truthy(&{ let __b = at_the_head.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reaches(args))(&__args) })) } }) {
    let _ = py_setitem(&read.clone(), &py_str("errand"), errand.clone());
    return read.clone();
    }
    }
    lent = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_the_fault_lent(args))(&__args) };
    if py_truthy(&lent.clone()) {
    alt = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lent.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _read_line(args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(alt.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reaches(args))(&__args) }) {
    let _ = py_setitem(&alt.clone(), &py_str("text"), geti(&read.clone(), &py_str("text")));
    return alt.clone();
    }
    }
    stand = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_stand_in(args))(&__args) };
    if py_truthy(&{ let __b = stand.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_describing_word_is_the_subject(args))(&__args) } } }) {
    let __unpk2 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_the_noun_lent(args))(&__args) }, 2, None);
    lent_noun = __unpk2[0].clone();
    noun = __unpk2[1].clone();
    alt = if py_truthy(&lent_noun.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lent_noun.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _read_line(args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&alt.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(alt.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reaches(args))(&__args) } } }) {
    let _ = py_setitem(&alt.clone(), &py_str("text"), if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { geti(&read.clone(), &py_str("text")) } else { py_str("") });
    let _ = py_setitem(&alt.clone(), &py_str("stand_in"), py_dict(vec![(py_str("word"), geti(&stand.clone(), &py_str("word"))), (py_str("noun"), noun.clone())]));
    return alt.clone();
    }
    }
    asked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asked_again(args))(&__args) };
    if py_truthy(&asked.clone()) {
    let __unpk3 = py_unpack(&asked.clone(), 2, None);
    said = __unpk3[0].clone();
    kind = __unpk3[1].clone();
    asked = if py_truthy(&{ let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("kind")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }, &py_str("question"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&kind.clone(), &py_str("number"))) } }) { said.clone() } else { Py::None };
    }
    if py_truthy(&asked.clone()) {
    alt = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(asked.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _read_line(args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&alt.clone(), "get", args))(&__args) }) {
    let _ = py_setitem(&alt.clone(), &py_str("text"), if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { geti(&read.clone(), &py_str("text")) } else { py_str("") });
    let _ = py_setitem(&alt.clone(), &py_str("fixes"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fixes")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) });
    let _ = py_setitem(&alt.clone(), &py_str("ref"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ref")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) });
    let _ = py_setitem(&alt.clone(), &py_str("leans_on"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("leans_on")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) });
    let _ = py_setitem(&alt.clone(), &py_str("asked_again"), asked.clone());
    return alt.clone();
    }
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reaches(args))(&__args) }) {
    return read.clone();
    }
    let __unpk4 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _confused_text(args))(&__args) }, 2, None);
    swapped = __unpk4[0].clone();
    pairs = __unpk4[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&swapped.clone()))) {
    return read.clone();
    }
    alt = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(swapped.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _read_line(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(alt.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reaches(args))(&__args) }))) {
    return read.clone();
    }
    let _ = py_setitem(&alt.clone(), &py_str("text"), geti(&read.clone(), &py_str("text")));
    let _ = py_setitem(&alt.clone(), &py_str("fixes"), op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fixes")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&alt.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &pairs.clone()));
    return alt.clone();
    Py::None
}
pub fn line_as_read(args: &[Py]) -> Py {
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
    let mut plain: Py = Py::None;
    let _ = py_str("The line with the repairs the engine made, for a reader that matches the line itself.\n\n    `clean` keeps the line as it was typed, and that is right for a reply that quotes the\n    person: \"you said: i need a recipie\". A reader that matches the line rather than its words\n    has the other need. \"a recipe for <dish>\" is a shape, and the shape read as typed missed\n    every line with a slip in the word \"recipe\" itself: \"i need a recipie for shortbread\" was\n    not read as naming a dish at all, so the dish table answered with a list of ten-minute\n    dinners while the shortbread card was there for the asking. `plain` is the same line with\n    the repairs this pass settled, so the dish is still the dish after a slip. A reading with\n    no plain line hands back the line as typed, so a reader never gets \"\".\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return py_str("");
    }
    plain = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plain")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plain.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&plain.clone(), "strip", args))(&__args) } } }) {
    return plain.clone();
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_ask_inside_an_errand(args: &[Py]) -> Py {
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
    let mut at_the_head: Py = Py::None;
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("The thing a \"search for ...\"/\"look up ...\" line is about, or \"\" when it carries no frame.\n\n    \"search for what time the shop closes\" -> \"what time the shop closes\";\n    \"can you look up the train times for me\" -> \"the train times\"; \"google it for me\" -> \"\" (there\n    is no thing to look up, so the line is the general one about what the bot can reach).\n    ");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SEARCH_FRAME(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,.?!")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(slice_or_raise(&line.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__NOT_ABOUT_THE_THING(), "sub", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rest.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_CLAUSE_WITH_NO_SUBJECT(), "match", args))(&__args) }) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    at_the_head = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("polite")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("polite2")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }, "strip", args))(&__args) }));
    return py_tuple(vec![rest.clone(), at_the_head.clone()]);
    Py::None
}
pub fn _reaches(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reading: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut key: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("True when a reading has anything to go on at all.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reading.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&g_REACHING_KEYS());
    for __item in __iter1 {
    key = __item.clone();
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reading.clone(), "get", args))(&__args) };
    if py_truthy(&value.clone()) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _an_introduction_that_says_how_they_are(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The state a line introduces with, or \"\".\n\n    \"call me quiet\", \"my name is lazy\", \"im called messy\": the frame says a name is coming, and\n    the word is how the person is. The store is right to refuse that word as a name (a session\n    calling somebody Quiet is worse than holding no name at all), but the line then read as an\n    introduction with nothing in it, and a phone's volume was explained to somebody choosing what\n    to be called. Read as what it says about the person instead: \"call me quiet\" is \"im quiet\".\n    Only a word from the reading's own table of how people are is taken, and only when it is the\n    whole line, so \"call me dave\" stays an introduction and \"call me when you land\" stays a ask.\n    ");
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_INTRODUCING_A_STATE(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::understanding::lexicon::g_A_DESCRIBING(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::understanding::lexicon::g_TRAITS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::understanding::lexicon::g_A_STATE(), &word.clone())) } } } }) {
    return op_add(&py_str("im "), &word.clone());
    }
    return py_str("");
    Py::None
}
pub fn _said_of_themselves(args: &[Py]) -> Py {
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
    let mut place: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "place") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: place"))) };
    let mut back: Py = Py::None;
    let mut lead: Py = Py::None;
    let _ = py_str("True when the word at `place` is how the person says they are.\n\n    \"im board\", \"im so board\", \"i feel really board\": a first-person frame, at most a filler or\n    two between it and the word. \"my board has split down the middle\" and \"the chairman of the\n    board\" are a board, and this is what tells the one from the other.\n    ");
    back = op_sub(&place.clone(), &Py::Int(1));
    while py_truthy(&{ let __b = op_ge(&back.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::understanding::lexicon::g_FEELING_FILLER(), &geti(&geti(&words.clone(), &back.clone()), &Py::Int(0)))) } }) {
    let __aug1 = op_sub(&back.clone(), &Py::Int(1));
    back = __aug1;
    }
    if py_truthy(&op_lt(&back.clone(), &Py::Int(0))) {
    return Py::Bool(false);
    }
    lead = geti(&geti(&words.clone(), &back.clone()), &Py::Int(0));
    if py_truthy(&Py::Bool(py_contains(&crate::understanding::lexicon::g_FEELING_LEAD(), &lead.clone()))) {
    return Py::Bool(true);
    }
    return { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("was"), py_str("wasnt"), py_str("wasn't"), py_str("is"), py_str("isnt"), py_str("isn't"), py_str("get"), py_str("got")]), &lead.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&back.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("i"), py_str("im"), py_str("i'm"), py_str("iam")]), &geti(&geti(&words.clone(), &op_sub(&back.clone(), &Py::Int(1))), &Py::Int(0)))) } } } };
    Py::None
}
pub fn _confused_text(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut end: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut meant: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut pairs: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut place: Py = Py::None;
    let mut start: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line with every sound-alike word put right, and the pairs that were changed.\n\n    \"\" and [] when the line holds none of them, which is nearly every line, so the second\n    chance costs one pass over a word list. Every word is looked at once, in the place it\n    stands: a word is never bent twice (\"quiet\" to \"quite\" and back again), and a word that is\n    only sometimes the other word is bent only where it is.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "strip", args))(&__args) })) } }) {
    return py_tuple(vec![py_str(""), py_list(vec![])]);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[A-Za-z']+")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut match_: Py = __item.clone(); __out.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_tuple(vec![py_str(""), py_list(vec![])]);
    }
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![]), Py::Int(0)]), 3, None);
    pieces = __unpk1[0].clone();
    pairs = __unpk1[1].clone();
    cut = __unpk1[2].clone();
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    place = __unpk3[0].clone();
    let __unpk4 = py_unpack(&__unpk3[1].clone(), 3, None);
    word = __unpk4[0].clone();
    start = __unpk4[1].clone();
    end = __unpk4[2].clone();
    before = if py_truthy(&place.clone()) { geti(&geti(&words.clone(), &op_sub(&place.clone(), &Py::Int(1))), &Py::Int(0)) } else { py_str("") };
    after = if py_truthy(&op_lt(&op_add(&place.clone(), &Py::Int(1)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) { geti(&geti(&words.clone(), &op_add(&place.clone(), &Py::Int(1))), &Py::Int(0)) } else { py_str("") };
    meant = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::understanding::lexicon::g_CONFUSIONS(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&meant.clone()))) {
    pair = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::understanding::lexicon::g_CONFUSIONS_AFTER(), "get", args))(&__args) };
    if py_truthy(&{ let __b = pair.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&geti(&pair.clone(), &Py::Int(1)), &before.clone())) } }) {
    meant = geti(&pair.clone(), &Py::Int(0));
    } else {
    pair = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::understanding::lexicon::g_CONFUSIONS_BEFORE(), "get", args))(&__args) };
    if py_truthy(&{ let __b = pair.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&geti(&pair.clone(), &Py::Int(1)), &after.clone())) } }) {
    meant = geti(&pair.clone(), &Py::Int(0));
    } else {
    if py_truthy(&Py::Bool(!py_truthy(&after.clone()))) {
    meant = Py::None;
    }
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&meant.clone()))) {
    pair = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::understanding::lexicon::g_CONFUSIONS_IN_A_FEELING(), "get", args))(&__args) };
    if py_truthy(&{ let __b = pair.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let __b = Py::Bool(!py_truthy(&after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&geti(&pair.clone(), &Py::Int(1)), &after.clone())) } }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); __args.push(place.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_of_themselves(args))(&__args) } } } } }) {
    meant = geti(&pair.clone(), &Py::Int(0));
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&meant.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::understanding::lexicon::g_QUESTION_WORDS(), &after.clone())) } }) {
    meant = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::understanding::lexicon::g_CONFUSIONS_BEFORE_A_QUESTION(), "get", args))(&__args) };
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&meant.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&meant.clone(), &word.clone())) } }) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&text.clone(), &cut.clone(), &start.clone(), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pieces.clone(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(meant.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pieces.clone(), "append", args))(&__args) };
    cut = end.clone();
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![word.clone(), meant.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pairs.clone(), "append", args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&pairs.clone()))) {
    return py_tuple(vec![py_str(""), py_list(vec![])]);
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&text.clone(), &cut.clone(), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pieces.clone(), "append", args))(&__args) };
    return py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pieces.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(""), "join", args))(&__args) }, pairs.clone()]);
    Py::None
}
pub fn _a_constraint_on_the_thing(args: &[Py]) -> Py {
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
    let _ = py_str("True for a constraint said as a fact about the thing in hand (\"my budget is 5k\").");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("norm")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CONSTRAINT_FACT(), "match", args))(&__args) }, &Py::None));
    Py::None
}
pub fn _read_line(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut _word: Py = Py::None;
    let mut about: Py = Py::None;
    let mut after: Py = Py::None;
    let mut after_entities: Py = Py::None;
    let mut animal_in_hand: Py = Py::None;
    let mut ask_subject: Py = Py::None;
    let mut asks_something: Py = Py::None;
    let mut carried: Py = Py::None;
    let mut carried_is_older: Py = Py::None;
    let mut carried_text: Py = Py::None;
    let mut carrying_on: Py = Py::None;
    let mut claimed: Py = Py::None;
    let mut constraint_before: Py = Py::None;
    let mut continues: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut earlier_entities: Py = Py::None;
    let mut earlier_subject: Py = Py::None;
    let mut entities: Py = Py::None;
    let mut entity: Py = Py::None;
    let mut history: Py = Py::None;
    let mut index: Py = Py::None;
    let mut inherited: Py = Py::None;
    let mut intent: Py = Py::None;
    let mut last_read: Py = Py::None;
    let mut leans_on: Py = Py::None;
    let mut line: Py = Py::None;
    let mut name: Py = Py::None;
    let mut out: Py = Py::None;
    let mut over_time: Py = Py::None;
    let mut over_time_what: Py = Py::None;
    let mut own_score: Py = Py::None;
    let mut own_subject: Py = Py::None;
    let mut own_topic: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut points: Py = Py::None;
    let mut presence: Py = Py::None;
    let mut previous: Py = Py::None;
    let mut previous_entities: Py = Py::None;
    let mut previous_text: Py = Py::None;
    let mut previous_topic: Py = Py::None;
    let mut ranked: Py = Py::None;
    let mut read: Py = Py::None;
    let mut ref_: Py = Py::None;
    let mut score: Py = Py::None;
    let mut setting: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut subject_from: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut topic_from: Py = Py::None;
    let mut topic_line: Py = Py::None;
    let mut weight: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("One reading of the user's line, before the second chance at the words in it.");
    memory = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { memory.clone() } else { py_dict(vec![]) };
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::normalize::without_the_figure_of_speech(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::normalize::_the_part_that_hurts(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::normalize::_the_sugar_that_is_blood(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::normalize::normalise(args))(&__args) };
    read = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::classify(args))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&geti(&read.clone(), &py_str("kind")), &py_str("thanks")))) {
    after = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::the_line_after_a_courtesy(args))(&__args) };
    if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_THANKED_FOR(), &geti(&after.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _about_something_of_its_own(args))(&__args) } } } } }) {
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::normalize::normalise(args))(&__args) };
    read = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::classify(args))(&__args) };
    }
    }
    history = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(g_LOOKBACK()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _user_lines(args))(&__args) };
    previous_text = if py_truthy(&history.clone()) { geti(&history.clone(), &op_neg(&Py::Int(1))) } else { Py::None };
    previous = if py_truthy(&previous_text.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(previous_text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::normalize::normalise(args))(&__args) } } else { Py::None };
    previous_entities = if py_truthy(&previous.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(previous.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::detect_entities(args))(&__args) } } else { py_list(vec![]) };
    previous_topic = Py::None;
    if py_truthy(&previous.clone()) {
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(previous.clone()); __args.push(Py::None); __args.push(previous_entities.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::detect_topic(args))(&__args) }, 2, None);
    previous_topic = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    }
    let __unpk2 = py_unpack(&py_tuple(vec![previous_text.clone(), previous.clone()]), 2, None);
    carried_text = __unpk2[0].clone();
    carried = __unpk2[1].clone();
    carried_is_older = Py::Bool(false);
    constraint_before = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(previous.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_constraint_on_the_thing(args))(&__args) };
    if py_truthy(&{ let __b = previous.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&previous_entities.clone())); if py_truthy(&__b) { __b } else { constraint_before.clone() } } } }) {
    own_subject = if py_truthy(&constraint_before.clone()) { Py::None } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(previous.clone()); __args.push(previous_entities.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::subject_topic(args))(&__args) } };
    let __unpk3 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&history.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pointed_at(args))(&__args) }, 2, None);
    earlier = __unpk3[0].clone();
    earlier_entities = __unpk3[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&earlier_entities.clone())); if !py_truthy(&__b) { __b } else { op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_ANY_PRONOUN()) } }) {
    let __unpk4 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_recorded_subject(args))(&__args) }, 2, None);
    earlier = __unpk4[0].clone();
    earlier_entities = __unpk4[1].clone();
    carried_is_older = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier_entities.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    if py_truthy(&{ let __b = earlier_entities.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&own_subject.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&constraint_before.clone())) } } } }) {
    earlier_subject = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); __args.push(earlier_entities.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::subject_topic(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&earlier_subject.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&earlier_subject.clone(), &own_subject.clone())) } }) {
    let __unpk5 = py_unpack(&py_tuple(vec![Py::None, py_list(vec![])]), 2, None);
    earlier = __unpk5[0].clone();
    earlier_entities = __unpk5[1].clone();
    carried_is_older = Py::Bool(false);
    }
    }
    if py_truthy(&earlier_entities.clone()) {
    previous_entities = earlier_entities.clone();
    let __unpk6 = py_unpack(&py_tuple(vec![geti(&earlier.clone(), &py_str("text")), earlier.clone()]), 2, None);
    carried_text = __unpk6[0].clone();
    carried = __unpk6[1].clone();
    if py_truthy(&Py::Bool(py_is(&previous_topic.clone(), &Py::None))) {
    previous_topic = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); __args.push(Py::None); __args.push(earlier_entities.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::detect_topic(args))(&__args) }, &Py::Int(0));
    }
    }
    }
    ref_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(read.clone()); __args.push(previous.clone()); __args.push(previous_entities.clone()); __args.push(previous_topic.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::references::resolve(args))(&__args) };
    about = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::after_any_link(args))(&__args) };
    topic_line = line.clone();
    after_entities = if py_truthy(&about.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(about.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::normalize::normalise(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::detect_entities(args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&after_entities.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(about.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::normalize::normalise(args))(&__args) }); __args.push(after_entities.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::subject_topic(args))(&__args) }, &Py::None)) } }) {
    let __unpk7 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(about.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::normalize::normalise(args))(&__args) }, after_entities.clone()]), 2, None);
    topic_line = __unpk7[0].clone();
    entities = __unpk7[1].clone();
    } else {
    entities = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::detect_entities(args))(&__args) };
    }
    continues = { let __b = geti(&ref_.clone(), &py_str("followup")); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic_line.clone()); __args.push(entities.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::subject_topic(args))(&__args) }, &Py::None)) } };
    inherited = Py::None;
    if py_truthy(&{ let __b = continues.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _changes_subject(args))(&__args) })) } }) {
    claimed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("topic")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(claimed.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { claimed.clone() } }) {
    inherited = claimed.clone();
    } else {
    inherited = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(history.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _recent_topic(args))(&__args) }; if py_truthy(&__b) { __b } else { previous_topic.clone() } };
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&read.clone(), &py_str("kind")), &py_str("statement"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&geti(&read.clone(), &py_str("asks")))); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&entities.clone()) { let mut entity: Py = __item.clone(); __out.push(Py::Bool(py_eq(&geti(&entity.clone(), &py_str("type")), &py_str("place")))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::noun_phrase_request(args))(&__args) } } } } } } }) {
    let _ = py_setitem(&read.clone(), &py_str("intent"), py_str("request"));
    let _ = py_setitem(&read.clone(), &py_str("asks"), py_list(vec![py_str("recommend")]));
    let _ = py_setitem(&read.clone(), &py_str("replies_to_bot"), Py::Bool(false));
    }
    asks_something = { let __b = Py::Bool(py_eq(&geti(&read.clone(), &py_str("kind")), &py_str("question"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("request"), py_str("advice")]), &geti(&read.clone(), &py_str("intent")))) } };
    let __unpk8 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic_line.clone()); __args.push(Py::None); __args.push(entities.clone()); __args.push(asks_something.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::detect_topic(args))(&__args) }, 2, None);
    own_topic = __unpk8[0].clone();
    own_score = __unpk8[1].clone();
    let __unpk9 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic_line.clone()); __args.push(inherited.clone()); __args.push(entities.clone()); __args.push(asks_something.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::detect_topic(args))(&__args) }, 2, None);
    topic = __unpk9[0].clone();
    score = __unpk9[1].clone();
    ranked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic_line.clone()); __args.push(Py::None); __args.push(entities.clone()); __args.push(asks_something.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::rank_topics(args))(&__args) }) { let __unpk6170 = py_unpack(&__item.clone(), 2, None); let mut name: Py = __unpk6170[0].clone(); let mut points: Py = __unpk6170[1].clone(); __out.push(name.clone()); } py_list(__out) })() };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_stop_in_a_chat_about_something(args))(&__args) }) {
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("intent")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }, &py_str("about_bot")))) {
    let _ = py_setitem(&read.clone(), &py_str("intent"), py_str("question"));
    }
    let _ = py_setitem(&read.clone(), &py_str("about_bot"), Py::Bool(false));
    let _ = py_setitem(&read.clone(), &py_str("about_bot_kind"), Py::None);
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&topic.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::opening_line(args))(&__args) })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(read.clone()); __args.push(entities.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _about_the_bot(args))(&__args) } } } } }) {
    topic = g_ABOUT_THE_BOT();
    ranked = op_add(&py_list(vec![g_ABOUT_THE_BOT()]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranked.clone()) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&name.clone(), &g_ABOUT_THE_BOT()))) {  __out.push(name.clone()); } } py_list(__out) })() });
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_is(&topic.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("greeting"), py_str("smalltalk")]), &geti(&read.clone(), &py_str("intent")))) } }) {
    topic = py_str("small-talk");
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_is(&topic.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("state_owner")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }, &py_str("self"))); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("emotion")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&entities.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("question"), py_str("request")]), &geti(&read.clone(), &py_str("intent")))) } } } } } } } }) {
    topic = py_str("feelings");
    ranked = op_add(&py_list(vec![py_str("feelings")]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranked.clone()) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&name.clone(), &py_str("feelings")))) {  __out.push(name.clone()); } } py_list(__out) })() });
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&read.clone(), &py_str("kind")), &py_str("question"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&topic.clone(), &py_str("small-talk"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&ranked.clone(), &g_OWN_QUESTION_TOPIC())) } } } }) {
    topic = g_OWN_QUESTION_TOPIC();
    ranked = op_add(&py_list(vec![g_OWN_QUESTION_TOPIC()]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranked.clone()) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&name.clone(), &g_OWN_QUESTION_TOPIC()))) {  __out.push(name.clone()); } } py_list(__out) })() });
    }
    subject = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&entities.clone()) { let mut entity: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_SUBJECT_TYPES(), &geti(&entity.clone(), &py_str("type"))))) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&entity.clone(), &py_str("text")), "lower", args))(&__args) }); } } py_list(__out) })() }); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("next", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __unpk10 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("clauses"))); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("focus")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }); __args.push(entities.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing_the_ask_names(args))(&__args) }, 3, None);
    ask_subject = __unpk10[0].clone();
    setting = __unpk10[1].clone();
    subject_from = __unpk10[2].clone();
    topic_from = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(topic.clone()); __args.push(entities.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::the_words_that_said(args))(&__args) };
    leans_on = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(read.clone()); __args.push(ref_.clone()); __args.push(carried_text.clone()); __args.push(carried.clone()); __args.push(previous.clone()); __args.push(own_topic.clone()); __args.push(previous_topic.clone()); __args.push(previous_entities.clone()); __args.push(carried_is_older.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("topic")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _leans_on(args))(&__args) };
    intent = geti(&read.clone(), &py_str("intent"));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&ranked.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("question"), py_str("request"), py_str("news"), py_str("statement"), py_str("ack")]), &intent.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::opening_line(args))(&__args) } } } } }) {
    intent = py_str("opening");
    } else {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&entities.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&leans_on.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("question"), py_str("request")]), &intent.clone())); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(previous.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _opens_the_conversation(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let __b = previous_topic.clone(); if py_truthy(&__b) { __b } else { previous_entities.clone() } })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| asks_with_nothing_named(args))(&__args) } } } } } } } } } } }) {
    intent = py_str("opening");
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("question"), py_str("request"), py_str("news"), py_str("statement"), py_str("ack")]), &intent.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::talk_frame(args))(&__args) } } }) {
    intent = py_str("opening");
    }
    }
    }
    presence = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _presence(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asks_for_a_moment(args))(&__args) } } };
    if py_truthy(&{ let __b = presence.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PRESENCE_NOT(), &geti(&read.clone(), &py_str("intent")))) } }) {
    presence = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_bitor(&g_PRESENCE_SUBJECT(), &g_PRESENCE_ASKER()))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asks_for_a_moment(args))(&__args) } } };
    }
    if py_truthy(&presence.clone()) {
    intent = py_str("presence");
    }
    plain = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("plain"))); __args.push(entities.clone()); __args.push(geti(&read.clone(), &py_str("state_owner"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_word_that_depends_on_the_owner(args))(&__args) };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("words"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::breeds::the_animal_a_breed_is(args))(&__args) };
    animal_in_hand = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ref_.clone()); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::references::the_animal_in_hand(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&animal_in_hand.clone()))) {
    animal_in_hand = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); __args.push(if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("understood")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::references::the_animal_the_chat_is_on(args))(&__args) };
    }
    let __unpk11 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(read.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::over_time(args))(&__args) }, 2, None);
    over_time = __unpk11[0].clone();
    over_time_what = __unpk11[1].clone();
    last_read = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("understood")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    out = py_dict(vec![(py_str("text"), if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { text.clone() } else { py_str("") }), (py_str("clean"), geti(&line.clone(), &py_str("clean"))), (py_str("words"), words.clone()), (py_str("plain"), plain.clone()), (py_str("animal_in_hand"), animal_in_hand.clone()), (py_str("change_of_subject"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("change_of_subject")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("dropped_subject"), geti(&line.clone(), &py_str("dropped_subject"))), (py_str("sense"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::senses::senses(args))(&__args) }), (py_str("over_time"), over_time.clone()), (py_str("over_time_what"), over_time_what.clone()), (py_str("kept"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("kept")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }), (py_str("kind"), if py_truthy(&presence.clone()) { py_str("question") } else { geti(&read.clone(), &py_str("kind")) }), (py_str("intent"), intent.clone()), (py_str("asks_how_i_am"), { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::asks_how_i_am(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("smalltalk"), py_str("greeting")]), &intent.clone())) } }), (py_str("presence"), presence.clone()), (py_str("asks"), geti(&read.clone(), &py_str("asks"))), (py_str("tag"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tag")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("taking_up"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(read.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::taking_up(args))(&__args) }), (py_str("shift"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("clean"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::shift_worked(args))(&__args) }), (py_str("care_home"), { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("clean"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::a_care_home(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("clean"))); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(g_HOME_LOOKBACK()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _user_lines(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::a_home_carried_forward(args))(&__args) } } }), (py_str("setup"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::the_setup(args))(&__args) }), (py_str("help_ask"), geti(&read.clone(), &py_str("help_ask"))), (py_str("asks_a_verdict"), geti(&read.clone(), &py_str("asks_a_verdict"))), (py_str("emotion"), geti(&read.clone(), &py_str("emotion"))), (py_str("state_owner"), geti(&read.clone(), &py_str("state_owner"))), (py_str("shortage"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("shortage")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }), (py_str("polarity"), geti(&read.clone(), &py_str("polarity"))), (py_str("warmth"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::the_warmth_said(args))(&__args) }), (py_str("body"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm_text"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::body::read(args))(&__args) }), (py_str("own_output"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::own_output_word(args))(&__args) }), (py_str("short"), geti(&read.clone(), &py_str("short"))), (py_str("first_person"), geti(&read.clone(), &py_str("first_person"))), (py_str("about_bot"), geti(&read.clone(), &py_str("about_bot"))), (py_str("about_bot_kind"), geti(&read.clone(), &py_str("about_bot_kind"))), (py_str("counts_the_bots_own_day"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("counts_the_bots_own_day")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("replies_to_bot"), geti(&read.clone(), &py_str("replies_to_bot"))), (py_str("agrees_to_try"), { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("agrees_to_try")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&intent.clone(), &py_str("ack"))) } }), (py_str("wants_a_quiz"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("wants_a_quiz")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("agreement"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("agreement")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }), (py_str("tight_on_money"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); __args.push(geti(&read.clone(), &py_str("kind"))); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::tight_on_money(args))(&__args) }), (py_str("amount_left"), Py::Bool(!py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::purse_till_at(args))(&__args) }, &Py::None))), (py_str("routine"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(read.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::is_a_routine(args))(&__args) }), (py_str("a_load_carried"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("clean"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| a_load_carried(args))(&__args) }), (py_str("car_fault"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("clean"))); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(crate::understanding::cars::g_LOOKBACK()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _user_lines(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::cars::a_broken_car_is_the_subject(args))(&__args) }), (py_str("summons"), geti(&read.clone(), &py_str("summons"))), (py_str("asks_back"), geti(&read.clone(), &py_str("asks_back"))), (py_str("stand_in"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _stand_in_here(args))(&__args) }), (py_str("excludes"), { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("excludes")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }), (py_str("constraint"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("constraint")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("confusion"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::confusion::confusion_of(args))(&__args) }), (py_str("suggestion"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("suggestion")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }), (py_str("ruled_out"), { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ruled_out")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }), (py_str("norm"), geti(&line.clone(), &py_str("norm"))), (py_str("norm_text"), geti(&line.clone(), &py_str("norm_text"))), (py_str("fixes"), geti(&line.clone(), &py_str("fixes"))), (py_str("clauses"), geti(&line.clone(), &py_str("clauses"))), (py_str("focus"), geti(&line.clone(), &py_str("focus"))), (py_str("asking"), { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("asking")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { geti(&line.clone(), &py_str("focus")) } }), (py_str("signals"), geti(&line.clone(), &py_str("signals"))), (py_str("topic"), topic.clone()), (py_str("topic_score"), score.clone()), (py_str("topics"), ranked.clone()), (py_str("topic_own"), own_topic.clone()), (py_str("topic_own_score"), own_score.clone()), (py_str("topic_from"), topic_from.clone()), (py_str("topic_is_a_brush"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&topic_from.clone()) { let __unpk4958 = py_unpack(&__item.clone(), 2, None); let mut _word: Py = __unpk4958[0].clone(); let mut weight: Py = __unpk4958[1].clone(); __out.push(op_lt(&weight.clone(), &Py::Int(2))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("unsure"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("norm")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); __args.push({ let __b = own_topic.clone(); if py_truthy(&__b) { __b } else { topic.clone() } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::unsure_words(args))(&__args) }), (py_str("entities"), entities.clone()), (py_str("person_names"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::person_names(args))(&__args) }), (py_str("background"), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::normalize::background_positions(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut index: Py = __item.clone(); __out.push(geti(&geti(&line.clone(), &py_str("norm")), &index.clone())); } py_list(__out) })() }), (py_str("intensifier"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("intensifier")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("intensifier_left"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("intensifier_left")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("subject"), subject.clone()), (py_str("subject_unclear"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); __args.push(inherited.clone()); __args.push(previous.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::unclear_subject(args))(&__args) }), (py_str("ask_subject"), ask_subject.clone()), (py_str("setting"), setting.clone()), (py_str("subject_from"), subject_from.clone()), (py_str("numbers"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::topics::detect_numbers(args))(&__args) }), (py_str("tried"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { geti(&line.clone(), &py_str("text")) } else { py_str("") }); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::attempts::things_tried(args))(&__args) }), (py_str("keywords"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _keywords(args))(&__args) }), (py_str("ref"), ref_.clone()), (py_str("followup"), geti(&ref_.clone(), &py_str("followup"))), (py_str("someone_new"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(previous_entities.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::references::person_is_new(args))(&__args) }), (py_str("asks_what_was_meant"), { let __b = geti(&read.clone(), &py_str("asks_what_was_meant")); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(leans_on.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_meaning_is_applied_to_what_was_said(args))(&__args) } } }), (py_str("leans_on"), leans_on.clone())]);
    carrying_on = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(out.clone()); __args.push(last_read.clone()); __kws.extend(vec![("carrying_on".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::over_time(args))(&__args) };
    if py_truthy(&Py::Bool(!py_is(&carrying_on.clone(), &Py::None))) {
    let __unpk12 = py_unpack(&carrying_on.clone(), 2, None);
    let _ = py_setitem(&out.clone(), &py_str("over_time"), __unpk12[0].clone());
    let _ = py_setitem(&out.clone(), &py_str("over_time_what"), __unpk12[1].clone());
    }
    return out.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.understanding".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_THE_BOT" => Some(g_ABOUT_THE_BOT()),
            "AFTER_BOT_FILLER" => Some(g_AFTER_BOT_FILLER()),
            "ALL_ON_THEM" => Some(g_ALL_ON_THEM()),
            "ANY_PRONOUN" => Some(g_ANY_PRONOUN()),
            "ASKING_WORDS" => Some(g_ASKING_WORDS()),
            "ASKS_WHY_IT_IS" => Some(g_ASKS_WHY_IT_IS()),
            "ASK_FREE_FILLER" => Some(g_ASK_FREE_FILLER()),
            "ASK_FREE_WORDS" => Some(g_ASK_FREE_WORDS()),
            "ASK_PERMISSION" => Some(g_ASK_PERMISSION()),
            "AT_IT_STILL" => Some(g_AT_IT_STILL()),
            "A_CLAUSE_VERB" => Some(g_A_CLAUSE_VERB()),
            "A_CONDITION_AFTER" => Some(g_A_CONDITION_AFTER()),
            "A_DESCRIBING_SUBJECT" => Some(g_A_DESCRIBING_SUBJECT()),
            "A_FAULT_TO_LEND" => Some(g_A_FAULT_TO_LEND()),
            "A_STAND_IN" => Some(g_A_STAND_IN()),
            "A_VALUE_IN_FRONT_OF_NOT" => Some(g_A_VALUE_IN_FRONT_OF_NOT()),
            "BE_VERB" => Some(g_BE_VERB()),
            "BOT_ASK_WORDS" => Some(g_BOT_ASK_WORDS()),
            "BOT_NOUNS" => Some(g_BOT_NOUNS()),
            "BOT_STATE" => Some(g_BOT_STATE()),
            "BROKEN_OF_A_THING" => Some(g_BROKEN_OF_A_THING()),
            "CARRY_PASS" => Some(g_CARRY_PASS()),
            "CARRY_TOPIC" => Some(g_CARRY_TOPIC()),
            "CARRY_WORDS" => Some(g_CARRY_WORDS()),
            "CHAT_WORDS" => Some(g_CHAT_WORDS()),
            "CONSTRAINT_FACT" => Some(g_CONSTRAINT_FACT()),
            "COPULA_AUX" => Some(g_COPULA_AUX()),
            "CORRECTION_HEAD" => Some(g_CORRECTION_HEAD()),
            "CORRECTION_MARKER" => Some(g_CORRECTION_MARKER()),
            "CORRECTION_OPENER" => Some(g_CORRECTION_OPENER()),
            "DAY_VERBS" => Some(g_DAY_VERBS()),
            "DAY_WORDS" => Some(g_DAY_WORDS()),
            "DETERMINER_TAIL" => Some(g_DETERMINER_TAIL()),
            "DID_IT" => Some(g_DID_IT()),
            "DOES_ALL_THEY_CAN" => Some(g_DOES_ALL_THEY_CAN()),
            "DOES_IT_ALL" => Some(g_DOES_IT_ALL()),
            "DOES_IT_RIGHT" => Some(g_DOES_IT_RIGHT()),
            "EDGE_PUNCTUATION" => Some(g_EDGE_PUNCTUATION()),
            "EVERYTHING_AT_ONCE" => Some(g_EVERYTHING_AT_ONCE()),
            "FIRST_PERSON_ANYWHERE" => Some(g_FIRST_PERSON_ANYWHERE()),
            "FOLLOW_OPENER" => Some(g_FOLLOW_OPENER()),
            "GOOD_TIME_WORDS" => Some(g_GOOD_TIME_WORDS()),
            "GREETING_PHRASE" => Some(g_GREETING_PHRASE()),
            "HELPS_ME_NOT" => Some(g_HELPS_ME_NOT()),
            "HOME_LOOKBACK" => Some(g_HOME_LOOKBACK()),
            "HOME_SCOPE" => Some(g_HOME_SCOPE()),
            "HYPOTHETICAL" => Some(g_HYPOTHETICAL()),
            "INTRODUCING_A_STATE" => Some(g_INTRODUCING_A_STATE()),
            "LEANS_ON_THEM" => Some(g_LEANS_ON_THEM()),
            "LOOKBACK" => Some(g_LOOKBACK()),
            "MANNER_TALK" => Some(g_MANNER_TALK()),
            "MAX_KEYWORDS" => Some(g_MAX_KEYWORDS()),
            "MEANING_TAIL" => Some(g_MEANING_TAIL()),
            "MEANING_WORDS" => Some(g_MEANING_WORDS()),
            "MODAL_STATE" => Some(g_MODAL_STATE()),
            "MODAL_WORDS" => Some(g_MODAL_WORDS()),
            "MOMENT_FILLER" => Some(g_MOMENT_FILLER()),
            "MOMENT_VERBS" => Some(g_MOMENT_VERBS()),
            "MOMENT_WORDS" => Some(g_MOMENT_WORDS()),
            "MORE_WORDS" => Some(g_MORE_WORDS()),
            "NOTHING_NAMED_ASK_WORDS" => Some(g_NOTHING_NAMED_ASK_WORDS()),
            "NOTHING_NAMED_FILLER" => Some(g_NOTHING_NAMED_FILLER()),
            "NOTHING_NAMED_VIEW_WORDS" => Some(g_NOTHING_NAMED_VIEW_WORDS()),
            "NOT_A_SUBJECT" => Some(g_NOT_A_SUBJECT()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NOT_MANNER" => Some(g_NOT_MANNER()),
            "NO_ONE_HELPS" => Some(g_NO_ONE_HELPS()),
            "NO_TIME_OFF" => Some(g_NO_TIME_OFF()),
            "OTHER_PRONOUNS" => Some(g_OTHER_PRONOUNS()),
            "OWN_QUESTION_TOPIC" => Some(g_OWN_QUESTION_TOPIC()),
            "OWN_VERB" => Some(g_OWN_VERB()),
            "PERMISSION_FILLER" => Some(g_PERMISSION_FILLER()),
            "PERSON_HEADS" => Some(g_PERSON_HEADS()),
            "PERSON_STOP" => Some(g_PERSON_STOP()),
            "POSSESSIVE" => Some(g_POSSESSIVE()),
            "PRAISED" => Some(g_PRAISED()),
            "PRESENCE_ASKER" => Some(g_PRESENCE_ASKER()),
            "PRESENCE_FILLER" => Some(g_PRESENCE_FILLER()),
            "PRESENCE_GREETING" => Some(g_PRESENCE_GREETING()),
            "PRESENCE_MAX" => Some(g_PRESENCE_MAX()),
            "PRESENCE_NOT" => Some(g_PRESENCE_NOT()),
            "PRESENCE_SUBJECT" => Some(g_PRESENCE_SUBJECT()),
            "PRESENCE_WORD" => Some(g_PRESENCE_WORD()),
            "PROPERTY_ASK" => Some(g_PROPERTY_ASK()),
            "PROPERTY_SUBJECTS" => Some(g_PROPERTY_SUBJECTS()),
            "REACHING_KEYS" => Some(g_REACHING_KEYS()),
            "RUNS_MYSELF_RAGGED" => Some(g_RUNS_MYSELF_RAGGED()),
            "RUNS_THE_HOUSE" => Some(g_RUNS_THE_HOUSE()),
            "SAME_KIND_WORDS" => Some(g_SAME_KIND_WORDS()),
            "SECOND_JOIN_HEAD" => Some(g_SECOND_JOIN_HEAD()),
            "SECOND_JOIN_TAIL" => Some(g_SECOND_JOIN_TAIL()),
            "SECOND_PERSON" => Some(g_SECOND_PERSON()),
            "SHIFTS" => Some(g_SHIFTS()),
            "SHIFT_ACTUALLY" => Some(g_SHIFT_ACTUALLY()),
            "SPEAKER_SUBJECTS" => Some(g_SPEAKER_SUBJECTS()),
            "STAND_IN_NOT_THINGS" => Some(g_STAND_IN_NOT_THINGS()),
            "STAND_IN_SELECTORS" => Some(g_STAND_IN_SELECTORS()),
            "STAND_IN_WORDS" => Some(g_STAND_IN_WORDS()),
            "STOPWORDS" => Some(g_STOPWORDS()),
            "SUBJECT_TYPES" => Some(g_SUBJECT_TYPES()),
            "TALK_ASK" => Some(g_TALK_ASK()),
            "TALK_FILLER" => Some(g_TALK_FILLER()),
            "TALK_VERBS" => Some(g_TALK_VERBS()),
            "THANKED_FOR" => Some(g_THANKED_FOR()),
            "THE_ONE_WHO" => Some(g_THE_ONE_WHO()),
            "THIRD_PERSON" => Some(g_THIRD_PERSON()),
            "WHO_ENDS" => Some(g_WHO_ENDS()),
            "WORK_SCOPE" => Some(g_WORK_SCOPE()),
            "WORN_OUT_OF_A_PERSON" => Some(g_WORN_OUT_OF_A_PERSON()),
            "_A_CLAUSE_WITH_NO_SUBJECT" => Some(g__A_CLAUSE_WITH_NO_SUBJECT()),
            "_A_THING_THAT_CHANGES" => Some(g__A_THING_THAT_CHANGES()),
            "_NOT_ABOUT_THE_THING" => Some(g__NOT_ABOUT_THE_THING()),
            "_SEARCH_FRAME" => Some(g__SEARCH_FRAME()),
            "_a_constraint_on_the_thing" => Some(py_func("_a_constraint_on_the_thing", |args: &[Py]| _a_constraint_on_the_thing(args))),
            "_a_describing_word_is_the_subject" => Some(py_func("_a_describing_word_is_the_subject", |args: &[Py]| _a_describing_word_is_the_subject(args))),
            "_a_manner" => Some(py_func("_a_manner", |args: &[Py]| _a_manner(args))),
            "_a_plural_word" => Some(py_func("_a_plural_word", |args: &[Py]| _a_plural_word(args))),
            "_a_singular_word" => Some(py_func("_a_singular_word", |args: &[Py]| _a_singular_word(args))),
            "_a_stand_in" => Some(py_func("_a_stand_in", |args: &[Py]| _a_stand_in(args))),
            "_a_stop_in_a_chat_about_something" => Some(py_func("_a_stop_in_a_chat_about_something", |args: &[Py]| _a_stop_in_a_chat_about_something(args))),
            "_a_word_that_depends_on_the_owner" => Some(py_func("_a_word_that_depends_on_the_owner", |args: &[Py]| _a_word_that_depends_on_the_owner(args))),
            "_about_a_person_or_a_trouble" => Some(py_func("_about_a_person_or_a_trouble", |args: &[Py]| _about_a_person_or_a_trouble(args))),
            "_about_something_of_its_own" => Some(py_func("_about_something_of_its_own", |args: &[Py]| _about_something_of_its_own(args))),
            "_about_the_bot" => Some(py_func("_about_the_bot", |args: &[Py]| _about_the_bot(args))),
            "_an_introduction_that_says_how_they_are" => Some(py_func("_an_introduction_that_says_how_they_are", |args: &[Py]| _an_introduction_that_says_how_they_are(args))),
            "_asked_again" => Some(py_func("_asked_again", |args: &[Py]| _asked_again(args))),
            "_asks_a_property_and_nothing_else" => Some(py_func("_asks_a_property_and_nothing_else", |args: &[Py]| _asks_a_property_and_nothing_else(args))),
            "_asks_for_a_moment" => Some(py_func("_asks_for_a_moment", |args: &[Py]| _asks_for_a_moment(args))),
            "_changes_subject" => Some(py_func("_changes_subject", |args: &[Py]| _changes_subject(args))),
            "_confused_text" => Some(py_func("_confused_text", |args: &[Py]| _confused_text(args))),
            "_keywords" => Some(py_func("_keywords", |args: &[Py]| _keywords(args))),
            "_leans_on" => Some(py_func("_leans_on", |args: &[Py]| _leans_on(args))),
            "_names_a_bot_noun" => Some(py_func("_names_a_bot_noun", |args: &[Py]| _names_a_bot_noun(args))),
            "_not_manner" => Some(py_func("_not_manner", |args: &[Py]| _not_manner(args))),
            "_opens_the_conversation" => Some(py_func("_opens_the_conversation", |args: &[Py]| _opens_the_conversation(args))),
            "_plural_of" => Some(py_func("_plural_of", |args: &[Py]| _plural_of(args))),
            "_pointable" => Some(py_func("_pointable", |args: &[Py]| _pointable(args))),
            "_pointed_at" => Some(py_func("_pointed_at", |args: &[Py]| _pointed_at(args))),
            "_presence" => Some(py_func("_presence", |args: &[Py]| _presence(args))),
            "_reaches" => Some(py_func("_reaches", |args: &[Py]| _reaches(args))),
            "_read_line" => Some(py_func("_read_line", |args: &[Py]| _read_line(args))),
            "_recent_topic" => Some(py_func("_recent_topic", |args: &[Py]| _recent_topic(args))),
            "_said_of_themselves" => Some(py_func("_said_of_themselves", |args: &[Py]| _said_of_themselves(args))),
            "_says_which_value_is_meant" => Some(py_func("_says_which_value_is_meant", |args: &[Py]| _says_which_value_is_meant(args))),
            "_singular_of" => Some(py_func("_singular_of", |args: &[Py]| _singular_of(args))),
            "_stand_in_here" => Some(py_func("_stand_in_here", |args: &[Py]| _stand_in_here(args))),
            "_the_ask_inside_an_errand" => Some(py_func("_the_ask_inside_an_errand", |args: &[Py]| _the_ask_inside_an_errand(args))),
            "_the_chat_subject" => Some(py_func("_the_chat_subject", |args: &[Py]| _the_chat_subject(args))),
            "_the_fault_to_lend" => Some(py_func("_the_fault_to_lend", |args: &[Py]| _the_fault_to_lend(args))),
            "_the_line_before" => Some(py_func("_the_line_before", |args: &[Py]| _the_line_before(args))),
            "_the_meaning_is_applied_to_what_was_said" => Some(py_func("_the_meaning_is_applied_to_what_was_said", |args: &[Py]| _the_meaning_is_applied_to_what_was_said(args))),
            "_the_new_thing" => Some(py_func("_the_new_thing", |args: &[Py]| _the_new_thing(args))),
            "_the_noun_to_lend" => Some(py_func("_the_noun_to_lend", |args: &[Py]| _the_noun_to_lend(args))),
            "_the_recorded_subject" => Some(py_func("_the_recorded_subject", |args: &[Py]| _the_recorded_subject(args))),
            "_the_same_kind" => Some(py_func("_the_same_kind", |args: &[Py]| _the_same_kind(args))),
            "_the_second_thing" => Some(py_func("_the_second_thing", |args: &[Py]| _the_second_thing(args))),
            "_the_spans_of" => Some(py_func("_the_spans_of", |args: &[Py]| _the_spans_of(args))),
            "_the_speaker_is_the_subject" => Some(py_func("_the_speaker_is_the_subject", |args: &[Py]| _the_speaker_is_the_subject(args))),
            "_the_swap_is_said" => Some(py_func("_the_swap_is_said", |args: &[Py]| _the_swap_is_said(args))),
            "_the_thing_the_ask_names" => Some(py_func("_the_thing_the_ask_names", |args: &[Py]| _the_thing_the_ask_names(args))),
            "_turns" => Some(py_func("_turns", |args: &[Py]| _turns(args))),
            "_user_lines" => Some(py_func("_user_lines", |args: &[Py]| _user_lines(args))),
            "_where_the_swap_goes" => Some(py_func("_where_the_swap_goes", |args: &[Py]| _where_the_swap_goes(args))),
            "_with_the_fault_lent" => Some(py_func("_with_the_fault_lent", |args: &[Py]| _with_the_fault_lent(args))),
            "_with_the_noun_lent" => Some(py_func("_with_the_noun_lent", |args: &[Py]| _with_the_noun_lent(args))),
            "a_load_carried" => Some(py_func("a_load_carried", |args: &[Py]| a_load_carried(args))),
            "asks_with_nothing_named" => Some(py_func("asks_with_nothing_named", |args: &[Py]| asks_with_nothing_named(args))),
            "line_as_read" => Some(py_func("line_as_read", |args: &[Py]| line_as_read(args))),
            "understand" => Some(py_func("understand", |args: &[Py]| understand(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_THE_BOT" => s_ABOUT_THE_BOT(v),
            "AFTER_BOT_FILLER" => s_AFTER_BOT_FILLER(v),
            "ALL_ON_THEM" => s_ALL_ON_THEM(v),
            "ANY_PRONOUN" => s_ANY_PRONOUN(v),
            "ASKING_WORDS" => s_ASKING_WORDS(v),
            "ASKS_WHY_IT_IS" => s_ASKS_WHY_IT_IS(v),
            "ASK_FREE_FILLER" => s_ASK_FREE_FILLER(v),
            "ASK_FREE_WORDS" => s_ASK_FREE_WORDS(v),
            "ASK_PERMISSION" => s_ASK_PERMISSION(v),
            "AT_IT_STILL" => s_AT_IT_STILL(v),
            "A_CLAUSE_VERB" => s_A_CLAUSE_VERB(v),
            "A_CONDITION_AFTER" => s_A_CONDITION_AFTER(v),
            "A_DESCRIBING_SUBJECT" => s_A_DESCRIBING_SUBJECT(v),
            "A_FAULT_TO_LEND" => s_A_FAULT_TO_LEND(v),
            "A_STAND_IN" => s_A_STAND_IN(v),
            "A_VALUE_IN_FRONT_OF_NOT" => s_A_VALUE_IN_FRONT_OF_NOT(v),
            "BE_VERB" => s_BE_VERB(v),
            "BOT_ASK_WORDS" => s_BOT_ASK_WORDS(v),
            "BOT_NOUNS" => s_BOT_NOUNS(v),
            "BOT_STATE" => s_BOT_STATE(v),
            "BROKEN_OF_A_THING" => s_BROKEN_OF_A_THING(v),
            "CARRY_PASS" => s_CARRY_PASS(v),
            "CARRY_TOPIC" => s_CARRY_TOPIC(v),
            "CARRY_WORDS" => s_CARRY_WORDS(v),
            "CHAT_WORDS" => s_CHAT_WORDS(v),
            "CONSTRAINT_FACT" => s_CONSTRAINT_FACT(v),
            "COPULA_AUX" => s_COPULA_AUX(v),
            "CORRECTION_HEAD" => s_CORRECTION_HEAD(v),
            "CORRECTION_MARKER" => s_CORRECTION_MARKER(v),
            "CORRECTION_OPENER" => s_CORRECTION_OPENER(v),
            "DAY_VERBS" => s_DAY_VERBS(v),
            "DAY_WORDS" => s_DAY_WORDS(v),
            "DETERMINER_TAIL" => s_DETERMINER_TAIL(v),
            "DID_IT" => s_DID_IT(v),
            "DOES_ALL_THEY_CAN" => s_DOES_ALL_THEY_CAN(v),
            "DOES_IT_ALL" => s_DOES_IT_ALL(v),
            "DOES_IT_RIGHT" => s_DOES_IT_RIGHT(v),
            "EDGE_PUNCTUATION" => s_EDGE_PUNCTUATION(v),
            "EVERYTHING_AT_ONCE" => s_EVERYTHING_AT_ONCE(v),
            "FIRST_PERSON_ANYWHERE" => s_FIRST_PERSON_ANYWHERE(v),
            "FOLLOW_OPENER" => s_FOLLOW_OPENER(v),
            "GOOD_TIME_WORDS" => s_GOOD_TIME_WORDS(v),
            "GREETING_PHRASE" => s_GREETING_PHRASE(v),
            "HELPS_ME_NOT" => s_HELPS_ME_NOT(v),
            "HOME_LOOKBACK" => s_HOME_LOOKBACK(v),
            "HOME_SCOPE" => s_HOME_SCOPE(v),
            "HYPOTHETICAL" => s_HYPOTHETICAL(v),
            "INTRODUCING_A_STATE" => s_INTRODUCING_A_STATE(v),
            "LEANS_ON_THEM" => s_LEANS_ON_THEM(v),
            "LOOKBACK" => s_LOOKBACK(v),
            "MANNER_TALK" => s_MANNER_TALK(v),
            "MAX_KEYWORDS" => s_MAX_KEYWORDS(v),
            "MEANING_TAIL" => s_MEANING_TAIL(v),
            "MEANING_WORDS" => s_MEANING_WORDS(v),
            "MODAL_STATE" => s_MODAL_STATE(v),
            "MODAL_WORDS" => s_MODAL_WORDS(v),
            "MOMENT_FILLER" => s_MOMENT_FILLER(v),
            "MOMENT_VERBS" => s_MOMENT_VERBS(v),
            "MOMENT_WORDS" => s_MOMENT_WORDS(v),
            "MORE_WORDS" => s_MORE_WORDS(v),
            "NOTHING_NAMED_ASK_WORDS" => s_NOTHING_NAMED_ASK_WORDS(v),
            "NOTHING_NAMED_FILLER" => s_NOTHING_NAMED_FILLER(v),
            "NOTHING_NAMED_VIEW_WORDS" => s_NOTHING_NAMED_VIEW_WORDS(v),
            "NOT_A_SUBJECT" => s_NOT_A_SUBJECT(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "NOT_MANNER" => s_NOT_MANNER(v),
            "NO_ONE_HELPS" => s_NO_ONE_HELPS(v),
            "NO_TIME_OFF" => s_NO_TIME_OFF(v),
            "OTHER_PRONOUNS" => s_OTHER_PRONOUNS(v),
            "OWN_QUESTION_TOPIC" => s_OWN_QUESTION_TOPIC(v),
            "OWN_VERB" => s_OWN_VERB(v),
            "PERMISSION_FILLER" => s_PERMISSION_FILLER(v),
            "PERSON_HEADS" => s_PERSON_HEADS(v),
            "PERSON_STOP" => s_PERSON_STOP(v),
            "POSSESSIVE" => s_POSSESSIVE(v),
            "PRAISED" => s_PRAISED(v),
            "PRESENCE_ASKER" => s_PRESENCE_ASKER(v),
            "PRESENCE_FILLER" => s_PRESENCE_FILLER(v),
            "PRESENCE_GREETING" => s_PRESENCE_GREETING(v),
            "PRESENCE_MAX" => s_PRESENCE_MAX(v),
            "PRESENCE_NOT" => s_PRESENCE_NOT(v),
            "PRESENCE_SUBJECT" => s_PRESENCE_SUBJECT(v),
            "PRESENCE_WORD" => s_PRESENCE_WORD(v),
            "PROPERTY_ASK" => s_PROPERTY_ASK(v),
            "PROPERTY_SUBJECTS" => s_PROPERTY_SUBJECTS(v),
            "REACHING_KEYS" => s_REACHING_KEYS(v),
            "RUNS_MYSELF_RAGGED" => s_RUNS_MYSELF_RAGGED(v),
            "RUNS_THE_HOUSE" => s_RUNS_THE_HOUSE(v),
            "SAME_KIND_WORDS" => s_SAME_KIND_WORDS(v),
            "SECOND_JOIN_HEAD" => s_SECOND_JOIN_HEAD(v),
            "SECOND_JOIN_TAIL" => s_SECOND_JOIN_TAIL(v),
            "SECOND_PERSON" => s_SECOND_PERSON(v),
            "SHIFTS" => s_SHIFTS(v),
            "SHIFT_ACTUALLY" => s_SHIFT_ACTUALLY(v),
            "SPEAKER_SUBJECTS" => s_SPEAKER_SUBJECTS(v),
            "STAND_IN_NOT_THINGS" => s_STAND_IN_NOT_THINGS(v),
            "STAND_IN_SELECTORS" => s_STAND_IN_SELECTORS(v),
            "STAND_IN_WORDS" => s_STAND_IN_WORDS(v),
            "STOPWORDS" => s_STOPWORDS(v),
            "SUBJECT_TYPES" => s_SUBJECT_TYPES(v),
            "TALK_ASK" => s_TALK_ASK(v),
            "TALK_FILLER" => s_TALK_FILLER(v),
            "TALK_VERBS" => s_TALK_VERBS(v),
            "THANKED_FOR" => s_THANKED_FOR(v),
            "THE_ONE_WHO" => s_THE_ONE_WHO(v),
            "THIRD_PERSON" => s_THIRD_PERSON(v),
            "WHO_ENDS" => s_WHO_ENDS(v),
            "WORK_SCOPE" => s_WORK_SCOPE(v),
            "WORN_OUT_OF_A_PERSON" => s_WORN_OUT_OF_A_PERSON(v),
            "_A_CLAUSE_WITH_NO_SUBJECT" => s__A_CLAUSE_WITH_NO_SUBJECT(v),
            "_A_THING_THAT_CHANGES" => s__A_THING_THAT_CHANGES(v),
            "_NOT_ABOUT_THE_THING" => s__NOT_ABOUT_THE_THING(v),
            "_SEARCH_FRAME" => s__SEARCH_FRAME(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("understanding", module_obj);
    pyrt::xrefs::reg_fn("understanding", "_the_second_thing", py_func("_the_second_thing", |args: &[Py]| _the_second_thing(args)));
    pyrt::xrefs::reg_fn("understanding", "_says_which_value_is_meant", py_func("_says_which_value_is_meant", |args: &[Py]| _says_which_value_is_meant(args)));
    pyrt::xrefs::reg_fn("understanding", "_the_fault_to_lend", py_func("_the_fault_to_lend", |args: &[Py]| _the_fault_to_lend(args)));
    pyrt::xrefs::reg_fn("understanding", "_a_stand_in", py_func("_a_stand_in", |args: &[Py]| _a_stand_in(args)));
    pyrt::xrefs::reg_fn("understanding", "_plural_of", py_func("_plural_of", |args: &[Py]| _plural_of(args)));
    pyrt::xrefs::reg_fn("understanding", "_a_plural_word", py_func("_a_plural_word", |args: &[Py]| _a_plural_word(args)));
    pyrt::xrefs::reg_fn("understanding", "_singular_of", py_func("_singular_of", |args: &[Py]| _singular_of(args)));
    pyrt::xrefs::reg_fn("understanding", "_a_singular_word", py_func("_a_singular_word", |args: &[Py]| _a_singular_word(args)));
    pyrt::xrefs::reg_fn("understanding", "_the_noun_to_lend", py_func("_the_noun_to_lend", |args: &[Py]| _the_noun_to_lend(args)));
    pyrt::xrefs::reg_fn("understanding", "_with_the_noun_lent", py_func("_with_the_noun_lent", |args: &[Py]| _with_the_noun_lent(args)));
    pyrt::xrefs::reg_fn("understanding", "_a_describing_word_is_the_subject", py_func("_a_describing_word_is_the_subject", |args: &[Py]| _a_describing_word_is_the_subject(args)));
    pyrt::xrefs::reg_fn("understanding", "_stand_in_here", py_func("_stand_in_here", |args: &[Py]| _stand_in_here(args)));
    pyrt::xrefs::reg_fn("understanding", "_with_the_fault_lent", py_func("_with_the_fault_lent", |args: &[Py]| _with_the_fault_lent(args)));
    pyrt::xrefs::reg_fn("understanding", "_turns", py_func("_turns", |args: &[Py]| _turns(args)));
    pyrt::xrefs::reg_fn("understanding", "_user_lines", py_func("_user_lines", |args: &[Py]| _user_lines(args)));
    pyrt::xrefs::reg_fn("understanding", "_pointable", py_func("_pointable", |args: &[Py]| _pointable(args)));
    pyrt::xrefs::reg_fn("understanding", "_pointed_at", py_func("_pointed_at", |args: &[Py]| _pointed_at(args)));
    pyrt::xrefs::reg_fn("understanding", "_the_recorded_subject", py_func("_the_recorded_subject", |args: &[Py]| _the_recorded_subject(args)));
    pyrt::xrefs::reg_fn("understanding", "_recent_topic", py_func("_recent_topic", |args: &[Py]| _recent_topic(args)));
    pyrt::xrefs::reg_fn("understanding", "_changes_subject", py_func("_changes_subject", |args: &[Py]| _changes_subject(args)));
    pyrt::xrefs::reg_fn("understanding", "_names_a_bot_noun", py_func("_names_a_bot_noun", |args: &[Py]| _names_a_bot_noun(args)));
    pyrt::xrefs::reg_fn("understanding", "_a_stop_in_a_chat_about_something", py_func("_a_stop_in_a_chat_about_something", |args: &[Py]| _a_stop_in_a_chat_about_something(args)));
    pyrt::xrefs::reg_fn("understanding", "_about_the_bot", py_func("_about_the_bot", |args: &[Py]| _about_the_bot(args)));
    pyrt::xrefs::reg_fn("understanding", "_the_speaker_is_the_subject", py_func("_the_speaker_is_the_subject", |args: &[Py]| _the_speaker_is_the_subject(args)));
    pyrt::xrefs::reg_fn("understanding", "_a_word_that_depends_on_the_owner", py_func("_a_word_that_depends_on_the_owner", |args: &[Py]| _a_word_that_depends_on_the_owner(args)));
    pyrt::xrefs::reg_fn("understanding", "_opens_the_conversation", py_func("_opens_the_conversation", |args: &[Py]| _opens_the_conversation(args)));
    pyrt::xrefs::reg_fn("understanding", "asks_with_nothing_named", py_func("asks_with_nothing_named", |args: &[Py]| asks_with_nothing_named(args)));
    pyrt::xrefs::reg_fn("understanding", "_presence", py_func("_presence", |args: &[Py]| _presence(args)));
    pyrt::xrefs::reg_fn("understanding", "_a_manner", py_func("_a_manner", |args: &[Py]| _a_manner(args)));
    pyrt::xrefs::reg_fn("understanding", "_not_manner", py_func("_not_manner", |args: &[Py]| _not_manner(args)));
    pyrt::xrefs::reg_fn("understanding", "a_load_carried", py_func("a_load_carried", |args: &[Py]| a_load_carried(args)));
    pyrt::xrefs::reg_fn("understanding", "_asks_for_a_moment", py_func("_asks_for_a_moment", |args: &[Py]| _asks_for_a_moment(args)));
    pyrt::xrefs::reg_fn("understanding", "_keywords", py_func("_keywords", |args: &[Py]| _keywords(args)));
    pyrt::xrefs::reg_fn("understanding", "_asks_a_property_and_nothing_else", py_func("_asks_a_property_and_nothing_else", |args: &[Py]| _asks_a_property_and_nothing_else(args)));
    pyrt::xrefs::reg_fn("understanding", "_the_chat_subject", py_func("_the_chat_subject", |args: &[Py]| _the_chat_subject(args)));
    pyrt::xrefs::reg_fn("understanding", "_the_meaning_is_applied_to_what_was_said", py_func("_the_meaning_is_applied_to_what_was_said", |args: &[Py]| _the_meaning_is_applied_to_what_was_said(args)));
    pyrt::xrefs::reg_fn("understanding", "_leans_on", py_func("_leans_on", |args: &[Py]| _leans_on(args)));
    pyrt::xrefs::reg_fn("understanding", "_about_a_person_or_a_trouble", py_func("_about_a_person_or_a_trouble", |args: &[Py]| _about_a_person_or_a_trouble(args)));
    pyrt::xrefs::reg_fn("understanding", "_about_something_of_its_own", py_func("_about_something_of_its_own", |args: &[Py]| _about_something_of_its_own(args)));
    pyrt::xrefs::reg_fn("understanding", "_the_new_thing", py_func("_the_new_thing", |args: &[Py]| _the_new_thing(args)));
    pyrt::xrefs::reg_fn("understanding", "_the_same_kind", py_func("_the_same_kind", |args: &[Py]| _the_same_kind(args)));
    pyrt::xrefs::reg_fn("understanding", "_where_the_swap_goes", py_func("_where_the_swap_goes", |args: &[Py]| _where_the_swap_goes(args)));
    pyrt::xrefs::reg_fn("understanding", "_the_swap_is_said", py_func("_the_swap_is_said", |args: &[Py]| _the_swap_is_said(args)));
    pyrt::xrefs::reg_fn("understanding", "_the_line_before", py_func("_the_line_before", |args: &[Py]| _the_line_before(args)));
    pyrt::xrefs::reg_fn("understanding", "_asked_again", py_func("_asked_again", |args: &[Py]| _asked_again(args)));
    pyrt::xrefs::reg_fn("understanding", "_the_spans_of", py_func("_the_spans_of", |args: &[Py]| _the_spans_of(args)));
    pyrt::xrefs::reg_fn("understanding", "_the_thing_the_ask_names", py_func("_the_thing_the_ask_names", |args: &[Py]| _the_thing_the_ask_names(args)));
    pyrt::xrefs::reg_fn("understanding", "understand", py_func("understand", |args: &[Py]| understand(args)));
    pyrt::xrefs::reg_fn("understanding", "line_as_read", py_func("line_as_read", |args: &[Py]| line_as_read(args)));
    pyrt::xrefs::reg_fn("understanding", "_the_ask_inside_an_errand", py_func("_the_ask_inside_an_errand", |args: &[Py]| _the_ask_inside_an_errand(args)));
    pyrt::xrefs::reg_fn("understanding", "_reaches", py_func("_reaches", |args: &[Py]| _reaches(args)));
    pyrt::xrefs::reg_fn("understanding", "_an_introduction_that_says_how_they_are", py_func("_an_introduction_that_says_how_they_are", |args: &[Py]| _an_introduction_that_says_how_they_are(args)));
    pyrt::xrefs::reg_fn("understanding", "_said_of_themselves", py_func("_said_of_themselves", |args: &[Py]| _said_of_themselves(args)));
    pyrt::xrefs::reg_fn("understanding", "_confused_text", py_func("_confused_text", |args: &[Py]| _confused_text(args)));
    pyrt::xrefs::reg_fn("understanding", "_a_constraint_on_the_thing", py_func("_a_constraint_on_the_thing", |args: &[Py]| _a_constraint_on_the_thing(args)));
    pyrt::xrefs::reg_fn("understanding", "_read_line", py_func("_read_line", |args: &[Py]| _read_line(args)));
    pyrt::xrefs::reg_access("understanding", "ABOUT_THE_BOT", g_ABOUT_THE_BOT, s_ABOUT_THE_BOT);
    pyrt::xrefs::reg_access("understanding", "AFTER_BOT_FILLER", g_AFTER_BOT_FILLER, s_AFTER_BOT_FILLER);
    pyrt::xrefs::reg_access("understanding", "ALL_ON_THEM", g_ALL_ON_THEM, s_ALL_ON_THEM);
    pyrt::xrefs::reg_access("understanding", "ANY_PRONOUN", g_ANY_PRONOUN, s_ANY_PRONOUN);
    pyrt::xrefs::reg_access("understanding", "ASKING_WORDS", g_ASKING_WORDS, s_ASKING_WORDS);
    pyrt::xrefs::reg_access("understanding", "ASKS_WHY_IT_IS", g_ASKS_WHY_IT_IS, s_ASKS_WHY_IT_IS);
    pyrt::xrefs::reg_access("understanding", "ASK_FREE_FILLER", g_ASK_FREE_FILLER, s_ASK_FREE_FILLER);
    pyrt::xrefs::reg_access("understanding", "ASK_FREE_WORDS", g_ASK_FREE_WORDS, s_ASK_FREE_WORDS);
    pyrt::xrefs::reg_access("understanding", "ASK_PERMISSION", g_ASK_PERMISSION, s_ASK_PERMISSION);
    pyrt::xrefs::reg_access("understanding", "AT_IT_STILL", g_AT_IT_STILL, s_AT_IT_STILL);
    pyrt::xrefs::reg_access("understanding", "A_CLAUSE_VERB", g_A_CLAUSE_VERB, s_A_CLAUSE_VERB);
    pyrt::xrefs::reg_access("understanding", "A_CONDITION_AFTER", g_A_CONDITION_AFTER, s_A_CONDITION_AFTER);
    pyrt::xrefs::reg_access("understanding", "A_DESCRIBING_SUBJECT", g_A_DESCRIBING_SUBJECT, s_A_DESCRIBING_SUBJECT);
    pyrt::xrefs::reg_access("understanding", "A_FAULT_TO_LEND", g_A_FAULT_TO_LEND, s_A_FAULT_TO_LEND);
    pyrt::xrefs::reg_access("understanding", "A_STAND_IN", g_A_STAND_IN, s_A_STAND_IN);
    pyrt::xrefs::reg_access("understanding", "A_VALUE_IN_FRONT_OF_NOT", g_A_VALUE_IN_FRONT_OF_NOT, s_A_VALUE_IN_FRONT_OF_NOT);
    pyrt::xrefs::reg_access("understanding", "BE_VERB", g_BE_VERB, s_BE_VERB);
    pyrt::xrefs::reg_access("understanding", "BOT_ASK_WORDS", g_BOT_ASK_WORDS, s_BOT_ASK_WORDS);
    pyrt::xrefs::reg_access("understanding", "BOT_NOUNS", g_BOT_NOUNS, s_BOT_NOUNS);
    pyrt::xrefs::reg_access("understanding", "BOT_STATE", g_BOT_STATE, s_BOT_STATE);
    pyrt::xrefs::reg_access("understanding", "BROKEN_OF_A_THING", g_BROKEN_OF_A_THING, s_BROKEN_OF_A_THING);
    pyrt::xrefs::reg_access("understanding", "CARRY_PASS", g_CARRY_PASS, s_CARRY_PASS);
    pyrt::xrefs::reg_access("understanding", "CARRY_TOPIC", g_CARRY_TOPIC, s_CARRY_TOPIC);
    pyrt::xrefs::reg_access("understanding", "CARRY_WORDS", g_CARRY_WORDS, s_CARRY_WORDS);
    pyrt::xrefs::reg_access("understanding", "CHAT_WORDS", g_CHAT_WORDS, s_CHAT_WORDS);
    pyrt::xrefs::reg_access("understanding", "CONSTRAINT_FACT", g_CONSTRAINT_FACT, s_CONSTRAINT_FACT);
    pyrt::xrefs::reg_access("understanding", "COPULA_AUX", g_COPULA_AUX, s_COPULA_AUX);
    pyrt::xrefs::reg_access("understanding", "CORRECTION_HEAD", g_CORRECTION_HEAD, s_CORRECTION_HEAD);
    pyrt::xrefs::reg_access("understanding", "CORRECTION_MARKER", g_CORRECTION_MARKER, s_CORRECTION_MARKER);
    pyrt::xrefs::reg_access("understanding", "CORRECTION_OPENER", g_CORRECTION_OPENER, s_CORRECTION_OPENER);
    pyrt::xrefs::reg_access("understanding", "DAY_VERBS", g_DAY_VERBS, s_DAY_VERBS);
    pyrt::xrefs::reg_access("understanding", "DAY_WORDS", g_DAY_WORDS, s_DAY_WORDS);
    pyrt::xrefs::reg_access("understanding", "DETERMINER_TAIL", g_DETERMINER_TAIL, s_DETERMINER_TAIL);
    pyrt::xrefs::reg_access("understanding", "DID_IT", g_DID_IT, s_DID_IT);
    pyrt::xrefs::reg_access("understanding", "DOES_ALL_THEY_CAN", g_DOES_ALL_THEY_CAN, s_DOES_ALL_THEY_CAN);
    pyrt::xrefs::reg_access("understanding", "DOES_IT_ALL", g_DOES_IT_ALL, s_DOES_IT_ALL);
    pyrt::xrefs::reg_access("understanding", "DOES_IT_RIGHT", g_DOES_IT_RIGHT, s_DOES_IT_RIGHT);
    pyrt::xrefs::reg_access("understanding", "EDGE_PUNCTUATION", g_EDGE_PUNCTUATION, s_EDGE_PUNCTUATION);
    pyrt::xrefs::reg_access("understanding", "EVERYTHING_AT_ONCE", g_EVERYTHING_AT_ONCE, s_EVERYTHING_AT_ONCE);
    pyrt::xrefs::reg_access("understanding", "FIRST_PERSON_ANYWHERE", g_FIRST_PERSON_ANYWHERE, s_FIRST_PERSON_ANYWHERE);
    pyrt::xrefs::reg_access("understanding", "FOLLOW_OPENER", g_FOLLOW_OPENER, s_FOLLOW_OPENER);
    pyrt::xrefs::reg_access("understanding", "GOOD_TIME_WORDS", g_GOOD_TIME_WORDS, s_GOOD_TIME_WORDS);
    pyrt::xrefs::reg_access("understanding", "GREETING_PHRASE", g_GREETING_PHRASE, s_GREETING_PHRASE);
    pyrt::xrefs::reg_access("understanding", "HELPS_ME_NOT", g_HELPS_ME_NOT, s_HELPS_ME_NOT);
    pyrt::xrefs::reg_access("understanding", "HOME_LOOKBACK", g_HOME_LOOKBACK, s_HOME_LOOKBACK);
    pyrt::xrefs::reg_access("understanding", "HOME_SCOPE", g_HOME_SCOPE, s_HOME_SCOPE);
    pyrt::xrefs::reg_access("understanding", "HYPOTHETICAL", g_HYPOTHETICAL, s_HYPOTHETICAL);
    pyrt::xrefs::reg_access("understanding", "INTRODUCING_A_STATE", g_INTRODUCING_A_STATE, s_INTRODUCING_A_STATE);
    pyrt::xrefs::reg_access("understanding", "LEANS_ON_THEM", g_LEANS_ON_THEM, s_LEANS_ON_THEM);
    pyrt::xrefs::reg_access("understanding", "LOOKBACK", g_LOOKBACK, s_LOOKBACK);
    pyrt::xrefs::reg_access("understanding", "MANNER_TALK", g_MANNER_TALK, s_MANNER_TALK);
    pyrt::xrefs::reg_access("understanding", "MAX_KEYWORDS", g_MAX_KEYWORDS, s_MAX_KEYWORDS);
    pyrt::xrefs::reg_access("understanding", "MEANING_TAIL", g_MEANING_TAIL, s_MEANING_TAIL);
    pyrt::xrefs::reg_access("understanding", "MEANING_WORDS", g_MEANING_WORDS, s_MEANING_WORDS);
    pyrt::xrefs::reg_access("understanding", "MODAL_STATE", g_MODAL_STATE, s_MODAL_STATE);
    pyrt::xrefs::reg_access("understanding", "MODAL_WORDS", g_MODAL_WORDS, s_MODAL_WORDS);
    pyrt::xrefs::reg_access("understanding", "MOMENT_FILLER", g_MOMENT_FILLER, s_MOMENT_FILLER);
    pyrt::xrefs::reg_access("understanding", "MOMENT_VERBS", g_MOMENT_VERBS, s_MOMENT_VERBS);
    pyrt::xrefs::reg_access("understanding", "MOMENT_WORDS", g_MOMENT_WORDS, s_MOMENT_WORDS);
    pyrt::xrefs::reg_access("understanding", "MORE_WORDS", g_MORE_WORDS, s_MORE_WORDS);
    pyrt::xrefs::reg_access("understanding", "NOTHING_NAMED_ASK_WORDS", g_NOTHING_NAMED_ASK_WORDS, s_NOTHING_NAMED_ASK_WORDS);
    pyrt::xrefs::reg_access("understanding", "NOTHING_NAMED_FILLER", g_NOTHING_NAMED_FILLER, s_NOTHING_NAMED_FILLER);
    pyrt::xrefs::reg_access("understanding", "NOTHING_NAMED_VIEW_WORDS", g_NOTHING_NAMED_VIEW_WORDS, s_NOTHING_NAMED_VIEW_WORDS);
    pyrt::xrefs::reg_access("understanding", "NOT_A_SUBJECT", g_NOT_A_SUBJECT, s_NOT_A_SUBJECT);
    pyrt::xrefs::reg_access("understanding", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("understanding", "NOT_MANNER", g_NOT_MANNER, s_NOT_MANNER);
    pyrt::xrefs::reg_access("understanding", "NO_ONE_HELPS", g_NO_ONE_HELPS, s_NO_ONE_HELPS);
    pyrt::xrefs::reg_access("understanding", "NO_TIME_OFF", g_NO_TIME_OFF, s_NO_TIME_OFF);
    pyrt::xrefs::reg_access("understanding", "OTHER_PRONOUNS", g_OTHER_PRONOUNS, s_OTHER_PRONOUNS);
    pyrt::xrefs::reg_access("understanding", "OWN_QUESTION_TOPIC", g_OWN_QUESTION_TOPIC, s_OWN_QUESTION_TOPIC);
    pyrt::xrefs::reg_access("understanding", "OWN_VERB", g_OWN_VERB, s_OWN_VERB);
    pyrt::xrefs::reg_access("understanding", "PERMISSION_FILLER", g_PERMISSION_FILLER, s_PERMISSION_FILLER);
    pyrt::xrefs::reg_access("understanding", "PERSON_HEADS", g_PERSON_HEADS, s_PERSON_HEADS);
    pyrt::xrefs::reg_access("understanding", "PERSON_STOP", g_PERSON_STOP, s_PERSON_STOP);
    pyrt::xrefs::reg_access("understanding", "POSSESSIVE", g_POSSESSIVE, s_POSSESSIVE);
    pyrt::xrefs::reg_access("understanding", "PRAISED", g_PRAISED, s_PRAISED);
    pyrt::xrefs::reg_access("understanding", "PRESENCE_ASKER", g_PRESENCE_ASKER, s_PRESENCE_ASKER);
    pyrt::xrefs::reg_access("understanding", "PRESENCE_FILLER", g_PRESENCE_FILLER, s_PRESENCE_FILLER);
    pyrt::xrefs::reg_access("understanding", "PRESENCE_GREETING", g_PRESENCE_GREETING, s_PRESENCE_GREETING);
    pyrt::xrefs::reg_access("understanding", "PRESENCE_MAX", g_PRESENCE_MAX, s_PRESENCE_MAX);
    pyrt::xrefs::reg_access("understanding", "PRESENCE_NOT", g_PRESENCE_NOT, s_PRESENCE_NOT);
    pyrt::xrefs::reg_access("understanding", "PRESENCE_SUBJECT", g_PRESENCE_SUBJECT, s_PRESENCE_SUBJECT);
    pyrt::xrefs::reg_access("understanding", "PRESENCE_WORD", g_PRESENCE_WORD, s_PRESENCE_WORD);
    pyrt::xrefs::reg_access("understanding", "PROPERTY_ASK", g_PROPERTY_ASK, s_PROPERTY_ASK);
    pyrt::xrefs::reg_access("understanding", "PROPERTY_SUBJECTS", g_PROPERTY_SUBJECTS, s_PROPERTY_SUBJECTS);
    pyrt::xrefs::reg_access("understanding", "REACHING_KEYS", g_REACHING_KEYS, s_REACHING_KEYS);
    pyrt::xrefs::reg_access("understanding", "RUNS_MYSELF_RAGGED", g_RUNS_MYSELF_RAGGED, s_RUNS_MYSELF_RAGGED);
    pyrt::xrefs::reg_access("understanding", "RUNS_THE_HOUSE", g_RUNS_THE_HOUSE, s_RUNS_THE_HOUSE);
    pyrt::xrefs::reg_access("understanding", "SAME_KIND_WORDS", g_SAME_KIND_WORDS, s_SAME_KIND_WORDS);
    pyrt::xrefs::reg_access("understanding", "SECOND_JOIN_HEAD", g_SECOND_JOIN_HEAD, s_SECOND_JOIN_HEAD);
    pyrt::xrefs::reg_access("understanding", "SECOND_JOIN_TAIL", g_SECOND_JOIN_TAIL, s_SECOND_JOIN_TAIL);
    pyrt::xrefs::reg_access("understanding", "SECOND_PERSON", g_SECOND_PERSON, s_SECOND_PERSON);
    pyrt::xrefs::reg_access("understanding", "SHIFTS", g_SHIFTS, s_SHIFTS);
    pyrt::xrefs::reg_access("understanding", "SHIFT_ACTUALLY", g_SHIFT_ACTUALLY, s_SHIFT_ACTUALLY);
    pyrt::xrefs::reg_access("understanding", "SPEAKER_SUBJECTS", g_SPEAKER_SUBJECTS, s_SPEAKER_SUBJECTS);
    pyrt::xrefs::reg_access("understanding", "STAND_IN_NOT_THINGS", g_STAND_IN_NOT_THINGS, s_STAND_IN_NOT_THINGS);
    pyrt::xrefs::reg_access("understanding", "STAND_IN_SELECTORS", g_STAND_IN_SELECTORS, s_STAND_IN_SELECTORS);
    pyrt::xrefs::reg_access("understanding", "STAND_IN_WORDS", g_STAND_IN_WORDS, s_STAND_IN_WORDS);
    pyrt::xrefs::reg_access("understanding", "STOPWORDS", g_STOPWORDS, s_STOPWORDS);
    pyrt::xrefs::reg_access("understanding", "SUBJECT_TYPES", g_SUBJECT_TYPES, s_SUBJECT_TYPES);
    pyrt::xrefs::reg_access("understanding", "TALK_ASK", g_TALK_ASK, s_TALK_ASK);
    pyrt::xrefs::reg_access("understanding", "TALK_FILLER", g_TALK_FILLER, s_TALK_FILLER);
    pyrt::xrefs::reg_access("understanding", "TALK_VERBS", g_TALK_VERBS, s_TALK_VERBS);
    pyrt::xrefs::reg_access("understanding", "THANKED_FOR", g_THANKED_FOR, s_THANKED_FOR);
    pyrt::xrefs::reg_access("understanding", "THE_ONE_WHO", g_THE_ONE_WHO, s_THE_ONE_WHO);
    pyrt::xrefs::reg_access("understanding", "THIRD_PERSON", g_THIRD_PERSON, s_THIRD_PERSON);
    pyrt::xrefs::reg_access("understanding", "WHO_ENDS", g_WHO_ENDS, s_WHO_ENDS);
    pyrt::xrefs::reg_access("understanding", "WORK_SCOPE", g_WORK_SCOPE, s_WORK_SCOPE);
    pyrt::xrefs::reg_access("understanding", "WORN_OUT_OF_A_PERSON", g_WORN_OUT_OF_A_PERSON, s_WORN_OUT_OF_A_PERSON);
    pyrt::xrefs::reg_access("understanding", "_A_CLAUSE_WITH_NO_SUBJECT", g__A_CLAUSE_WITH_NO_SUBJECT, s__A_CLAUSE_WITH_NO_SUBJECT);
    pyrt::xrefs::reg_access("understanding", "_A_THING_THAT_CHANGES", g__A_THING_THAT_CHANGES, s__A_THING_THAT_CHANGES);
    pyrt::xrefs::reg_access("understanding", "_NOT_ABOUT_THE_THING", g__NOT_ABOUT_THE_THING, s__NOT_ABOUT_THE_THING);
    pyrt::xrefs::reg_access("understanding", "_SEARCH_FRAME", g__SEARCH_FRAME, s__SEARCH_FRAME);
}

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


pub static ABILITY_AT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABOUT_AT_A_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABOUT_BOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABOUT_BOT_END: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACTS_ON_THE_SPEAKER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACT_ON_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACT_ON_LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ADDRESSED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ADDRESSED_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ADDRESS_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AFFIRM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AFTER_COMPARATIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AFTER_HELP_WAS_GOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AFTER_HELP_WAS_GOOD_SAID_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AGREEMENT_IDIOMS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AGREE_AFTER_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AGREE_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ALREADY_ASKED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ALWAYS_SELF_ONLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ANSWERING_A_QUOTED_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ANY_ALREADY_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ANY_ANSWER_LINKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ANY_ANSWER_WANTED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKING_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKING_VERBS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_ABOUT_A_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_IF_ANYTHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_IT_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_ABOUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_AUX: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_FOLLOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_FOR_A_NAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_FRAMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_KEYS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_MODALS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_TARGET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_THE_ROOM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_THE_ROOM_ANY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_THE_ROOM_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_THE_ROOM_WANTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_VERBS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASSISTANT_NAMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_BODY_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_BROKEN_HEART: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_COMPARISON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DATE_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FRAME_IN_FRONT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HAVING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HOW_MANY_OF_THE_BOTS_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_LIMIT_ABOUT_PEOPLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_MONEY_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_MONTH_PERIOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_NOT_A_SUGGESTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PERIOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PERSON_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PERSON_AFTER_A_POSSESSIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PERSON_ELSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PERSON_IS_THE_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PERSON_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_POSSESSED_HEART: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_REPORTED_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SAID_BY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SAID_BY_ANYONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SUGGESTED_CLAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_THING_NOT_A_CLAIM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_TOLD_TO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_WHAT_ITS_LIKE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_YOUR_KIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_YOUR_OWN_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BACK_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BACK_ASKING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BACK_AT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BACK_INTO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BACK_JOIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BACK_NOT_A_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_DEFINITION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_VERBS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BEEN_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BEEN_WHEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BEFORE_COURTESY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BEFORE_HELP_WAS_GOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BEFORE_STATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BETTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BETTER_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BETWEEN_STATE_AND_NOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BE_FORMS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BODY_STATE_CUES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BODY_STATE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BODY_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BOT_CHATS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BOT_COUNTED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BOT_DAY_SHAPES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BOT_DAY_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BOT_DOERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BOT_DOINGS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BOT_HAVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BOT_LIFE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BOT_LIFE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BOT_LIFE_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BOUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BROKEN_HEARTED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BROKE_A_HEART: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CAPABILITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CAPABILITY_CARRIER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CAPABILITY_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARE_HOME_BRING_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARE_HOME_BRING_WHO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARE_HOME_BRING_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARE_HOME_GUILT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARE_HOME_KIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARE_HOME_MOVED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARE_HOME_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARE_HOME_PLACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARE_HOME_VISIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHILD_NOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHOICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLAIM_ENDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLAIM_STEMS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLAUSE_HELPERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLAUSE_OPENERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLAUSE_VERBS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMMIT_ALONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMMIT_ALONE_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMMIT_NO_HARM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMMIT_OPEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMMIT_STOP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMMIT_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMMIT_TRY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMPARATIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMPARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMPARISON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONFUSED_SELF_ONLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONJUNCTIONS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COPULA: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COPULAS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COPULA_BEFORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COPULA_STATES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CORRECTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COUNT_FILLERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COUNT_OPENERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COURTESY_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COURTESY_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CUE_FOLLOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEFINITION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEGREE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DENY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DENY_BEFORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DENY_BETWEEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DESCRIBES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DETERMINERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOING_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ECHO_CARRIERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EMOTION_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EPISODE_NOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EVENT_NOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EVENT_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EXPERIENCE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EXPERIENCE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FAREWELLS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FAREWELL_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_COPULA: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_CUES_BEFORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_NOT_MOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_SOUND_FRAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FELL_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FILLER_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FILLER_BEFORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIRST_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOUND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRAME_NOT_A_SETUP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FREE_OR_BUSY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GENDER_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GETTING_WORSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GIVEN_ADVICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GOING_ON_WHEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GOOD_EMOTION_NAMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GOOD_WORD_BY_COMPARISON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GOOD_WORD_COMPARED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GOOD_WORD_MISSING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GRATITUDE_ASK_AGAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GRATITUDE_JOINERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GRATITUDE_OBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GREETINGS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GREETING_ASK_ABOUT_SOMETHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GREETING_ASK_SECOND_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GREETING_ASK_STATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GREETING_ASK_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GREETING_ASK_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GREETING_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GREETING_WHAT_SAYING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HAND_OVER_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HAND_OVER_MINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HAND_OVER_MODALS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HAND_OVER_SUBJECTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HAND_OVER_TO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HAND_OVER_VERBS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HAVE_A_BROKEN_HEART: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEART_DEGREES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEART_IN_A_STATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEDGED_ALONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEDGED_COMMIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEDGED_DOUBT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEDGED_LOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEDGED_OK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEDGED_WRAPPER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEDGE_LEAD_ONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEDGE_LEAD_TWO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELP_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELP_ASK_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELP_ASK_OWN_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELP_ASK_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELP_ASK_TAIL_CLAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELP_WAS_GOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IDIOM_STATES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ING_TAKES_NO_OBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ING_THINGS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static INTEREST_HEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static INTEREST_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static INTEREST_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static INTRODUCES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ITSELF_LIFE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ITSELF_LONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ITSELF_WHEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ITSELF_WHEN_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static JOIN_OPENERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEPING_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEPS_BUSY_SHAPE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LABEL_ON_A_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LAUGH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LAUGHED_OFF_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LAUGH_AT_THE_END: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEARNED_ON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEARNING_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LETTER_ASKED_FOR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LETTER_ITSELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LETTER_RECEIVED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LETTER_SOMEONE_ELSES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIMIT_A_FACT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIMIT_NOT_A_START: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIMIT_NOT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIMIT_REFUSAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIMIT_ROOM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIMIT_SHAPES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIMIT_SOFT_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIMIT_SOFT_BEFORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MAKER_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MANIPULATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MANNERS_OPENERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MANNERS_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MATH_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEANT_APPLIED_TO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ME_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ME_LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ME_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOOD_NOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOOD_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOOD_TAIL_TAKEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOOD_TAIL_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOTION_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMING_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEEDS_ANSWER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEGATIONS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEGATION_SKIP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEGATIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEGATORS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEWS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEW_ASK_AHEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEW_ASK_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEW_FRAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_AFTER_THE_BOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_AUX: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_DEFINITION_HEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_MOOD_THERE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_BEFORE_THE_BOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_THEIR_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_THE_SPEAKERS_DAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOUN_REQUEST_MAX: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOUN_REQUEST_SELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOUN_STATES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_BETTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_HARM_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NUMBER_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OBJECT_MARKERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OK_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OPENING_FRAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OPENING_MAX: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OPENING_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OPENS_A_FRAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OPINION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OPPOSITE_PAIRS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OTHER_PEOPLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWNS_A_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_PERIOD_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_PERIOD_HEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_PERIOD_NOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_PERIOD_REACH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_PERIOD_STOPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_RESERVES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PARTICLE_STATES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PARTS_OF_ME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAST_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAST_AUX: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAST_ENDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERIOD_CUES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON_NOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON_PREPOSITIONS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PHRASE_OPENERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PICKING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PICKING_QUALITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLACE_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POINTED_AT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POLITE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POSITIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POSSESSIVE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRAISE_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRAISING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRAISING_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRODUCT_ASK_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRODUCT_ASK_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRONOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRONOUN_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUESTION_NOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUESTION_STARTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUESTION_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUESTION_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECOMMEND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECOMMEND_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REFERRING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REFUSED_PAIR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RELATIVE_SUBJECTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REQUEST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ROBOT_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ROUTINE_IN_THE_PAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ROUTINE_IS_A_FAULT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ROUTINE_SHAPE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RULED_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RULED_OUT_CORRECTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RULED_OUT_ENDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RULED_OUT_NO_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAD_PLAIN_CUES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_AS_A_NEGATION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAYS_ABOUT_YOU: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAYS_HOW_THEY_ARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAYS_WHAT_ARE_YOU: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAYS_WHAT_IT_IS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SCALE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_ONLY_CUES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_ONLY_STATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SETUP_CONNECTIVES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SETUP_MOST_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SETUP_NOTHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHIFT_EARLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHIFT_LATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHIFT_NIGHT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHORT_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SIGNING_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLIPPED_FROM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SMALL_TALK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SMALL_TALK_LEFTOVER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SMALL_TALK_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOUND_FOLLOWS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPEAKER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPEAKER_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPENDS_ITS_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPENT_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STARTED_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STATE_AS_FEELING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STATE_JOINERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STATE_NOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STATE_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STATE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STATING_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEPS_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_ANY_OBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_VERBS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_WRONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_COMING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_WRONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STOMACH_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STOP_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STOP_ASK_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUBJECT_NEGATIONS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUBJECT_OF_ITS_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUMMONS_CALL_DONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUMMONS_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUMMONS_RANG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUMMONS_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUMMONS_SEE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUMMONS_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUMMONS_WANTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUMMONS_WHO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUMMONS_WHO_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUMMONS_WRITTEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUPPLY_PREPOSITIONS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUPPLY_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAG_CHECKABLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAG_ENDINGS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAG_PLAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAG_RIGHT_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAG_TIME_NUMBER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAIL_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAIL_ASK_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAIL_STOPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TALK_ASK_OPENERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TALK_ASK_VERBS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TALK_HEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TALK_OPENING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TALK_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TALK_TAIL_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TASTE_VERBS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TERMS_OF_ADDRESS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THANKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THANKS_ABOUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THANKS_ASK_NOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THANKS_ASK_PREPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THANKS_ASK_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THANKS_NOISE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIR_PROBLEM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THING_OR_PERSON_CUES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIME_AFTER_A_PARTICLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIME_FILLERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIME_MARKERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIPS_ABOUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIPS_AMOUNT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIPS_FRAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIPS_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIPS_WANTED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIP_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TO_A_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TURNED_ROUND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERBISH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERB_FORMS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERB_ONLY_STATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERB_THIRD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERDICT_ADJECTIVES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERDICT_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERDICT_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERDICT_NOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_IN_THE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHERE_BOT_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHILE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHO_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHO_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHO_TALKING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WH_CONTINUES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORK_ON_ME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WRITE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static YESNO_STARTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static YOU_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASKS_AFTER_A_COMMA: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASKS_AFTER_A_COMMA_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASKS_MID_LINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASKS_WITH_AN_AUXILIARY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASKS_WITH_A_QUESTION_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AVAILABILITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_AN_AMOUNT_OF_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_AN_OCCASION_OF_THEIR_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_ARITHMETIC: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_A_CONVERSION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_A_DIFFERENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_A_PLAN_ABOUT_THE_WEATHER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_A_TRAFFIC_LIGHT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_BARE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_BARE_LIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_BARE_LIVE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_BARE_RECORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_BARE_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_CLASS_OF_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_COUNT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_DAY_OR_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_DAY_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_DEFINITION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_DETERMINER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_DID_YOU_SEE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_FOLLOW_UP_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_FRESHENING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HAS_A_NUMBER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HISTORY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HISTORY_ITSELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HOW_A_THING_MEASURES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HOW_IT_WORKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HOW_MANY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HOW_MUCH_A_FEELING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HOW_MUCH_ENOUGH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HOW_MUCH_IS_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HOW_MUCH_IT_HOLDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HOW_MUCH_I_SPENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HOW_MUCH_OF_A_UNIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HOW_OLD_IS_THE_WORLD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_LINE_THAT_NAMES_A_PLACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_MADE_OF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_MEASURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_METHOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_MY_OWN_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_MY_OWN_PLAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_PAST_YEAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_PLACE_THAT_KEEPS_HOURS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_POST_HOLDER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_PRICE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_RECORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_REPORTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_REPORTS_THE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_SAYS_NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_SERVICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_SERVICE_STATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_THE_CLOCK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_WEATHER_HERE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_WORD_MEANS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_WORKING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BARE_LIVE_LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CHANGING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _EVENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _HOURS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _I_SAID_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MONEY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MONEY_ASKED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MONEY_FOR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MONEY_NOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NEWS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NIGHT_WISHES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_A_FITTING_THAT_OPENS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_OPEN_TO_SOMEBODY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _RATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ROUTINE_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SAID_ON_ITS_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SEALED_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SERVICE_ON_A_DAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SETTLED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SOUND_CUES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THANKS_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _TIMETABLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _UNITS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _UNITS_BOUGHT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WEATHER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WISH_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ABILITY_AT() -> Py { ensure_init(); ABILITY_AT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABILITY_AT(v: Py) { ensure_init(); if let Ok(mut w) = ABILITY_AT.write() { *w = v; } }
pub fn g_ABOUT_AT_A_TIME() -> Py { ensure_init(); ABOUT_AT_A_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOUT_AT_A_TIME(v: Py) { ensure_init(); if let Ok(mut w) = ABOUT_AT_A_TIME.write() { *w = v; } }
pub fn g_ABOUT_BOT() -> Py { ensure_init(); ABOUT_BOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOUT_BOT(v: Py) { ensure_init(); if let Ok(mut w) = ABOUT_BOT.write() { *w = v; } }
pub fn g_ABOUT_BOT_END() -> Py { ensure_init(); ABOUT_BOT_END.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOUT_BOT_END(v: Py) { ensure_init(); if let Ok(mut w) = ABOUT_BOT_END.write() { *w = v; } }
pub fn g_ACK() -> Py { ensure_init(); ACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACK(v: Py) { ensure_init(); if let Ok(mut w) = ACK.write() { *w = v; } }
pub fn g_ACTS_ON_THE_SPEAKER() -> Py { ensure_init(); ACTS_ON_THE_SPEAKER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACTS_ON_THE_SPEAKER(v: Py) { ensure_init(); if let Ok(mut w) = ACTS_ON_THE_SPEAKER.write() { *w = v; } }
pub fn g_ACT_ON_FILLER() -> Py { ensure_init(); ACT_ON_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACT_ON_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = ACT_ON_FILLER.write() { *w = v; } }
pub fn g_ACT_ON_LEAD() -> Py { ensure_init(); ACT_ON_LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACT_ON_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = ACT_ON_LEAD.write() { *w = v; } }
pub fn g_ADDRESSED() -> Py { ensure_init(); ADDRESSED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ADDRESSED(v: Py) { ensure_init(); if let Ok(mut w) = ADDRESSED.write() { *w = v; } }
pub fn g_ADDRESSED_LINES() -> Py { ensure_init(); ADDRESSED_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ADDRESSED_LINES(v: Py) { ensure_init(); if let Ok(mut w) = ADDRESSED_LINES.write() { *w = v; } }
pub fn g_ADDRESS_WORDS() -> Py { ensure_init(); ADDRESS_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ADDRESS_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = ADDRESS_WORDS.write() { *w = v; } }
pub fn g_AFFIRM() -> Py { ensure_init(); AFFIRM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AFFIRM(v: Py) { ensure_init(); if let Ok(mut w) = AFFIRM.write() { *w = v; } }
pub fn g_AFTER_COMPARATIVE() -> Py { ensure_init(); AFTER_COMPARATIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AFTER_COMPARATIVE(v: Py) { ensure_init(); if let Ok(mut w) = AFTER_COMPARATIVE.write() { *w = v; } }
pub fn g_AFTER_HELP_WAS_GOOD() -> Py { ensure_init(); AFTER_HELP_WAS_GOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AFTER_HELP_WAS_GOOD(v: Py) { ensure_init(); if let Ok(mut w) = AFTER_HELP_WAS_GOOD.write() { *w = v; } }
pub fn g_AFTER_HELP_WAS_GOOD_SAID_OUT() -> Py { ensure_init(); AFTER_HELP_WAS_GOOD_SAID_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AFTER_HELP_WAS_GOOD_SAID_OUT(v: Py) { ensure_init(); if let Ok(mut w) = AFTER_HELP_WAS_GOOD_SAID_OUT.write() { *w = v; } }
pub fn g_AGREEMENT_IDIOMS() -> Py { ensure_init(); AGREEMENT_IDIOMS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AGREEMENT_IDIOMS(v: Py) { ensure_init(); if let Ok(mut w) = AGREEMENT_IDIOMS.write() { *w = v; } }
pub fn g_AGREE_AFTER_NOT() -> Py { ensure_init(); AGREE_AFTER_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AGREE_AFTER_NOT(v: Py) { ensure_init(); if let Ok(mut w) = AGREE_AFTER_NOT.write() { *w = v; } }
pub fn g_AGREE_TAIL() -> Py { ensure_init(); AGREE_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AGREE_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = AGREE_TAIL.write() { *w = v; } }
pub fn g_ALREADY_ASKED() -> Py { ensure_init(); ALREADY_ASKED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ALREADY_ASKED(v: Py) { ensure_init(); if let Ok(mut w) = ALREADY_ASKED.write() { *w = v; } }
pub fn g_ALWAYS_SELF_ONLY() -> Py { ensure_init(); ALWAYS_SELF_ONLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ALWAYS_SELF_ONLY(v: Py) { ensure_init(); if let Ok(mut w) = ALWAYS_SELF_ONLY.write() { *w = v; } }
pub fn g_ANSWERING_A_QUOTED_ASK() -> Py { ensure_init(); ANSWERING_A_QUOTED_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANSWERING_A_QUOTED_ASK(v: Py) { ensure_init(); if let Ok(mut w) = ANSWERING_A_QUOTED_ASK.write() { *w = v; } }
pub fn g_ANY_ALREADY_ASKS() -> Py { ensure_init(); ANY_ALREADY_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANY_ALREADY_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = ANY_ALREADY_ASKS.write() { *w = v; } }
pub fn g_ANY_ANSWER_LINKS() -> Py { ensure_init(); ANY_ANSWER_LINKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANY_ANSWER_LINKS(v: Py) { ensure_init(); if let Ok(mut w) = ANY_ANSWER_LINKS.write() { *w = v; } }
pub fn g_ANY_ANSWER_WANTED() -> Py { ensure_init(); ANY_ANSWER_WANTED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANY_ANSWER_WANTED(v: Py) { ensure_init(); if let Ok(mut w) = ANY_ANSWER_WANTED.write() { *w = v; } }
pub fn g_ASKING_SUBJECT() -> Py { ensure_init(); ASKING_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKING_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = ASKING_SUBJECT.write() { *w = v; } }
pub fn g_ASKING_VERBS() -> Py { ensure_init(); ASKING_VERBS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKING_VERBS(v: Py) { ensure_init(); if let Ok(mut w) = ASKING_VERBS.write() { *w = v; } }
pub fn g_ASKS_ABOUT_A_PERSON() -> Py { ensure_init(); ASKS_ABOUT_A_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_ABOUT_A_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_ABOUT_A_PERSON.write() { *w = v; } }
pub fn g_ASKS_IF_ANYTHING() -> Py { ensure_init(); ASKS_IF_ANYTHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_IF_ANYTHING(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_IF_ANYTHING.write() { *w = v; } }
pub fn g_ASKS_IT_BACK() -> Py { ensure_init(); ASKS_IT_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_IT_BACK(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_IT_BACK.write() { *w = v; } }
pub fn g_ASK_ABOUT() -> Py { ensure_init(); ASK_ABOUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_ABOUT(v: Py) { ensure_init(); if let Ok(mut w) = ASK_ABOUT.write() { *w = v; } }
pub fn g_ASK_AUX() -> Py { ensure_init(); ASK_AUX.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_AUX(v: Py) { ensure_init(); if let Ok(mut w) = ASK_AUX.write() { *w = v; } }
pub fn g_ASK_FOLLOW() -> Py { ensure_init(); ASK_FOLLOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_FOLLOW(v: Py) { ensure_init(); if let Ok(mut w) = ASK_FOLLOW.write() { *w = v; } }
pub fn g_ASK_FOR_A_NAME() -> Py { ensure_init(); ASK_FOR_A_NAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_FOR_A_NAME(v: Py) { ensure_init(); if let Ok(mut w) = ASK_FOR_A_NAME.write() { *w = v; } }
pub fn g_ASK_FRAMES() -> Py { ensure_init(); ASK_FRAMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_FRAMES(v: Py) { ensure_init(); if let Ok(mut w) = ASK_FRAMES.write() { *w = v; } }
pub fn g_ASK_KEYS() -> Py { ensure_init(); ASK_KEYS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_KEYS(v: Py) { ensure_init(); if let Ok(mut w) = ASK_KEYS.write() { *w = v; } }
pub fn g_ASK_MODALS() -> Py { ensure_init(); ASK_MODALS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_MODALS(v: Py) { ensure_init(); if let Ok(mut w) = ASK_MODALS.write() { *w = v; } }
pub fn g_ASK_TARGET() -> Py { ensure_init(); ASK_TARGET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_TARGET(v: Py) { ensure_init(); if let Ok(mut w) = ASK_TARGET.write() { *w = v; } }
pub fn g_ASK_THE_ROOM() -> Py { ensure_init(); ASK_THE_ROOM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_THE_ROOM(v: Py) { ensure_init(); if let Ok(mut w) = ASK_THE_ROOM.write() { *w = v; } }
pub fn g_ASK_THE_ROOM_ANY() -> Py { ensure_init(); ASK_THE_ROOM_ANY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_THE_ROOM_ANY(v: Py) { ensure_init(); if let Ok(mut w) = ASK_THE_ROOM_ANY.write() { *w = v; } }
pub fn g_ASK_THE_ROOM_VERB() -> Py { ensure_init(); ASK_THE_ROOM_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_THE_ROOM_VERB(v: Py) { ensure_init(); if let Ok(mut w) = ASK_THE_ROOM_VERB.write() { *w = v; } }
pub fn g_ASK_THE_ROOM_WANTS() -> Py { ensure_init(); ASK_THE_ROOM_WANTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_THE_ROOM_WANTS(v: Py) { ensure_init(); if let Ok(mut w) = ASK_THE_ROOM_WANTS.write() { *w = v; } }
pub fn g_ASK_VERBS() -> Py { ensure_init(); ASK_VERBS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_VERBS(v: Py) { ensure_init(); if let Ok(mut w) = ASK_VERBS.write() { *w = v; } }
pub fn g_ASSISTANT_NAMES() -> Py { ensure_init(); ASSISTANT_NAMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASSISTANT_NAMES(v: Py) { ensure_init(); if let Ok(mut w) = ASSISTANT_NAMES.write() { *w = v; } }
pub fn g_A_BODY_SUBJECT() -> Py { ensure_init(); A_BODY_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_BODY_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = A_BODY_SUBJECT.write() { *w = v; } }
pub fn g_A_BROKEN_HEART() -> Py { ensure_init(); A_BROKEN_HEART.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_BROKEN_HEART(v: Py) { ensure_init(); if let Ok(mut w) = A_BROKEN_HEART.write() { *w = v; } }
pub fn g_A_COMPARISON() -> Py { ensure_init(); A_COMPARISON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_COMPARISON(v: Py) { ensure_init(); if let Ok(mut w) = A_COMPARISON.write() { *w = v; } }
pub fn g_A_DATE_FIGURE() -> Py { ensure_init(); A_DATE_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DATE_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = A_DATE_FIGURE.write() { *w = v; } }
pub fn g_A_FRAME_IN_FRONT() -> Py { ensure_init(); A_FRAME_IN_FRONT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FRAME_IN_FRONT(v: Py) { ensure_init(); if let Ok(mut w) = A_FRAME_IN_FRONT.write() { *w = v; } }
pub fn g_A_HAVING() -> Py { ensure_init(); A_HAVING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HAVING(v: Py) { ensure_init(); if let Ok(mut w) = A_HAVING.write() { *w = v; } }
pub fn g_A_HOW_MANY_OF_THE_BOTS_OWN() -> Py { ensure_init(); A_HOW_MANY_OF_THE_BOTS_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HOW_MANY_OF_THE_BOTS_OWN(v: Py) { ensure_init(); if let Ok(mut w) = A_HOW_MANY_OF_THE_BOTS_OWN.write() { *w = v; } }
pub fn g_A_LIMIT_ABOUT_PEOPLE() -> Py { ensure_init(); A_LIMIT_ABOUT_PEOPLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_LIMIT_ABOUT_PEOPLE(v: Py) { ensure_init(); if let Ok(mut w) = A_LIMIT_ABOUT_PEOPLE.write() { *w = v; } }
pub fn g_A_MONEY_SUBJECT() -> Py { ensure_init(); A_MONEY_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_MONEY_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = A_MONEY_SUBJECT.write() { *w = v; } }
pub fn g_A_MONTH_PERIOD() -> Py { ensure_init(); A_MONTH_PERIOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_MONTH_PERIOD(v: Py) { ensure_init(); if let Ok(mut w) = A_MONTH_PERIOD.write() { *w = v; } }
pub fn g_A_NOT_A_SUGGESTION() -> Py { ensure_init(); A_NOT_A_SUGGESTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_NOT_A_SUGGESTION(v: Py) { ensure_init(); if let Ok(mut w) = A_NOT_A_SUGGESTION.write() { *w = v; } }
pub fn g_A_PERIOD() -> Py { ensure_init(); A_PERIOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PERIOD(v: Py) { ensure_init(); if let Ok(mut w) = A_PERIOD.write() { *w = v; } }
pub fn g_A_PERSON_AFTER() -> Py { ensure_init(); A_PERSON_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = A_PERSON_AFTER.write() { *w = v; } }
pub fn g_A_PERSON_AFTER_A_POSSESSIVE() -> Py { ensure_init(); A_PERSON_AFTER_A_POSSESSIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_AFTER_A_POSSESSIVE(v: Py) { ensure_init(); if let Ok(mut w) = A_PERSON_AFTER_A_POSSESSIVE.write() { *w = v; } }
pub fn g_A_PERSON_ELSE() -> Py { ensure_init(); A_PERSON_ELSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_ELSE(v: Py) { ensure_init(); if let Ok(mut w) = A_PERSON_ELSE.write() { *w = v; } }
pub fn g_A_PERSON_IS_THE_SUBJECT() -> Py { ensure_init(); A_PERSON_IS_THE_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_IS_THE_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = A_PERSON_IS_THE_SUBJECT.write() { *w = v; } }
pub fn g_A_PERSON_NAMED() -> Py { ensure_init(); A_PERSON_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = A_PERSON_NAMED.write() { *w = v; } }
pub fn g_A_POSSESSED_HEART() -> Py { ensure_init(); A_POSSESSED_HEART.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_POSSESSED_HEART(v: Py) { ensure_init(); if let Ok(mut w) = A_POSSESSED_HEART.write() { *w = v; } }
pub fn g_A_REPORTED_VERB() -> Py { ensure_init(); A_REPORTED_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_REPORTED_VERB(v: Py) { ensure_init(); if let Ok(mut w) = A_REPORTED_VERB.write() { *w = v; } }
pub fn g_A_SAID_BY() -> Py { ensure_init(); A_SAID_BY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SAID_BY(v: Py) { ensure_init(); if let Ok(mut w) = A_SAID_BY.write() { *w = v; } }
pub fn g_A_SAID_BY_ANYONE() -> Py { ensure_init(); A_SAID_BY_ANYONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SAID_BY_ANYONE(v: Py) { ensure_init(); if let Ok(mut w) = A_SAID_BY_ANYONE.write() { *w = v; } }
pub fn g_A_SUGGESTED_CLAUSE() -> Py { ensure_init(); A_SUGGESTED_CLAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SUGGESTED_CLAUSE(v: Py) { ensure_init(); if let Ok(mut w) = A_SUGGESTED_CLAUSE.write() { *w = v; } }
pub fn g_A_THING_NOT_A_CLAIM() -> Py { ensure_init(); A_THING_NOT_A_CLAIM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_THING_NOT_A_CLAIM(v: Py) { ensure_init(); if let Ok(mut w) = A_THING_NOT_A_CLAIM.write() { *w = v; } }
pub fn g_A_TOLD_TO() -> Py { ensure_init(); A_TOLD_TO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_TOLD_TO(v: Py) { ensure_init(); if let Ok(mut w) = A_TOLD_TO.write() { *w = v; } }
pub fn g_A_WHAT_ITS_LIKE() -> Py { ensure_init(); A_WHAT_ITS_LIKE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_WHAT_ITS_LIKE(v: Py) { ensure_init(); if let Ok(mut w) = A_WHAT_ITS_LIKE.write() { *w = v; } }
pub fn g_A_YOUR_KIN() -> Py { ensure_init(); A_YOUR_KIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_YOUR_KIN(v: Py) { ensure_init(); if let Ok(mut w) = A_YOUR_KIN.write() { *w = v; } }
pub fn g_A_YOUR_OWN_THING() -> Py { ensure_init(); A_YOUR_OWN_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_YOUR_OWN_THING(v: Py) { ensure_init(); if let Ok(mut w) = A_YOUR_OWN_THING.write() { *w = v; } }
pub fn g_BACK_AFTER() -> Py { ensure_init(); BACK_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BACK_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = BACK_AFTER.write() { *w = v; } }
pub fn g_BACK_ASKING() -> Py { ensure_init(); BACK_ASKING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BACK_ASKING(v: Py) { ensure_init(); if let Ok(mut w) = BACK_ASKING.write() { *w = v; } }
pub fn g_BACK_AT() -> Py { ensure_init(); BACK_AT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BACK_AT(v: Py) { ensure_init(); if let Ok(mut w) = BACK_AT.write() { *w = v; } }
pub fn g_BACK_INTO() -> Py { ensure_init(); BACK_INTO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BACK_INTO(v: Py) { ensure_init(); if let Ok(mut w) = BACK_INTO.write() { *w = v; } }
pub fn g_BACK_JOIN() -> Py { ensure_init(); BACK_JOIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BACK_JOIN(v: Py) { ensure_init(); if let Ok(mut w) = BACK_JOIN.write() { *w = v; } }
pub fn g_BACK_NOT_A_SUBJECT() -> Py { ensure_init(); BACK_NOT_A_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BACK_NOT_A_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = BACK_NOT_A_SUBJECT.write() { *w = v; } }
pub fn g_BARE_DEFINITION() -> Py { ensure_init(); BARE_DEFINITION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_DEFINITION(v: Py) { ensure_init(); if let Ok(mut w) = BARE_DEFINITION.write() { *w = v; } }
pub fn g_BARE_VERBS() -> Py { ensure_init(); BARE_VERBS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_VERBS(v: Py) { ensure_init(); if let Ok(mut w) = BARE_VERBS.write() { *w = v; } }
pub fn g_BEEN_SAID() -> Py { ensure_init(); BEEN_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BEEN_SAID(v: Py) { ensure_init(); if let Ok(mut w) = BEEN_SAID.write() { *w = v; } }
pub fn g_BEEN_WHEN() -> Py { ensure_init(); BEEN_WHEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BEEN_WHEN(v: Py) { ensure_init(); if let Ok(mut w) = BEEN_WHEN.write() { *w = v; } }
pub fn g_BEFORE_COURTESY() -> Py { ensure_init(); BEFORE_COURTESY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BEFORE_COURTESY(v: Py) { ensure_init(); if let Ok(mut w) = BEFORE_COURTESY.write() { *w = v; } }
pub fn g_BEFORE_HELP_WAS_GOOD() -> Py { ensure_init(); BEFORE_HELP_WAS_GOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BEFORE_HELP_WAS_GOOD(v: Py) { ensure_init(); if let Ok(mut w) = BEFORE_HELP_WAS_GOOD.write() { *w = v; } }
pub fn g_BEFORE_STATE() -> Py { ensure_init(); BEFORE_STATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BEFORE_STATE(v: Py) { ensure_init(); if let Ok(mut w) = BEFORE_STATE.write() { *w = v; } }
pub fn g_BETTER() -> Py { ensure_init(); BETTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BETTER(v: Py) { ensure_init(); if let Ok(mut w) = BETTER.write() { *w = v; } }
pub fn g_BETTER_OFF() -> Py { ensure_init(); BETTER_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BETTER_OFF(v: Py) { ensure_init(); if let Ok(mut w) = BETTER_OFF.write() { *w = v; } }
pub fn g_BETWEEN_STATE_AND_NOUN() -> Py { ensure_init(); BETWEEN_STATE_AND_NOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BETWEEN_STATE_AND_NOUN(v: Py) { ensure_init(); if let Ok(mut w) = BETWEEN_STATE_AND_NOUN.write() { *w = v; } }
pub fn g_BE_FORMS() -> Py { ensure_init(); BE_FORMS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BE_FORMS(v: Py) { ensure_init(); if let Ok(mut w) = BE_FORMS.write() { *w = v; } }
pub fn g_BODY_STATE_CUES() -> Py { ensure_init(); BODY_STATE_CUES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BODY_STATE_CUES(v: Py) { ensure_init(); if let Ok(mut w) = BODY_STATE_CUES.write() { *w = v; } }
pub fn g_BODY_STATE_WORDS() -> Py { ensure_init(); BODY_STATE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BODY_STATE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = BODY_STATE_WORDS.write() { *w = v; } }
pub fn g_BODY_SUBJECT() -> Py { ensure_init(); BODY_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BODY_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = BODY_SUBJECT.write() { *w = v; } }
pub fn g_BOT_CHATS() -> Py { ensure_init(); BOT_CHATS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BOT_CHATS(v: Py) { ensure_init(); if let Ok(mut w) = BOT_CHATS.write() { *w = v; } }
pub fn g_BOT_COUNTED() -> Py { ensure_init(); BOT_COUNTED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BOT_COUNTED(v: Py) { ensure_init(); if let Ok(mut w) = BOT_COUNTED.write() { *w = v; } }
pub fn g_BOT_DAY_SHAPES() -> Py { ensure_init(); BOT_DAY_SHAPES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BOT_DAY_SHAPES(v: Py) { ensure_init(); if let Ok(mut w) = BOT_DAY_SHAPES.write() { *w = v; } }
pub fn g_BOT_DAY_WORDS() -> Py { ensure_init(); BOT_DAY_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BOT_DAY_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = BOT_DAY_WORDS.write() { *w = v; } }
pub fn g_BOT_DOERS() -> Py { ensure_init(); BOT_DOERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BOT_DOERS(v: Py) { ensure_init(); if let Ok(mut w) = BOT_DOERS.write() { *w = v; } }
pub fn g_BOT_DOINGS() -> Py { ensure_init(); BOT_DOINGS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BOT_DOINGS(v: Py) { ensure_init(); if let Ok(mut w) = BOT_DOINGS.write() { *w = v; } }
pub fn g_BOT_HAVE() -> Py { ensure_init(); BOT_HAVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BOT_HAVE(v: Py) { ensure_init(); if let Ok(mut w) = BOT_HAVE.write() { *w = v; } }
pub fn g_BOT_LIFE() -> Py { ensure_init(); BOT_LIFE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BOT_LIFE(v: Py) { ensure_init(); if let Ok(mut w) = BOT_LIFE.write() { *w = v; } }
pub fn g_BOT_LIFE_ASK() -> Py { ensure_init(); BOT_LIFE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BOT_LIFE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = BOT_LIFE_ASK.write() { *w = v; } }
pub fn g_BOT_LIFE_FILLER() -> Py { ensure_init(); BOT_LIFE_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BOT_LIFE_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = BOT_LIFE_FILLER.write() { *w = v; } }
pub fn g_BOUT() -> Py { ensure_init(); BOUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BOUT(v: Py) { ensure_init(); if let Ok(mut w) = BOUT.write() { *w = v; } }
pub fn g_BROKEN_HEARTED() -> Py { ensure_init(); BROKEN_HEARTED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BROKEN_HEARTED(v: Py) { ensure_init(); if let Ok(mut w) = BROKEN_HEARTED.write() { *w = v; } }
pub fn g_BROKE_A_HEART() -> Py { ensure_init(); BROKE_A_HEART.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BROKE_A_HEART(v: Py) { ensure_init(); if let Ok(mut w) = BROKE_A_HEART.write() { *w = v; } }
pub fn g_CAPABILITY() -> Py { ensure_init(); CAPABILITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CAPABILITY(v: Py) { ensure_init(); if let Ok(mut w) = CAPABILITY.write() { *w = v; } }
pub fn g_CAPABILITY_CARRIER() -> Py { ensure_init(); CAPABILITY_CARRIER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CAPABILITY_CARRIER(v: Py) { ensure_init(); if let Ok(mut w) = CAPABILITY_CARRIER.write() { *w = v; } }
pub fn g_CAPABILITY_FILLER() -> Py { ensure_init(); CAPABILITY_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CAPABILITY_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = CAPABILITY_FILLER.write() { *w = v; } }
pub fn g_CARE_HOME_BRING_ASKS() -> Py { ensure_init(); CARE_HOME_BRING_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARE_HOME_BRING_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = CARE_HOME_BRING_ASKS.write() { *w = v; } }
pub fn g_CARE_HOME_BRING_WHO() -> Py { ensure_init(); CARE_HOME_BRING_WHO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARE_HOME_BRING_WHO(v: Py) { ensure_init(); if let Ok(mut w) = CARE_HOME_BRING_WHO.write() { *w = v; } }
pub fn g_CARE_HOME_BRING_WORD() -> Py { ensure_init(); CARE_HOME_BRING_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARE_HOME_BRING_WORD(v: Py) { ensure_init(); if let Ok(mut w) = CARE_HOME_BRING_WORD.write() { *w = v; } }
pub fn g_CARE_HOME_GUILT() -> Py { ensure_init(); CARE_HOME_GUILT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARE_HOME_GUILT(v: Py) { ensure_init(); if let Ok(mut w) = CARE_HOME_GUILT.write() { *w = v; } }
pub fn g_CARE_HOME_KIN() -> Py { ensure_init(); CARE_HOME_KIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARE_HOME_KIN(v: Py) { ensure_init(); if let Ok(mut w) = CARE_HOME_KIN.write() { *w = v; } }
pub fn g_CARE_HOME_MOVED() -> Py { ensure_init(); CARE_HOME_MOVED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARE_HOME_MOVED(v: Py) { ensure_init(); if let Ok(mut w) = CARE_HOME_MOVED.write() { *w = v; } }
pub fn g_CARE_HOME_NOT() -> Py { ensure_init(); CARE_HOME_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARE_HOME_NOT(v: Py) { ensure_init(); if let Ok(mut w) = CARE_HOME_NOT.write() { *w = v; } }
pub fn g_CARE_HOME_PLACE() -> Py { ensure_init(); CARE_HOME_PLACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARE_HOME_PLACE(v: Py) { ensure_init(); if let Ok(mut w) = CARE_HOME_PLACE.write() { *w = v; } }
pub fn g_CARE_HOME_VISIT() -> Py { ensure_init(); CARE_HOME_VISIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARE_HOME_VISIT(v: Py) { ensure_init(); if let Ok(mut w) = CARE_HOME_VISIT.write() { *w = v; } }
pub fn g_CHILD_NOUNS() -> Py { ensure_init(); CHILD_NOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHILD_NOUNS(v: Py) { ensure_init(); if let Ok(mut w) = CHILD_NOUNS.write() { *w = v; } }
pub fn g_CHOICE() -> Py { ensure_init(); CHOICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHOICE(v: Py) { ensure_init(); if let Ok(mut w) = CHOICE.write() { *w = v; } }
pub fn g_CLAIM_ENDS() -> Py { ensure_init(); CLAIM_ENDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLAIM_ENDS(v: Py) { ensure_init(); if let Ok(mut w) = CLAIM_ENDS.write() { *w = v; } }
pub fn g_CLAIM_STEMS() -> Py { ensure_init(); CLAIM_STEMS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLAIM_STEMS(v: Py) { ensure_init(); if let Ok(mut w) = CLAIM_STEMS.write() { *w = v; } }
pub fn g_CLAUSE_HELPERS() -> Py { ensure_init(); CLAUSE_HELPERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_HELPERS(v: Py) { ensure_init(); if let Ok(mut w) = CLAUSE_HELPERS.write() { *w = v; } }
pub fn g_CLAUSE_OPENERS() -> Py { ensure_init(); CLAUSE_OPENERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_OPENERS(v: Py) { ensure_init(); if let Ok(mut w) = CLAUSE_OPENERS.write() { *w = v; } }
pub fn g_CLAUSE_VERBS() -> Py { ensure_init(); CLAUSE_VERBS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_VERBS(v: Py) { ensure_init(); if let Ok(mut w) = CLAUSE_VERBS.write() { *w = v; } }
pub fn g_COMMIT_ALONE() -> Py { ensure_init(); COMMIT_ALONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMMIT_ALONE(v: Py) { ensure_init(); if let Ok(mut w) = COMMIT_ALONE.write() { *w = v; } }
pub fn g_COMMIT_ALONE_TAIL() -> Py { ensure_init(); COMMIT_ALONE_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMMIT_ALONE_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = COMMIT_ALONE_TAIL.write() { *w = v; } }
pub fn g_COMMIT_NO_HARM() -> Py { ensure_init(); COMMIT_NO_HARM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMMIT_NO_HARM(v: Py) { ensure_init(); if let Ok(mut w) = COMMIT_NO_HARM.write() { *w = v; } }
pub fn g_COMMIT_OPEN() -> Py { ensure_init(); COMMIT_OPEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMMIT_OPEN(v: Py) { ensure_init(); if let Ok(mut w) = COMMIT_OPEN.write() { *w = v; } }
pub fn g_COMMIT_STOP() -> Py { ensure_init(); COMMIT_STOP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMMIT_STOP(v: Py) { ensure_init(); if let Ok(mut w) = COMMIT_STOP.write() { *w = v; } }
pub fn g_COMMIT_SUBJECT() -> Py { ensure_init(); COMMIT_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMMIT_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = COMMIT_SUBJECT.write() { *w = v; } }
pub fn g_COMMIT_TRY() -> Py { ensure_init(); COMMIT_TRY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMMIT_TRY(v: Py) { ensure_init(); if let Ok(mut w) = COMMIT_TRY.write() { *w = v; } }
pub fn g_COMPARATIVE() -> Py { ensure_init(); COMPARATIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMPARATIVE(v: Py) { ensure_init(); if let Ok(mut w) = COMPARATIVE.write() { *w = v; } }
pub fn g_COMPARE() -> Py { ensure_init(); COMPARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMPARE(v: Py) { ensure_init(); if let Ok(mut w) = COMPARE.write() { *w = v; } }
pub fn g_COMPARISON() -> Py { ensure_init(); COMPARISON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMPARISON(v: Py) { ensure_init(); if let Ok(mut w) = COMPARISON.write() { *w = v; } }
pub fn g_CONFUSED_SELF_ONLY() -> Py { ensure_init(); CONFUSED_SELF_ONLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONFUSED_SELF_ONLY(v: Py) { ensure_init(); if let Ok(mut w) = CONFUSED_SELF_ONLY.write() { *w = v; } }
pub fn g_CONJUNCTIONS() -> Py { ensure_init(); CONJUNCTIONS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONJUNCTIONS(v: Py) { ensure_init(); if let Ok(mut w) = CONJUNCTIONS.write() { *w = v; } }
pub fn g_COPULA() -> Py { ensure_init(); COPULA.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COPULA(v: Py) { ensure_init(); if let Ok(mut w) = COPULA.write() { *w = v; } }
pub fn g_COPULAS() -> Py { ensure_init(); COPULAS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COPULAS(v: Py) { ensure_init(); if let Ok(mut w) = COPULAS.write() { *w = v; } }
pub fn g_COPULA_BEFORE() -> Py { ensure_init(); COPULA_BEFORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COPULA_BEFORE(v: Py) { ensure_init(); if let Ok(mut w) = COPULA_BEFORE.write() { *w = v; } }
pub fn g_COPULA_STATES() -> Py { ensure_init(); COPULA_STATES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COPULA_STATES(v: Py) { ensure_init(); if let Ok(mut w) = COPULA_STATES.write() { *w = v; } }
pub fn g_CORRECTION() -> Py { ensure_init(); CORRECTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CORRECTION(v: Py) { ensure_init(); if let Ok(mut w) = CORRECTION.write() { *w = v; } }
pub fn g_COUNT_FILLERS() -> Py { ensure_init(); COUNT_FILLERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COUNT_FILLERS(v: Py) { ensure_init(); if let Ok(mut w) = COUNT_FILLERS.write() { *w = v; } }
pub fn g_COUNT_OPENERS() -> Py { ensure_init(); COUNT_OPENERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COUNT_OPENERS(v: Py) { ensure_init(); if let Ok(mut w) = COUNT_OPENERS.write() { *w = v; } }
pub fn g_COURTESY_THING() -> Py { ensure_init(); COURTESY_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COURTESY_THING(v: Py) { ensure_init(); if let Ok(mut w) = COURTESY_THING.write() { *w = v; } }
pub fn g_COURTESY_WORD() -> Py { ensure_init(); COURTESY_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COURTESY_WORD(v: Py) { ensure_init(); if let Ok(mut w) = COURTESY_WORD.write() { *w = v; } }
pub fn g_CUE_FOLLOW() -> Py { ensure_init(); CUE_FOLLOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CUE_FOLLOW(v: Py) { ensure_init(); if let Ok(mut w) = CUE_FOLLOW.write() { *w = v; } }
pub fn g_DEFINITION() -> Py { ensure_init(); DEFINITION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEFINITION(v: Py) { ensure_init(); if let Ok(mut w) = DEFINITION.write() { *w = v; } }
pub fn g_DEGREE() -> Py { ensure_init(); DEGREE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEGREE(v: Py) { ensure_init(); if let Ok(mut w) = DEGREE.write() { *w = v; } }
pub fn g_DENY() -> Py { ensure_init(); DENY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DENY(v: Py) { ensure_init(); if let Ok(mut w) = DENY.write() { *w = v; } }
pub fn g_DENY_BEFORE() -> Py { ensure_init(); DENY_BEFORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DENY_BEFORE(v: Py) { ensure_init(); if let Ok(mut w) = DENY_BEFORE.write() { *w = v; } }
pub fn g_DENY_BETWEEN() -> Py { ensure_init(); DENY_BETWEEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DENY_BETWEEN(v: Py) { ensure_init(); if let Ok(mut w) = DENY_BETWEEN.write() { *w = v; } }
pub fn g_DESCRIBES() -> Py { ensure_init(); DESCRIBES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DESCRIBES(v: Py) { ensure_init(); if let Ok(mut w) = DESCRIBES.write() { *w = v; } }
pub fn g_DETERMINERS() -> Py { ensure_init(); DETERMINERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DETERMINERS(v: Py) { ensure_init(); if let Ok(mut w) = DETERMINERS.write() { *w = v; } }
pub fn g_DOING_FILLER() -> Py { ensure_init(); DOING_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOING_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = DOING_FILLER.write() { *w = v; } }
pub fn g_ECHO_CARRIERS() -> Py { ensure_init(); ECHO_CARRIERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ECHO_CARRIERS(v: Py) { ensure_init(); if let Ok(mut w) = ECHO_CARRIERS.write() { *w = v; } }
pub fn g_EMOTION_WORDS() -> Py { ensure_init(); EMOTION_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EMOTION_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = EMOTION_WORDS.write() { *w = v; } }
pub fn g_EPISODE_NOUNS() -> Py { ensure_init(); EPISODE_NOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EPISODE_NOUNS(v: Py) { ensure_init(); if let Ok(mut w) = EPISODE_NOUNS.write() { *w = v; } }
pub fn g_EVENT_NOUNS() -> Py { ensure_init(); EVENT_NOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EVENT_NOUNS(v: Py) { ensure_init(); if let Ok(mut w) = EVENT_NOUNS.write() { *w = v; } }
pub fn g_EVENT_WORDS() -> Py { ensure_init(); EVENT_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EVENT_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = EVENT_WORDS.write() { *w = v; } }
pub fn g_EXPERIENCE_ASK() -> Py { ensure_init(); EXPERIENCE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EXPERIENCE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = EXPERIENCE_ASK.write() { *w = v; } }
pub fn g_EXPERIENCE_WORDS() -> Py { ensure_init(); EXPERIENCE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EXPERIENCE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = EXPERIENCE_WORDS.write() { *w = v; } }
pub fn g_FALL() -> Py { ensure_init(); FALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FALL(v: Py) { ensure_init(); if let Ok(mut w) = FALL.write() { *w = v; } }
pub fn g_FAREWELLS() -> Py { ensure_init(); FAREWELLS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FAREWELLS(v: Py) { ensure_init(); if let Ok(mut w) = FAREWELLS.write() { *w = v; } }
pub fn g_FAREWELL_TAIL() -> Py { ensure_init(); FAREWELL_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FAREWELL_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = FAREWELL_TAIL.write() { *w = v; } }
pub fn g_FEELING_COPULA() -> Py { ensure_init(); FEELING_COPULA.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_COPULA(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_COPULA.write() { *w = v; } }
pub fn g_FEELING_CUES_BEFORE() -> Py { ensure_init(); FEELING_CUES_BEFORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_CUES_BEFORE(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_CUES_BEFORE.write() { *w = v; } }
pub fn g_FEELING_FILLER() -> Py { ensure_init(); FEELING_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_FILLER.write() { *w = v; } }
pub fn g_FEELING_NOT_MOOD() -> Py { ensure_init(); FEELING_NOT_MOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_NOT_MOOD(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_NOT_MOOD.write() { *w = v; } }
pub fn g_FEELING_SOUND_FRAME() -> Py { ensure_init(); FEELING_SOUND_FRAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_SOUND_FRAME(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_SOUND_FRAME.write() { *w = v; } }
pub fn g_FELL_WORDS() -> Py { ensure_init(); FELL_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FELL_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = FELL_WORDS.write() { *w = v; } }
pub fn g_FILLER_AFTER() -> Py { ensure_init(); FILLER_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FILLER_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = FILLER_AFTER.write() { *w = v; } }
pub fn g_FILLER_BEFORE() -> Py { ensure_init(); FILLER_BEFORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FILLER_BEFORE(v: Py) { ensure_init(); if let Ok(mut w) = FILLER_BEFORE.write() { *w = v; } }
pub fn g_FIRST_PERSON() -> Py { ensure_init(); FIRST_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIRST_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = FIRST_PERSON.write() { *w = v; } }
pub fn g_FOUND() -> Py { ensure_init(); FOUND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOUND(v: Py) { ensure_init(); if let Ok(mut w) = FOUND.write() { *w = v; } }
pub fn g_FRAME_NOT_A_SETUP() -> Py { ensure_init(); FRAME_NOT_A_SETUP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRAME_NOT_A_SETUP(v: Py) { ensure_init(); if let Ok(mut w) = FRAME_NOT_A_SETUP.write() { *w = v; } }
pub fn g_FREE_OR_BUSY() -> Py { ensure_init(); FREE_OR_BUSY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FREE_OR_BUSY(v: Py) { ensure_init(); if let Ok(mut w) = FREE_OR_BUSY.write() { *w = v; } }
pub fn g_GENDER_WORDS() -> Py { ensure_init(); GENDER_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GENDER_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = GENDER_WORDS.write() { *w = v; } }
pub fn g_GETTING_WORSE() -> Py { ensure_init(); GETTING_WORSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GETTING_WORSE(v: Py) { ensure_init(); if let Ok(mut w) = GETTING_WORSE.write() { *w = v; } }
pub fn g_GIVEN_ADVICE() -> Py { ensure_init(); GIVEN_ADVICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GIVEN_ADVICE(v: Py) { ensure_init(); if let Ok(mut w) = GIVEN_ADVICE.write() { *w = v; } }
pub fn g_GOING_ON_WHEN() -> Py { ensure_init(); GOING_ON_WHEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GOING_ON_WHEN(v: Py) { ensure_init(); if let Ok(mut w) = GOING_ON_WHEN.write() { *w = v; } }
pub fn g_GOOD_EMOTION_NAMES() -> Py { ensure_init(); GOOD_EMOTION_NAMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GOOD_EMOTION_NAMES(v: Py) { ensure_init(); if let Ok(mut w) = GOOD_EMOTION_NAMES.write() { *w = v; } }
pub fn g_GOOD_WORD_BY_COMPARISON() -> Py { ensure_init(); GOOD_WORD_BY_COMPARISON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GOOD_WORD_BY_COMPARISON(v: Py) { ensure_init(); if let Ok(mut w) = GOOD_WORD_BY_COMPARISON.write() { *w = v; } }
pub fn g_GOOD_WORD_COMPARED() -> Py { ensure_init(); GOOD_WORD_COMPARED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GOOD_WORD_COMPARED(v: Py) { ensure_init(); if let Ok(mut w) = GOOD_WORD_COMPARED.write() { *w = v; } }
pub fn g_GOOD_WORD_MISSING() -> Py { ensure_init(); GOOD_WORD_MISSING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GOOD_WORD_MISSING(v: Py) { ensure_init(); if let Ok(mut w) = GOOD_WORD_MISSING.write() { *w = v; } }
pub fn g_GRATITUDE_ASK_AGAIN() -> Py { ensure_init(); GRATITUDE_ASK_AGAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GRATITUDE_ASK_AGAIN(v: Py) { ensure_init(); if let Ok(mut w) = GRATITUDE_ASK_AGAIN.write() { *w = v; } }
pub fn g_GRATITUDE_JOINERS() -> Py { ensure_init(); GRATITUDE_JOINERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GRATITUDE_JOINERS(v: Py) { ensure_init(); if let Ok(mut w) = GRATITUDE_JOINERS.write() { *w = v; } }
pub fn g_GRATITUDE_OBJECT() -> Py { ensure_init(); GRATITUDE_OBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GRATITUDE_OBJECT(v: Py) { ensure_init(); if let Ok(mut w) = GRATITUDE_OBJECT.write() { *w = v; } }
pub fn g_GREETINGS() -> Py { ensure_init(); GREETINGS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GREETINGS(v: Py) { ensure_init(); if let Ok(mut w) = GREETINGS.write() { *w = v; } }
pub fn g_GREETING_ASK_ABOUT_SOMETHING() -> Py { ensure_init(); GREETING_ASK_ABOUT_SOMETHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GREETING_ASK_ABOUT_SOMETHING(v: Py) { ensure_init(); if let Ok(mut w) = GREETING_ASK_ABOUT_SOMETHING.write() { *w = v; } }
pub fn g_GREETING_ASK_SECOND_PERSON() -> Py { ensure_init(); GREETING_ASK_SECOND_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GREETING_ASK_SECOND_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = GREETING_ASK_SECOND_PERSON.write() { *w = v; } }
pub fn g_GREETING_ASK_STATE() -> Py { ensure_init(); GREETING_ASK_STATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GREETING_ASK_STATE(v: Py) { ensure_init(); if let Ok(mut w) = GREETING_ASK_STATE.write() { *w = v; } }
pub fn g_GREETING_ASK_SUBJECT() -> Py { ensure_init(); GREETING_ASK_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GREETING_ASK_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = GREETING_ASK_SUBJECT.write() { *w = v; } }
pub fn g_GREETING_ASK_WORDS() -> Py { ensure_init(); GREETING_ASK_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GREETING_ASK_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = GREETING_ASK_WORDS.write() { *w = v; } }
pub fn g_GREETING_TAIL() -> Py { ensure_init(); GREETING_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GREETING_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = GREETING_TAIL.write() { *w = v; } }
pub fn g_GREETING_WHAT_SAYING() -> Py { ensure_init(); GREETING_WHAT_SAYING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GREETING_WHAT_SAYING(v: Py) { ensure_init(); if let Ok(mut w) = GREETING_WHAT_SAYING.write() { *w = v; } }
pub fn g_HAND_OVER_LEADS() -> Py { ensure_init(); HAND_OVER_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HAND_OVER_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = HAND_OVER_LEADS.write() { *w = v; } }
pub fn g_HAND_OVER_MINE() -> Py { ensure_init(); HAND_OVER_MINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HAND_OVER_MINE(v: Py) { ensure_init(); if let Ok(mut w) = HAND_OVER_MINE.write() { *w = v; } }
pub fn g_HAND_OVER_MODALS() -> Py { ensure_init(); HAND_OVER_MODALS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HAND_OVER_MODALS(v: Py) { ensure_init(); if let Ok(mut w) = HAND_OVER_MODALS.write() { *w = v; } }
pub fn g_HAND_OVER_SUBJECTS() -> Py { ensure_init(); HAND_OVER_SUBJECTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HAND_OVER_SUBJECTS(v: Py) { ensure_init(); if let Ok(mut w) = HAND_OVER_SUBJECTS.write() { *w = v; } }
pub fn g_HAND_OVER_TO() -> Py { ensure_init(); HAND_OVER_TO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HAND_OVER_TO(v: Py) { ensure_init(); if let Ok(mut w) = HAND_OVER_TO.write() { *w = v; } }
pub fn g_HAND_OVER_VERBS() -> Py { ensure_init(); HAND_OVER_VERBS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HAND_OVER_VERBS(v: Py) { ensure_init(); if let Ok(mut w) = HAND_OVER_VERBS.write() { *w = v; } }
pub fn g_HAVE_A_BROKEN_HEART() -> Py { ensure_init(); HAVE_A_BROKEN_HEART.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HAVE_A_BROKEN_HEART(v: Py) { ensure_init(); if let Ok(mut w) = HAVE_A_BROKEN_HEART.write() { *w = v; } }
pub fn g_HEART_DEGREES() -> Py { ensure_init(); HEART_DEGREES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEART_DEGREES(v: Py) { ensure_init(); if let Ok(mut w) = HEART_DEGREES.write() { *w = v; } }
pub fn g_HEART_IN_A_STATE() -> Py { ensure_init(); HEART_IN_A_STATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEART_IN_A_STATE(v: Py) { ensure_init(); if let Ok(mut w) = HEART_IN_A_STATE.write() { *w = v; } }
pub fn g_HEDGED_ALONE() -> Py { ensure_init(); HEDGED_ALONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEDGED_ALONE(v: Py) { ensure_init(); if let Ok(mut w) = HEDGED_ALONE.write() { *w = v; } }
pub fn g_HEDGED_COMMIT() -> Py { ensure_init(); HEDGED_COMMIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEDGED_COMMIT(v: Py) { ensure_init(); if let Ok(mut w) = HEDGED_COMMIT.write() { *w = v; } }
pub fn g_HEDGED_DOUBT() -> Py { ensure_init(); HEDGED_DOUBT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEDGED_DOUBT(v: Py) { ensure_init(); if let Ok(mut w) = HEDGED_DOUBT.write() { *w = v; } }
pub fn g_HEDGED_LOW() -> Py { ensure_init(); HEDGED_LOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEDGED_LOW(v: Py) { ensure_init(); if let Ok(mut w) = HEDGED_LOW.write() { *w = v; } }
pub fn g_HEDGED_OK() -> Py { ensure_init(); HEDGED_OK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEDGED_OK(v: Py) { ensure_init(); if let Ok(mut w) = HEDGED_OK.write() { *w = v; } }
pub fn g_HEDGED_WRAPPER() -> Py { ensure_init(); HEDGED_WRAPPER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEDGED_WRAPPER(v: Py) { ensure_init(); if let Ok(mut w) = HEDGED_WRAPPER.write() { *w = v; } }
pub fn g_HEDGE_LEAD_ONE() -> Py { ensure_init(); HEDGE_LEAD_ONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEDGE_LEAD_ONE(v: Py) { ensure_init(); if let Ok(mut w) = HEDGE_LEAD_ONE.write() { *w = v; } }
pub fn g_HEDGE_LEAD_TWO() -> Py { ensure_init(); HEDGE_LEAD_TWO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEDGE_LEAD_TWO(v: Py) { ensure_init(); if let Ok(mut w) = HEDGE_LEAD_TWO.write() { *w = v; } }
pub fn g_HELP_ASK() -> Py { ensure_init(); HELP_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELP_ASK(v: Py) { ensure_init(); if let Ok(mut w) = HELP_ASK.write() { *w = v; } }
pub fn g_HELP_ASK_NOT() -> Py { ensure_init(); HELP_ASK_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELP_ASK_NOT(v: Py) { ensure_init(); if let Ok(mut w) = HELP_ASK_NOT.write() { *w = v; } }
pub fn g_HELP_ASK_OWN_SUBJECT() -> Py { ensure_init(); HELP_ASK_OWN_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELP_ASK_OWN_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = HELP_ASK_OWN_SUBJECT.write() { *w = v; } }
pub fn g_HELP_ASK_SUBJECT() -> Py { ensure_init(); HELP_ASK_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELP_ASK_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = HELP_ASK_SUBJECT.write() { *w = v; } }
pub fn g_HELP_ASK_TAIL_CLAUSE() -> Py { ensure_init(); HELP_ASK_TAIL_CLAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELP_ASK_TAIL_CLAUSE(v: Py) { ensure_init(); if let Ok(mut w) = HELP_ASK_TAIL_CLAUSE.write() { *w = v; } }
pub fn g_HELP_WAS_GOOD() -> Py { ensure_init(); HELP_WAS_GOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELP_WAS_GOOD(v: Py) { ensure_init(); if let Ok(mut w) = HELP_WAS_GOOD.write() { *w = v; } }
pub fn g_IDIOM_STATES() -> Py { ensure_init(); IDIOM_STATES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IDIOM_STATES(v: Py) { ensure_init(); if let Ok(mut w) = IDIOM_STATES.write() { *w = v; } }
pub fn g_ING_TAKES_NO_OBJECT() -> Py { ensure_init(); ING_TAKES_NO_OBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ING_TAKES_NO_OBJECT(v: Py) { ensure_init(); if let Ok(mut w) = ING_TAKES_NO_OBJECT.write() { *w = v; } }
pub fn g_ING_THINGS() -> Py { ensure_init(); ING_THINGS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ING_THINGS(v: Py) { ensure_init(); if let Ok(mut w) = ING_THINGS.write() { *w = v; } }
pub fn g_INTEREST_HEAD() -> Py { ensure_init(); INTEREST_HEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_INTEREST_HEAD(v: Py) { ensure_init(); if let Ok(mut w) = INTEREST_HEAD.write() { *w = v; } }
pub fn g_INTEREST_NOT() -> Py { ensure_init(); INTEREST_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_INTEREST_NOT(v: Py) { ensure_init(); if let Ok(mut w) = INTEREST_NOT.write() { *w = v; } }
pub fn g_INTEREST_TAIL() -> Py { ensure_init(); INTEREST_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_INTEREST_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = INTEREST_TAIL.write() { *w = v; } }
pub fn g_INTRODUCES() -> Py { ensure_init(); INTRODUCES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_INTRODUCES(v: Py) { ensure_init(); if let Ok(mut w) = INTRODUCES.write() { *w = v; } }
pub fn g_ITSELF_LIFE() -> Py { ensure_init(); ITSELF_LIFE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ITSELF_LIFE(v: Py) { ensure_init(); if let Ok(mut w) = ITSELF_LIFE.write() { *w = v; } }
pub fn g_ITSELF_LONG() -> Py { ensure_init(); ITSELF_LONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ITSELF_LONG(v: Py) { ensure_init(); if let Ok(mut w) = ITSELF_LONG.write() { *w = v; } }
pub fn g_ITSELF_WHEN() -> Py { ensure_init(); ITSELF_WHEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ITSELF_WHEN(v: Py) { ensure_init(); if let Ok(mut w) = ITSELF_WHEN.write() { *w = v; } }
pub fn g_ITSELF_WHEN_TAIL() -> Py { ensure_init(); ITSELF_WHEN_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ITSELF_WHEN_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = ITSELF_WHEN_TAIL.write() { *w = v; } }
pub fn g_JOIN_OPENERS() -> Py { ensure_init(); JOIN_OPENERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_JOIN_OPENERS(v: Py) { ensure_init(); if let Ok(mut w) = JOIN_OPENERS.write() { *w = v; } }
pub fn g_KEEPING_ASK() -> Py { ensure_init(); KEEPING_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEPING_ASK(v: Py) { ensure_init(); if let Ok(mut w) = KEEPING_ASK.write() { *w = v; } }
pub fn g_KEEPS_BUSY_SHAPE() -> Py { ensure_init(); KEEPS_BUSY_SHAPE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEPS_BUSY_SHAPE(v: Py) { ensure_init(); if let Ok(mut w) = KEEPS_BUSY_SHAPE.write() { *w = v; } }
pub fn g_KIN() -> Py { ensure_init(); KIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KIN(v: Py) { ensure_init(); if let Ok(mut w) = KIN.write() { *w = v; } }
pub fn g_LABEL_ON_A_PERSON() -> Py { ensure_init(); LABEL_ON_A_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LABEL_ON_A_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = LABEL_ON_A_PERSON.write() { *w = v; } }
pub fn g_LAUGH() -> Py { ensure_init(); LAUGH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LAUGH(v: Py) { ensure_init(); if let Ok(mut w) = LAUGH.write() { *w = v; } }
pub fn g_LAUGHED_OFF_AFTER() -> Py { ensure_init(); LAUGHED_OFF_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LAUGHED_OFF_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = LAUGHED_OFF_AFTER.write() { *w = v; } }
pub fn g_LAUGH_AT_THE_END() -> Py { ensure_init(); LAUGH_AT_THE_END.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LAUGH_AT_THE_END(v: Py) { ensure_init(); if let Ok(mut w) = LAUGH_AT_THE_END.write() { *w = v; } }
pub fn g_LEARNED_ON() -> Py { ensure_init(); LEARNED_ON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEARNED_ON(v: Py) { ensure_init(); if let Ok(mut w) = LEARNED_ON.write() { *w = v; } }
pub fn g_LEARNING_WORDS() -> Py { ensure_init(); LEARNING_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEARNING_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = LEARNING_WORDS.write() { *w = v; } }
pub fn g_LETTER_ASKED_FOR() -> Py { ensure_init(); LETTER_ASKED_FOR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LETTER_ASKED_FOR(v: Py) { ensure_init(); if let Ok(mut w) = LETTER_ASKED_FOR.write() { *w = v; } }
pub fn g_LETTER_ITSELF() -> Py { ensure_init(); LETTER_ITSELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LETTER_ITSELF(v: Py) { ensure_init(); if let Ok(mut w) = LETTER_ITSELF.write() { *w = v; } }
pub fn g_LETTER_RECEIVED() -> Py { ensure_init(); LETTER_RECEIVED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LETTER_RECEIVED(v: Py) { ensure_init(); if let Ok(mut w) = LETTER_RECEIVED.write() { *w = v; } }
pub fn g_LETTER_SOMEONE_ELSES() -> Py { ensure_init(); LETTER_SOMEONE_ELSES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LETTER_SOMEONE_ELSES(v: Py) { ensure_init(); if let Ok(mut w) = LETTER_SOMEONE_ELSES.write() { *w = v; } }
pub fn g_LIMIT_A_FACT() -> Py { ensure_init(); LIMIT_A_FACT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIMIT_A_FACT(v: Py) { ensure_init(); if let Ok(mut w) = LIMIT_A_FACT.write() { *w = v; } }
pub fn g_LIMIT_NOT_A_START() -> Py { ensure_init(); LIMIT_NOT_A_START.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIMIT_NOT_A_START(v: Py) { ensure_init(); if let Ok(mut w) = LIMIT_NOT_A_START.write() { *w = v; } }
pub fn g_LIMIT_NOT_A_THING() -> Py { ensure_init(); LIMIT_NOT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIMIT_NOT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = LIMIT_NOT_A_THING.write() { *w = v; } }
pub fn g_LIMIT_REFUSAL() -> Py { ensure_init(); LIMIT_REFUSAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIMIT_REFUSAL(v: Py) { ensure_init(); if let Ok(mut w) = LIMIT_REFUSAL.write() { *w = v; } }
pub fn g_LIMIT_ROOM() -> Py { ensure_init(); LIMIT_ROOM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIMIT_ROOM(v: Py) { ensure_init(); if let Ok(mut w) = LIMIT_ROOM.write() { *w = v; } }
pub fn g_LIMIT_SHAPES() -> Py { ensure_init(); LIMIT_SHAPES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIMIT_SHAPES(v: Py) { ensure_init(); if let Ok(mut w) = LIMIT_SHAPES.write() { *w = v; } }
pub fn g_LIMIT_SOFT_AFTER() -> Py { ensure_init(); LIMIT_SOFT_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIMIT_SOFT_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = LIMIT_SOFT_AFTER.write() { *w = v; } }
pub fn g_LIMIT_SOFT_BEFORE() -> Py { ensure_init(); LIMIT_SOFT_BEFORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIMIT_SOFT_BEFORE(v: Py) { ensure_init(); if let Ok(mut w) = LIMIT_SOFT_BEFORE.write() { *w = v; } }
pub fn g_MAKER_WORDS() -> Py { ensure_init(); MAKER_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MAKER_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = MAKER_WORDS.write() { *w = v; } }
pub fn g_MANIPULATE() -> Py { ensure_init(); MANIPULATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MANIPULATE(v: Py) { ensure_init(); if let Ok(mut w) = MANIPULATE.write() { *w = v; } }
pub fn g_MANNERS_OPENERS() -> Py { ensure_init(); MANNERS_OPENERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MANNERS_OPENERS(v: Py) { ensure_init(); if let Ok(mut w) = MANNERS_OPENERS.write() { *w = v; } }
pub fn g_MANNERS_WORDS() -> Py { ensure_init(); MANNERS_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MANNERS_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = MANNERS_WORDS.write() { *w = v; } }
pub fn g_MATH_WORDS() -> Py { ensure_init(); MATH_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MATH_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = MATH_WORDS.write() { *w = v; } }
pub fn g_MEANT_APPLIED_TO() -> Py { ensure_init(); MEANT_APPLIED_TO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEANT_APPLIED_TO(v: Py) { ensure_init(); if let Ok(mut w) = MEANT_APPLIED_TO.write() { *w = v; } }
pub fn g_ME_ASKS() -> Py { ensure_init(); ME_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ME_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = ME_ASKS.write() { *w = v; } }
pub fn g_ME_LEAD() -> Py { ensure_init(); ME_LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ME_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = ME_LEAD.write() { *w = v; } }
pub fn g_ME_TAIL() -> Py { ensure_init(); ME_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ME_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = ME_TAIL.write() { *w = v; } }
pub fn g_MOOD_NOUNS() -> Py { ensure_init(); MOOD_NOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOOD_NOUNS(v: Py) { ensure_init(); if let Ok(mut w) = MOOD_NOUNS.write() { *w = v; } }
pub fn g_MOOD_TAIL() -> Py { ensure_init(); MOOD_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOOD_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = MOOD_TAIL.write() { *w = v; } }
pub fn g_MOOD_TAIL_TAKEN() -> Py { ensure_init(); MOOD_TAIL_TAKEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOOD_TAIL_TAKEN(v: Py) { ensure_init(); if let Ok(mut w) = MOOD_TAIL_TAKEN.write() { *w = v; } }
pub fn g_MOOD_TAIL_WORDS() -> Py { ensure_init(); MOOD_TAIL_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOOD_TAIL_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = MOOD_TAIL_WORDS.write() { *w = v; } }
pub fn g_MOTION_WORDS() -> Py { ensure_init(); MOTION_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOTION_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = MOTION_WORDS.write() { *w = v; } }
pub fn g_NAMING_WORDS() -> Py { ensure_init(); NAMING_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMING_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = NAMING_WORDS.write() { *w = v; } }
pub fn g_NEEDS_ANSWER() -> Py { ensure_init(); NEEDS_ANSWER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEEDS_ANSWER(v: Py) { ensure_init(); if let Ok(mut w) = NEEDS_ANSWER.write() { *w = v; } }
pub fn g_NEGATIONS() -> Py { ensure_init(); NEGATIONS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEGATIONS(v: Py) { ensure_init(); if let Ok(mut w) = NEGATIONS.write() { *w = v; } }
pub fn g_NEGATION_SKIP() -> Py { ensure_init(); NEGATION_SKIP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEGATION_SKIP(v: Py) { ensure_init(); if let Ok(mut w) = NEGATION_SKIP.write() { *w = v; } }
pub fn g_NEGATIVE() -> Py { ensure_init(); NEGATIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEGATIVE(v: Py) { ensure_init(); if let Ok(mut w) = NEGATIVE.write() { *w = v; } }
pub fn g_NEGATORS() -> Py { ensure_init(); NEGATORS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEGATORS(v: Py) { ensure_init(); if let Ok(mut w) = NEGATORS.write() { *w = v; } }
pub fn g_NEWS() -> Py { ensure_init(); NEWS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEWS(v: Py) { ensure_init(); if let Ok(mut w) = NEWS.write() { *w = v; } }
pub fn g_NEW_ASK_AHEAD() -> Py { ensure_init(); NEW_ASK_AHEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEW_ASK_AHEAD(v: Py) { ensure_init(); if let Ok(mut w) = NEW_ASK_AHEAD.write() { *w = v; } }
pub fn g_NEW_ASK_WORDS() -> Py { ensure_init(); NEW_ASK_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEW_ASK_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = NEW_ASK_WORDS.write() { *w = v; } }
pub fn g_NEW_FRAME() -> Py { ensure_init(); NEW_FRAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEW_FRAME(v: Py) { ensure_init(); if let Ok(mut w) = NEW_FRAME.write() { *w = v; } }
pub fn g_NOT_AFTER_THE_BOT() -> Py { ensure_init(); NOT_AFTER_THE_BOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_AFTER_THE_BOT(v: Py) { ensure_init(); if let Ok(mut w) = NOT_AFTER_THE_BOT.write() { *w = v; } }
pub fn g_NOT_AUX() -> Py { ensure_init(); NOT_AUX.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_AUX(v: Py) { ensure_init(); if let Ok(mut w) = NOT_AUX.write() { *w = v; } }
pub fn g_NOT_A_DEFINITION_HEAD() -> Py { ensure_init(); NOT_A_DEFINITION_HEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DEFINITION_HEAD(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_DEFINITION_HEAD.write() { *w = v; } }
pub fn g_NOT_A_MOOD_THERE() -> Py { ensure_init(); NOT_A_MOOD_THERE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_MOOD_THERE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_MOOD_THERE.write() { *w = v; } }
pub fn g_NOT_A_SUBJECT() -> Py { ensure_init(); NOT_A_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_SUBJECT.write() { *w = v; } }
pub fn g_NOT_A_THING() -> Py { ensure_init(); NOT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_THING.write() { *w = v; } }
pub fn g_NOT_BEFORE_THE_BOT() -> Py { ensure_init(); NOT_BEFORE_THE_BOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_BEFORE_THE_BOT(v: Py) { ensure_init(); if let Ok(mut w) = NOT_BEFORE_THE_BOT.write() { *w = v; } }
pub fn g_NOT_FILLER() -> Py { ensure_init(); NOT_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = NOT_FILLER.write() { *w = v; } }
pub fn g_NOT_THEIR_OWN() -> Py { ensure_init(); NOT_THEIR_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_THEIR_OWN(v: Py) { ensure_init(); if let Ok(mut w) = NOT_THEIR_OWN.write() { *w = v; } }
pub fn g_NOT_THE_SPEAKERS_DAY() -> Py { ensure_init(); NOT_THE_SPEAKERS_DAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_SPEAKERS_DAY(v: Py) { ensure_init(); if let Ok(mut w) = NOT_THE_SPEAKERS_DAY.write() { *w = v; } }
pub fn g_NOUN_REQUEST_MAX() -> Py { ensure_init(); NOUN_REQUEST_MAX.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOUN_REQUEST_MAX(v: Py) { ensure_init(); if let Ok(mut w) = NOUN_REQUEST_MAX.write() { *w = v; } }
pub fn g_NOUN_REQUEST_SELF() -> Py { ensure_init(); NOUN_REQUEST_SELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOUN_REQUEST_SELF(v: Py) { ensure_init(); if let Ok(mut w) = NOUN_REQUEST_SELF.write() { *w = v; } }
pub fn g_NOUN_STATES() -> Py { ensure_init(); NOUN_STATES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOUN_STATES(v: Py) { ensure_init(); if let Ok(mut w) = NOUN_STATES.write() { *w = v; } }
pub fn g_NO_BETTER() -> Py { ensure_init(); NO_BETTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_BETTER(v: Py) { ensure_init(); if let Ok(mut w) = NO_BETTER.write() { *w = v; } }
pub fn g_NO_HARM_SUBJECT() -> Py { ensure_init(); NO_HARM_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_HARM_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = NO_HARM_SUBJECT.write() { *w = v; } }
pub fn g_NUMBER_WORDS() -> Py { ensure_init(); NUMBER_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NUMBER_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = NUMBER_WORDS.write() { *w = v; } }
pub fn g_OBJECT_MARKERS() -> Py { ensure_init(); OBJECT_MARKERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OBJECT_MARKERS(v: Py) { ensure_init(); if let Ok(mut w) = OBJECT_MARKERS.write() { *w = v; } }
pub fn g_OK_WORDS() -> Py { ensure_init(); OK_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OK_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = OK_WORDS.write() { *w = v; } }
pub fn g_OPENING_FRAME() -> Py { ensure_init(); OPENING_FRAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OPENING_FRAME(v: Py) { ensure_init(); if let Ok(mut w) = OPENING_FRAME.write() { *w = v; } }
pub fn g_OPENING_MAX() -> Py { ensure_init(); OPENING_MAX.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OPENING_MAX(v: Py) { ensure_init(); if let Ok(mut w) = OPENING_MAX.write() { *w = v; } }
pub fn g_OPENING_WORDS() -> Py { ensure_init(); OPENING_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OPENING_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = OPENING_WORDS.write() { *w = v; } }
pub fn g_OPENS_A_FRAME() -> Py { ensure_init(); OPENS_A_FRAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OPENS_A_FRAME(v: Py) { ensure_init(); if let Ok(mut w) = OPENS_A_FRAME.write() { *w = v; } }
pub fn g_OPINION() -> Py { ensure_init(); OPINION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OPINION(v: Py) { ensure_init(); if let Ok(mut w) = OPINION.write() { *w = v; } }
pub fn g_OPPOSITE_PAIRS() -> Py { ensure_init(); OPPOSITE_PAIRS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OPPOSITE_PAIRS(v: Py) { ensure_init(); if let Ok(mut w) = OPPOSITE_PAIRS.write() { *w = v; } }
pub fn g_OTHER_PEOPLE() -> Py { ensure_init(); OTHER_PEOPLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OTHER_PEOPLE(v: Py) { ensure_init(); if let Ok(mut w) = OTHER_PEOPLE.write() { *w = v; } }
pub fn g_OWNS_A_PERSON() -> Py { ensure_init(); OWNS_A_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWNS_A_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = OWNS_A_PERSON.write() { *w = v; } }
pub fn g_OWN_PERIOD_FILLER() -> Py { ensure_init(); OWN_PERIOD_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_PERIOD_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = OWN_PERIOD_FILLER.write() { *w = v; } }
pub fn g_OWN_PERIOD_HEADS() -> Py { ensure_init(); OWN_PERIOD_HEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_PERIOD_HEADS(v: Py) { ensure_init(); if let Ok(mut w) = OWN_PERIOD_HEADS.write() { *w = v; } }
pub fn g_OWN_PERIOD_NOUNS() -> Py { ensure_init(); OWN_PERIOD_NOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_PERIOD_NOUNS(v: Py) { ensure_init(); if let Ok(mut w) = OWN_PERIOD_NOUNS.write() { *w = v; } }
pub fn g_OWN_PERIOD_REACH() -> Py { ensure_init(); OWN_PERIOD_REACH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_PERIOD_REACH(v: Py) { ensure_init(); if let Ok(mut w) = OWN_PERIOD_REACH.write() { *w = v; } }
pub fn g_OWN_PERIOD_STOPS() -> Py { ensure_init(); OWN_PERIOD_STOPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_PERIOD_STOPS(v: Py) { ensure_init(); if let Ok(mut w) = OWN_PERIOD_STOPS.write() { *w = v; } }
pub fn g_OWN_RESERVES() -> Py { ensure_init(); OWN_RESERVES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_RESERVES(v: Py) { ensure_init(); if let Ok(mut w) = OWN_RESERVES.write() { *w = v; } }
pub fn g_PARTICLE_STATES() -> Py { ensure_init(); PARTICLE_STATES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PARTICLE_STATES(v: Py) { ensure_init(); if let Ok(mut w) = PARTICLE_STATES.write() { *w = v; } }
pub fn g_PARTS_OF_ME() -> Py { ensure_init(); PARTS_OF_ME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PARTS_OF_ME(v: Py) { ensure_init(); if let Ok(mut w) = PARTS_OF_ME.write() { *w = v; } }
pub fn g_PAST_ASK() -> Py { ensure_init(); PAST_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAST_ASK(v: Py) { ensure_init(); if let Ok(mut w) = PAST_ASK.write() { *w = v; } }
pub fn g_PAST_AUX() -> Py { ensure_init(); PAST_AUX.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAST_AUX(v: Py) { ensure_init(); if let Ok(mut w) = PAST_AUX.write() { *w = v; } }
pub fn g_PAST_ENDS() -> Py { ensure_init(); PAST_ENDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAST_ENDS(v: Py) { ensure_init(); if let Ok(mut w) = PAST_ENDS.write() { *w = v; } }
pub fn g_PERIOD_CUES() -> Py { ensure_init(); PERIOD_CUES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERIOD_CUES(v: Py) { ensure_init(); if let Ok(mut w) = PERIOD_CUES.write() { *w = v; } }
pub fn g_PERSON_NOUNS() -> Py { ensure_init(); PERSON_NOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON_NOUNS(v: Py) { ensure_init(); if let Ok(mut w) = PERSON_NOUNS.write() { *w = v; } }
pub fn g_PERSON_PREPOSITIONS() -> Py { ensure_init(); PERSON_PREPOSITIONS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON_PREPOSITIONS(v: Py) { ensure_init(); if let Ok(mut w) = PERSON_PREPOSITIONS.write() { *w = v; } }
pub fn g_PERSON_SUBJECT() -> Py { ensure_init(); PERSON_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = PERSON_SUBJECT.write() { *w = v; } }
pub fn g_PHRASE_OPENERS() -> Py { ensure_init(); PHRASE_OPENERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PHRASE_OPENERS(v: Py) { ensure_init(); if let Ok(mut w) = PHRASE_OPENERS.write() { *w = v; } }
pub fn g_PICKING() -> Py { ensure_init(); PICKING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PICKING(v: Py) { ensure_init(); if let Ok(mut w) = PICKING.write() { *w = v; } }
pub fn g_PICKING_QUALITY() -> Py { ensure_init(); PICKING_QUALITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PICKING_QUALITY(v: Py) { ensure_init(); if let Ok(mut w) = PICKING_QUALITY.write() { *w = v; } }
pub fn g_PLACE_WORD() -> Py { ensure_init(); PLACE_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLACE_WORD(v: Py) { ensure_init(); if let Ok(mut w) = PLACE_WORD.write() { *w = v; } }
pub fn g_POINTED_AT() -> Py { ensure_init(); POINTED_AT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POINTED_AT(v: Py) { ensure_init(); if let Ok(mut w) = POINTED_AT.write() { *w = v; } }
pub fn g_POLITE() -> Py { ensure_init(); POLITE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POLITE(v: Py) { ensure_init(); if let Ok(mut w) = POLITE.write() { *w = v; } }
pub fn g_POSITIVE() -> Py { ensure_init(); POSITIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POSITIVE(v: Py) { ensure_init(); if let Ok(mut w) = POSITIVE.write() { *w = v; } }
pub fn g_POSSESSIVE_WORDS() -> Py { ensure_init(); POSSESSIVE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POSSESSIVE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = POSSESSIVE_WORDS.write() { *w = v; } }
pub fn g_PRAISE_SAID() -> Py { ensure_init(); PRAISE_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRAISE_SAID(v: Py) { ensure_init(); if let Ok(mut w) = PRAISE_SAID.write() { *w = v; } }
pub fn g_PRAISING() -> Py { ensure_init(); PRAISING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRAISING(v: Py) { ensure_init(); if let Ok(mut w) = PRAISING.write() { *w = v; } }
pub fn g_PRAISING_VERB() -> Py { ensure_init(); PRAISING_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRAISING_VERB(v: Py) { ensure_init(); if let Ok(mut w) = PRAISING_VERB.write() { *w = v; } }
pub fn g_PRODUCT_ASK_TAIL() -> Py { ensure_init(); PRODUCT_ASK_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRODUCT_ASK_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = PRODUCT_ASK_TAIL.write() { *w = v; } }
pub fn g_PRODUCT_ASK_WORDS() -> Py { ensure_init(); PRODUCT_ASK_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRODUCT_ASK_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = PRODUCT_ASK_WORDS.write() { *w = v; } }
pub fn g_PRONOUNS() -> Py { ensure_init(); PRONOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRONOUNS(v: Py) { ensure_init(); if let Ok(mut w) = PRONOUNS.write() { *w = v; } }
pub fn g_PRONOUN_SUBJECT() -> Py { ensure_init(); PRONOUN_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRONOUN_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = PRONOUN_SUBJECT.write() { *w = v; } }
pub fn g_QUESTION_NOUNS() -> Py { ensure_init(); QUESTION_NOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUESTION_NOUNS(v: Py) { ensure_init(); if let Ok(mut w) = QUESTION_NOUNS.write() { *w = v; } }
pub fn g_QUESTION_STARTS() -> Py { ensure_init(); QUESTION_STARTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUESTION_STARTS(v: Py) { ensure_init(); if let Ok(mut w) = QUESTION_STARTS.write() { *w = v; } }
pub fn g_QUESTION_WORD() -> Py { ensure_init(); QUESTION_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUESTION_WORD(v: Py) { ensure_init(); if let Ok(mut w) = QUESTION_WORD.write() { *w = v; } }
pub fn g_QUESTION_WORDS() -> Py { ensure_init(); QUESTION_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUESTION_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = QUESTION_WORDS.write() { *w = v; } }
pub fn g_RECOMMEND() -> Py { ensure_init(); RECOMMEND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECOMMEND(v: Py) { ensure_init(); if let Ok(mut w) = RECOMMEND.write() { *w = v; } }
pub fn g_RECOMMEND_PHRASES() -> Py { ensure_init(); RECOMMEND_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECOMMEND_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = RECOMMEND_PHRASES.write() { *w = v; } }
pub fn g_REFERRING() -> Py { ensure_init(); REFERRING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REFERRING(v: Py) { ensure_init(); if let Ok(mut w) = REFERRING.write() { *w = v; } }
pub fn g_REFUSED_PAIR() -> Py { ensure_init(); REFUSED_PAIR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REFUSED_PAIR(v: Py) { ensure_init(); if let Ok(mut w) = REFUSED_PAIR.write() { *w = v; } }
pub fn g_RELATIVE_SUBJECTS() -> Py { ensure_init(); RELATIVE_SUBJECTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RELATIVE_SUBJECTS(v: Py) { ensure_init(); if let Ok(mut w) = RELATIVE_SUBJECTS.write() { *w = v; } }
pub fn g_REQUEST() -> Py { ensure_init(); REQUEST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REQUEST(v: Py) { ensure_init(); if let Ok(mut w) = REQUEST.write() { *w = v; } }
pub fn g_ROBOT_WORDS() -> Py { ensure_init(); ROBOT_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ROBOT_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = ROBOT_WORDS.write() { *w = v; } }
pub fn g_ROUTINE_IN_THE_PAST() -> Py { ensure_init(); ROUTINE_IN_THE_PAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_IN_THE_PAST(v: Py) { ensure_init(); if let Ok(mut w) = ROUTINE_IN_THE_PAST.write() { *w = v; } }
pub fn g_ROUTINE_IS_A_FAULT() -> Py { ensure_init(); ROUTINE_IS_A_FAULT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_IS_A_FAULT(v: Py) { ensure_init(); if let Ok(mut w) = ROUTINE_IS_A_FAULT.write() { *w = v; } }
pub fn g_ROUTINE_SHAPE() -> Py { ensure_init(); ROUTINE_SHAPE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_SHAPE(v: Py) { ensure_init(); if let Ok(mut w) = ROUTINE_SHAPE.write() { *w = v; } }
pub fn g_RULED_OUT() -> Py { ensure_init(); RULED_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT(v: Py) { ensure_init(); if let Ok(mut w) = RULED_OUT.write() { *w = v; } }
pub fn g_RULED_OUT_CORRECTION() -> Py { ensure_init(); RULED_OUT_CORRECTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_CORRECTION(v: Py) { ensure_init(); if let Ok(mut w) = RULED_OUT_CORRECTION.write() { *w = v; } }
pub fn g_RULED_OUT_ENDS() -> Py { ensure_init(); RULED_OUT_ENDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_ENDS(v: Py) { ensure_init(); if let Ok(mut w) = RULED_OUT_ENDS.write() { *w = v; } }
pub fn g_RULED_OUT_NO_THING() -> Py { ensure_init(); RULED_OUT_NO_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_NO_THING(v: Py) { ensure_init(); if let Ok(mut w) = RULED_OUT_NO_THING.write() { *w = v; } }
pub fn g_SAD_PLAIN_CUES() -> Py { ensure_init(); SAD_PLAIN_CUES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAD_PLAIN_CUES(v: Py) { ensure_init(); if let Ok(mut w) = SAD_PLAIN_CUES.write() { *w = v; } }
pub fn g_SAID_AS_A_NEGATION() -> Py { ensure_init(); SAID_AS_A_NEGATION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_AS_A_NEGATION(v: Py) { ensure_init(); if let Ok(mut w) = SAID_AS_A_NEGATION.write() { *w = v; } }
pub fn g_SAYS_ABOUT_YOU() -> Py { ensure_init(); SAYS_ABOUT_YOU.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAYS_ABOUT_YOU(v: Py) { ensure_init(); if let Ok(mut w) = SAYS_ABOUT_YOU.write() { *w = v; } }
pub fn g_SAYS_HOW_THEY_ARE() -> Py { ensure_init(); SAYS_HOW_THEY_ARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAYS_HOW_THEY_ARE(v: Py) { ensure_init(); if let Ok(mut w) = SAYS_HOW_THEY_ARE.write() { *w = v; } }
pub fn g_SAYS_WHAT_ARE_YOU() -> Py { ensure_init(); SAYS_WHAT_ARE_YOU.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAYS_WHAT_ARE_YOU(v: Py) { ensure_init(); if let Ok(mut w) = SAYS_WHAT_ARE_YOU.write() { *w = v; } }
pub fn g_SAYS_WHAT_IT_IS() -> Py { ensure_init(); SAYS_WHAT_IT_IS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAYS_WHAT_IT_IS(v: Py) { ensure_init(); if let Ok(mut w) = SAYS_WHAT_IT_IS.write() { *w = v; } }
pub fn g_SCALE_ASK() -> Py { ensure_init(); SCALE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SCALE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = SCALE_ASK.write() { *w = v; } }
pub fn g_SELF_ASK() -> Py { ensure_init(); SELF_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_ASK(v: Py) { ensure_init(); if let Ok(mut w) = SELF_ASK.write() { *w = v; } }
pub fn g_SELF_FILLER() -> Py { ensure_init(); SELF_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = SELF_FILLER.write() { *w = v; } }
pub fn g_SELF_ONLY_CUES() -> Py { ensure_init(); SELF_ONLY_CUES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_ONLY_CUES(v: Py) { ensure_init(); if let Ok(mut w) = SELF_ONLY_CUES.write() { *w = v; } }
pub fn g_SELF_ONLY_STATE() -> Py { ensure_init(); SELF_ONLY_STATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_ONLY_STATE(v: Py) { ensure_init(); if let Ok(mut w) = SELF_ONLY_STATE.write() { *w = v; } }
pub fn g_SELF_WORDS() -> Py { ensure_init(); SELF_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = SELF_WORDS.write() { *w = v; } }
pub fn g_SETUP_CONNECTIVES() -> Py { ensure_init(); SETUP_CONNECTIVES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SETUP_CONNECTIVES(v: Py) { ensure_init(); if let Ok(mut w) = SETUP_CONNECTIVES.write() { *w = v; } }
pub fn g_SETUP_MOST_WORDS() -> Py { ensure_init(); SETUP_MOST_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SETUP_MOST_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = SETUP_MOST_WORDS.write() { *w = v; } }
pub fn g_SETUP_NOTHING() -> Py { ensure_init(); SETUP_NOTHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SETUP_NOTHING(v: Py) { ensure_init(); if let Ok(mut w) = SETUP_NOTHING.write() { *w = v; } }
pub fn g_SHIFT_EARLY() -> Py { ensure_init(); SHIFT_EARLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHIFT_EARLY(v: Py) { ensure_init(); if let Ok(mut w) = SHIFT_EARLY.write() { *w = v; } }
pub fn g_SHIFT_LATE() -> Py { ensure_init(); SHIFT_LATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHIFT_LATE(v: Py) { ensure_init(); if let Ok(mut w) = SHIFT_LATE.write() { *w = v; } }
pub fn g_SHIFT_NIGHT() -> Py { ensure_init(); SHIFT_NIGHT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHIFT_NIGHT(v: Py) { ensure_init(); if let Ok(mut w) = SHIFT_NIGHT.write() { *w = v; } }
pub fn g_SHORT_REPLIES() -> Py { ensure_init(); SHORT_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHORT_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = SHORT_REPLIES.write() { *w = v; } }
pub fn g_SIGNING_OFF() -> Py { ensure_init(); SIGNING_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SIGNING_OFF(v: Py) { ensure_init(); if let Ok(mut w) = SIGNING_OFF.write() { *w = v; } }
pub fn g_SLIPPED_FROM() -> Py { ensure_init(); SLIPPED_FROM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLIPPED_FROM(v: Py) { ensure_init(); if let Ok(mut w) = SLIPPED_FROM.write() { *w = v; } }
pub fn g_SMALL_TALK() -> Py { ensure_init(); SMALL_TALK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SMALL_TALK(v: Py) { ensure_init(); if let Ok(mut w) = SMALL_TALK.write() { *w = v; } }
pub fn g_SMALL_TALK_LEFTOVER() -> Py { ensure_init(); SMALL_TALK_LEFTOVER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SMALL_TALK_LEFTOVER(v: Py) { ensure_init(); if let Ok(mut w) = SMALL_TALK_LEFTOVER.write() { *w = v; } }
pub fn g_SMALL_TALK_WORDS() -> Py { ensure_init(); SMALL_TALK_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SMALL_TALK_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = SMALL_TALK_WORDS.write() { *w = v; } }
pub fn g_SOUND_FOLLOWS() -> Py { ensure_init(); SOUND_FOLLOWS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOUND_FOLLOWS(v: Py) { ensure_init(); if let Ok(mut w) = SOUND_FOLLOWS.write() { *w = v; } }
pub fn g_SPEAKER() -> Py { ensure_init(); SPEAKER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPEAKER(v: Py) { ensure_init(); if let Ok(mut w) = SPEAKER.write() { *w = v; } }
pub fn g_SPEAKER_TAIL() -> Py { ensure_init(); SPEAKER_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPEAKER_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = SPEAKER_TAIL.write() { *w = v; } }
pub fn g_SPENDS_ITS_TIME() -> Py { ensure_init(); SPENDS_ITS_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPENDS_ITS_TIME(v: Py) { ensure_init(); if let Ok(mut w) = SPENDS_ITS_TIME.write() { *w = v; } }
pub fn g_SPENT_TIME() -> Py { ensure_init(); SPENT_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPENT_TIME(v: Py) { ensure_init(); if let Ok(mut w) = SPENT_TIME.write() { *w = v; } }
pub fn g_STARTED_OUT() -> Py { ensure_init(); STARTED_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STARTED_OUT(v: Py) { ensure_init(); if let Ok(mut w) = STARTED_OUT.write() { *w = v; } }
pub fn g_STATE_AS_FEELING() -> Py { ensure_init(); STATE_AS_FEELING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STATE_AS_FEELING(v: Py) { ensure_init(); if let Ok(mut w) = STATE_AS_FEELING.write() { *w = v; } }
pub fn g_STATE_JOINERS() -> Py { ensure_init(); STATE_JOINERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STATE_JOINERS(v: Py) { ensure_init(); if let Ok(mut w) = STATE_JOINERS.write() { *w = v; } }
pub fn g_STATE_NOUNS() -> Py { ensure_init(); STATE_NOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STATE_NOUNS(v: Py) { ensure_init(); if let Ok(mut w) = STATE_NOUNS.write() { *w = v; } }
pub fn g_STATE_TAIL() -> Py { ensure_init(); STATE_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STATE_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = STATE_TAIL.write() { *w = v; } }
pub fn g_STATE_WORDS() -> Py { ensure_init(); STATE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STATE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = STATE_WORDS.write() { *w = v; } }
pub fn g_STATING_VERB() -> Py { ensure_init(); STATING_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STATING_VERB(v: Py) { ensure_init(); if let Ok(mut w) = STATING_VERB.write() { *w = v; } }
pub fn g_STEPS() -> Py { ensure_init(); STEPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEPS(v: Py) { ensure_init(); if let Ok(mut w) = STEPS.write() { *w = v; } }
pub fn g_STEPS_WORD() -> Py { ensure_init(); STEPS_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEPS_WORD(v: Py) { ensure_init(); if let Ok(mut w) = STEPS_WORD.write() { *w = v; } }
pub fn g_STEP_ANY_OBJECT() -> Py { ensure_init(); STEP_ANY_OBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_ANY_OBJECT(v: Py) { ensure_init(); if let Ok(mut w) = STEP_ANY_OBJECT.write() { *w = v; } }
pub fn g_STEP_VERBS() -> Py { ensure_init(); STEP_VERBS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_VERBS(v: Py) { ensure_init(); if let Ok(mut w) = STEP_VERBS.write() { *w = v; } }
pub fn g_STEP_WRONG() -> Py { ensure_init(); STEP_WRONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_WRONG(v: Py) { ensure_init(); if let Ok(mut w) = STEP_WRONG.write() { *w = v; } }
pub fn g_STILL_COMING() -> Py { ensure_init(); STILL_COMING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_COMING(v: Py) { ensure_init(); if let Ok(mut w) = STILL_COMING.write() { *w = v; } }
pub fn g_STILL_WRONG() -> Py { ensure_init(); STILL_WRONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_WRONG(v: Py) { ensure_init(); if let Ok(mut w) = STILL_WRONG.write() { *w = v; } }
pub fn g_STOMACH_WORDS() -> Py { ensure_init(); STOMACH_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STOMACH_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = STOMACH_WORDS.write() { *w = v; } }
pub fn g_STOP_ASK() -> Py { ensure_init(); STOP_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STOP_ASK(v: Py) { ensure_init(); if let Ok(mut w) = STOP_ASK.write() { *w = v; } }
pub fn g_STOP_ASK_FILLER() -> Py { ensure_init(); STOP_ASK_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STOP_ASK_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = STOP_ASK_FILLER.write() { *w = v; } }
pub fn g_SUBJECT_NEGATIONS() -> Py { ensure_init(); SUBJECT_NEGATIONS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUBJECT_NEGATIONS(v: Py) { ensure_init(); if let Ok(mut w) = SUBJECT_NEGATIONS.write() { *w = v; } }
pub fn g_SUBJECT_OF_ITS_OWN() -> Py { ensure_init(); SUBJECT_OF_ITS_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUBJECT_OF_ITS_OWN(v: Py) { ensure_init(); if let Ok(mut w) = SUBJECT_OF_ITS_OWN.write() { *w = v; } }
pub fn g_SUMMONS_CALL_DONE() -> Py { ensure_init(); SUMMONS_CALL_DONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUMMONS_CALL_DONE(v: Py) { ensure_init(); if let Ok(mut w) = SUMMONS_CALL_DONE.write() { *w = v; } }
pub fn g_SUMMONS_NOT() -> Py { ensure_init(); SUMMONS_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUMMONS_NOT(v: Py) { ensure_init(); if let Ok(mut w) = SUMMONS_NOT.write() { *w = v; } }
pub fn g_SUMMONS_RANG() -> Py { ensure_init(); SUMMONS_RANG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUMMONS_RANG(v: Py) { ensure_init(); if let Ok(mut w) = SUMMONS_RANG.write() { *w = v; } }
pub fn g_SUMMONS_SAID() -> Py { ensure_init(); SUMMONS_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUMMONS_SAID(v: Py) { ensure_init(); if let Ok(mut w) = SUMMONS_SAID.write() { *w = v; } }
pub fn g_SUMMONS_SEE() -> Py { ensure_init(); SUMMONS_SEE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUMMONS_SEE(v: Py) { ensure_init(); if let Ok(mut w) = SUMMONS_SEE.write() { *w = v; } }
pub fn g_SUMMONS_VERB() -> Py { ensure_init(); SUMMONS_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUMMONS_VERB(v: Py) { ensure_init(); if let Ok(mut w) = SUMMONS_VERB.write() { *w = v; } }
pub fn g_SUMMONS_WANTS() -> Py { ensure_init(); SUMMONS_WANTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUMMONS_WANTS(v: Py) { ensure_init(); if let Ok(mut w) = SUMMONS_WANTS.write() { *w = v; } }
pub fn g_SUMMONS_WHO() -> Py { ensure_init(); SUMMONS_WHO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUMMONS_WHO(v: Py) { ensure_init(); if let Ok(mut w) = SUMMONS_WHO.write() { *w = v; } }
pub fn g_SUMMONS_WHO_PHRASES() -> Py { ensure_init(); SUMMONS_WHO_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUMMONS_WHO_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = SUMMONS_WHO_PHRASES.write() { *w = v; } }
pub fn g_SUMMONS_WRITTEN() -> Py { ensure_init(); SUMMONS_WRITTEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUMMONS_WRITTEN(v: Py) { ensure_init(); if let Ok(mut w) = SUMMONS_WRITTEN.write() { *w = v; } }
pub fn g_SUPPLY_PREPOSITIONS() -> Py { ensure_init(); SUPPLY_PREPOSITIONS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUPPLY_PREPOSITIONS(v: Py) { ensure_init(); if let Ok(mut w) = SUPPLY_PREPOSITIONS.write() { *w = v; } }
pub fn g_SUPPLY_WORDS() -> Py { ensure_init(); SUPPLY_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUPPLY_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = SUPPLY_WORDS.write() { *w = v; } }
pub fn g_TAG_CHECKABLE() -> Py { ensure_init(); TAG_CHECKABLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAG_CHECKABLE(v: Py) { ensure_init(); if let Ok(mut w) = TAG_CHECKABLE.write() { *w = v; } }
pub fn g_TAG_ENDINGS() -> Py { ensure_init(); TAG_ENDINGS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAG_ENDINGS(v: Py) { ensure_init(); if let Ok(mut w) = TAG_ENDINGS.write() { *w = v; } }
pub fn g_TAG_PLAN() -> Py { ensure_init(); TAG_PLAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAG_PLAN(v: Py) { ensure_init(); if let Ok(mut w) = TAG_PLAN.write() { *w = v; } }
pub fn g_TAG_RIGHT_NOT() -> Py { ensure_init(); TAG_RIGHT_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAG_RIGHT_NOT(v: Py) { ensure_init(); if let Ok(mut w) = TAG_RIGHT_NOT.write() { *w = v; } }
pub fn g_TAG_TIME_NUMBER() -> Py { ensure_init(); TAG_TIME_NUMBER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAG_TIME_NUMBER(v: Py) { ensure_init(); if let Ok(mut w) = TAG_TIME_NUMBER.write() { *w = v; } }
pub fn g_TAIL_ASKS() -> Py { ensure_init(); TAIL_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAIL_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = TAIL_ASKS.write() { *w = v; } }
pub fn g_TAIL_ASK_FILLER() -> Py { ensure_init(); TAIL_ASK_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAIL_ASK_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = TAIL_ASK_FILLER.write() { *w = v; } }
pub fn g_TAIL_STOPS() -> Py { ensure_init(); TAIL_STOPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAIL_STOPS(v: Py) { ensure_init(); if let Ok(mut w) = TAIL_STOPS.write() { *w = v; } }
pub fn g_TALK_ASK_OPENERS() -> Py { ensure_init(); TALK_ASK_OPENERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TALK_ASK_OPENERS(v: Py) { ensure_init(); if let Ok(mut w) = TALK_ASK_OPENERS.write() { *w = v; } }
pub fn g_TALK_ASK_VERBS() -> Py { ensure_init(); TALK_ASK_VERBS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TALK_ASK_VERBS(v: Py) { ensure_init(); if let Ok(mut w) = TALK_ASK_VERBS.write() { *w = v; } }
pub fn g_TALK_HEAD() -> Py { ensure_init(); TALK_HEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TALK_HEAD(v: Py) { ensure_init(); if let Ok(mut w) = TALK_HEAD.write() { *w = v; } }
pub fn g_TALK_OPENING() -> Py { ensure_init(); TALK_OPENING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TALK_OPENING(v: Py) { ensure_init(); if let Ok(mut w) = TALK_OPENING.write() { *w = v; } }
pub fn g_TALK_TAIL() -> Py { ensure_init(); TALK_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TALK_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = TALK_TAIL.write() { *w = v; } }
pub fn g_TALK_TAIL_WORDS() -> Py { ensure_init(); TALK_TAIL_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TALK_TAIL_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = TALK_TAIL_WORDS.write() { *w = v; } }
pub fn g_TASTE_VERBS() -> Py { ensure_init(); TASTE_VERBS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TASTE_VERBS(v: Py) { ensure_init(); if let Ok(mut w) = TASTE_VERBS.write() { *w = v; } }
pub fn g_TERMS_OF_ADDRESS() -> Py { ensure_init(); TERMS_OF_ADDRESS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TERMS_OF_ADDRESS(v: Py) { ensure_init(); if let Ok(mut w) = TERMS_OF_ADDRESS.write() { *w = v; } }
pub fn g_THANKS() -> Py { ensure_init(); THANKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THANKS(v: Py) { ensure_init(); if let Ok(mut w) = THANKS.write() { *w = v; } }
pub fn g_THANKS_ABOUT() -> Py { ensure_init(); THANKS_ABOUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THANKS_ABOUT(v: Py) { ensure_init(); if let Ok(mut w) = THANKS_ABOUT.write() { *w = v; } }
pub fn g_THANKS_ASK_NOUNS() -> Py { ensure_init(); THANKS_ASK_NOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THANKS_ASK_NOUNS(v: Py) { ensure_init(); if let Ok(mut w) = THANKS_ASK_NOUNS.write() { *w = v; } }
pub fn g_THANKS_ASK_PREPS() -> Py { ensure_init(); THANKS_ASK_PREPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THANKS_ASK_PREPS(v: Py) { ensure_init(); if let Ok(mut w) = THANKS_ASK_PREPS.write() { *w = v; } }
pub fn g_THANKS_ASK_WORDS() -> Py { ensure_init(); THANKS_ASK_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THANKS_ASK_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = THANKS_ASK_WORDS.write() { *w = v; } }
pub fn g_THANKS_NOISE() -> Py { ensure_init(); THANKS_NOISE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THANKS_NOISE(v: Py) { ensure_init(); if let Ok(mut w) = THANKS_NOISE.write() { *w = v; } }
pub fn g_THEIR_PROBLEM() -> Py { ensure_init(); THEIR_PROBLEM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIR_PROBLEM(v: Py) { ensure_init(); if let Ok(mut w) = THEIR_PROBLEM.write() { *w = v; } }
pub fn g_THING_OR_PERSON_CUES() -> Py { ensure_init(); THING_OR_PERSON_CUES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THING_OR_PERSON_CUES(v: Py) { ensure_init(); if let Ok(mut w) = THING_OR_PERSON_CUES.write() { *w = v; } }
pub fn g_TIME_AFTER_A_PARTICLE() -> Py { ensure_init(); TIME_AFTER_A_PARTICLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIME_AFTER_A_PARTICLE(v: Py) { ensure_init(); if let Ok(mut w) = TIME_AFTER_A_PARTICLE.write() { *w = v; } }
pub fn g_TIME_FILLERS() -> Py { ensure_init(); TIME_FILLERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIME_FILLERS(v: Py) { ensure_init(); if let Ok(mut w) = TIME_FILLERS.write() { *w = v; } }
pub fn g_TIME_MARKERS() -> Py { ensure_init(); TIME_MARKERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIME_MARKERS(v: Py) { ensure_init(); if let Ok(mut w) = TIME_MARKERS.write() { *w = v; } }
pub fn g_TIPS() -> Py { ensure_init(); TIPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIPS(v: Py) { ensure_init(); if let Ok(mut w) = TIPS.write() { *w = v; } }
pub fn g_TIPS_ABOUT() -> Py { ensure_init(); TIPS_ABOUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIPS_ABOUT(v: Py) { ensure_init(); if let Ok(mut w) = TIPS_ABOUT.write() { *w = v; } }
pub fn g_TIPS_AMOUNT() -> Py { ensure_init(); TIPS_AMOUNT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIPS_AMOUNT(v: Py) { ensure_init(); if let Ok(mut w) = TIPS_AMOUNT.write() { *w = v; } }
pub fn g_TIPS_FRAME() -> Py { ensure_init(); TIPS_FRAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIPS_FRAME(v: Py) { ensure_init(); if let Ok(mut w) = TIPS_FRAME.write() { *w = v; } }
pub fn g_TIPS_SUBJECT() -> Py { ensure_init(); TIPS_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIPS_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = TIPS_SUBJECT.write() { *w = v; } }
pub fn g_TIPS_WANTED() -> Py { ensure_init(); TIPS_WANTED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIPS_WANTED(v: Py) { ensure_init(); if let Ok(mut w) = TIPS_WANTED.write() { *w = v; } }
pub fn g_TIP_WORDS() -> Py { ensure_init(); TIP_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIP_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = TIP_WORDS.write() { *w = v; } }
pub fn g_TO_A_VERB() -> Py { ensure_init(); TO_A_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TO_A_VERB(v: Py) { ensure_init(); if let Ok(mut w) = TO_A_VERB.write() { *w = v; } }
pub fn g_TURNED_ROUND() -> Py { ensure_init(); TURNED_ROUND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TURNED_ROUND(v: Py) { ensure_init(); if let Ok(mut w) = TURNED_ROUND.write() { *w = v; } }
pub fn g_VERBISH() -> Py { ensure_init(); VERBISH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERBISH(v: Py) { ensure_init(); if let Ok(mut w) = VERBISH.write() { *w = v; } }
pub fn g_VERB_FORMS() -> Py { ensure_init(); VERB_FORMS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERB_FORMS(v: Py) { ensure_init(); if let Ok(mut w) = VERB_FORMS.write() { *w = v; } }
pub fn g_VERB_ONLY_STATE() -> Py { ensure_init(); VERB_ONLY_STATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERB_ONLY_STATE(v: Py) { ensure_init(); if let Ok(mut w) = VERB_ONLY_STATE.write() { *w = v; } }
pub fn g_VERB_THIRD() -> Py { ensure_init(); VERB_THIRD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERB_THIRD(v: Py) { ensure_init(); if let Ok(mut w) = VERB_THIRD.write() { *w = v; } }
pub fn g_VERDICT_ADJECTIVES() -> Py { ensure_init(); VERDICT_ADJECTIVES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERDICT_ADJECTIVES(v: Py) { ensure_init(); if let Ok(mut w) = VERDICT_ADJECTIVES.write() { *w = v; } }
pub fn g_VERDICT_ASK() -> Py { ensure_init(); VERDICT_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERDICT_ASK(v: Py) { ensure_init(); if let Ok(mut w) = VERDICT_ASK.write() { *w = v; } }
pub fn g_VERDICT_FILLER() -> Py { ensure_init(); VERDICT_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERDICT_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = VERDICT_FILLER.write() { *w = v; } }
pub fn g_VERDICT_NOUNS() -> Py { ensure_init(); VERDICT_NOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERDICT_NOUNS(v: Py) { ensure_init(); if let Ok(mut w) = VERDICT_NOUNS.write() { *w = v; } }
pub fn g_WHEN_IN_THE_ASK() -> Py { ensure_init(); WHEN_IN_THE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_IN_THE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_IN_THE_ASK.write() { *w = v; } }
pub fn g_WHERE_BOT_TAIL() -> Py { ensure_init(); WHERE_BOT_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHERE_BOT_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = WHERE_BOT_TAIL.write() { *w = v; } }
pub fn g_WHILE_ASK() -> Py { ensure_init(); WHILE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHILE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = WHILE_ASK.write() { *w = v; } }
pub fn g_WHO_A_THING() -> Py { ensure_init(); WHO_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHO_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = WHO_A_THING.write() { *w = v; } }
pub fn g_WHO_TAIL() -> Py { ensure_init(); WHO_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHO_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = WHO_TAIL.write() { *w = v; } }
pub fn g_WHO_TALKING() -> Py { ensure_init(); WHO_TALKING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHO_TALKING(v: Py) { ensure_init(); if let Ok(mut w) = WHO_TALKING.write() { *w = v; } }
pub fn g_WH_CONTINUES() -> Py { ensure_init(); WH_CONTINUES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WH_CONTINUES(v: Py) { ensure_init(); if let Ok(mut w) = WH_CONTINUES.write() { *w = v; } }
pub fn g_WORK_ON_ME() -> Py { ensure_init(); WORK_ON_ME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORK_ON_ME(v: Py) { ensure_init(); if let Ok(mut w) = WORK_ON_ME.write() { *w = v; } }
pub fn g_WRITE() -> Py { ensure_init(); WRITE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WRITE(v: Py) { ensure_init(); if let Ok(mut w) = WRITE.write() { *w = v; } }
pub fn g_YESNO_STARTS() -> Py { ensure_init(); YESNO_STARTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_YESNO_STARTS(v: Py) { ensure_init(); if let Ok(mut w) = YESNO_STARTS.write() { *w = v; } }
pub fn g_YOU_WORDS() -> Py { ensure_init(); YOU_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_YOU_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = YOU_WORDS.write() { *w = v; } }
pub fn g__ASKS_AFTER_A_COMMA() -> Py { ensure_init(); _ASKS_AFTER_A_COMMA.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASKS_AFTER_A_COMMA(v: Py) { ensure_init(); if let Ok(mut w) = _ASKS_AFTER_A_COMMA.write() { *w = v; } }
pub fn g__ASKS_AFTER_A_COMMA_WORD() -> Py { ensure_init(); _ASKS_AFTER_A_COMMA_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASKS_AFTER_A_COMMA_WORD(v: Py) { ensure_init(); if let Ok(mut w) = _ASKS_AFTER_A_COMMA_WORD.write() { *w = v; } }
pub fn g__ASKS_MID_LINE() -> Py { ensure_init(); _ASKS_MID_LINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASKS_MID_LINE(v: Py) { ensure_init(); if let Ok(mut w) = _ASKS_MID_LINE.write() { *w = v; } }
pub fn g__ASKS_WITH_AN_AUXILIARY() -> Py { ensure_init(); _ASKS_WITH_AN_AUXILIARY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASKS_WITH_AN_AUXILIARY(v: Py) { ensure_init(); if let Ok(mut w) = _ASKS_WITH_AN_AUXILIARY.write() { *w = v; } }
pub fn g__ASKS_WITH_A_QUESTION_WORD() -> Py { ensure_init(); _ASKS_WITH_A_QUESTION_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASKS_WITH_A_QUESTION_WORD(v: Py) { ensure_init(); if let Ok(mut w) = _ASKS_WITH_A_QUESTION_WORD.write() { *w = v; } }
pub fn g__AVAILABILITY() -> Py { ensure_init(); _AVAILABILITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AVAILABILITY(v: Py) { ensure_init(); if let Ok(mut w) = _AVAILABILITY.write() { *w = v; } }
pub fn g__A_AN_AMOUNT_OF_A_THING() -> Py { ensure_init(); _A_AN_AMOUNT_OF_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_AN_AMOUNT_OF_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = _A_AN_AMOUNT_OF_A_THING.write() { *w = v; } }
pub fn g__A_AN_OCCASION_OF_THEIR_OWN() -> Py { ensure_init(); _A_AN_OCCASION_OF_THEIR_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_AN_OCCASION_OF_THEIR_OWN(v: Py) { ensure_init(); if let Ok(mut w) = _A_AN_OCCASION_OF_THEIR_OWN.write() { *w = v; } }
pub fn g__A_ARITHMETIC() -> Py { ensure_init(); _A_ARITHMETIC.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_ARITHMETIC(v: Py) { ensure_init(); if let Ok(mut w) = _A_ARITHMETIC.write() { *w = v; } }
pub fn g__A_A_CONVERSION() -> Py { ensure_init(); _A_A_CONVERSION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_A_CONVERSION(v: Py) { ensure_init(); if let Ok(mut w) = _A_A_CONVERSION.write() { *w = v; } }
pub fn g__A_A_DIFFERENCE() -> Py { ensure_init(); _A_A_DIFFERENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_A_DIFFERENCE(v: Py) { ensure_init(); if let Ok(mut w) = _A_A_DIFFERENCE.write() { *w = v; } }
pub fn g__A_A_PLAN_ABOUT_THE_WEATHER() -> Py { ensure_init(); _A_A_PLAN_ABOUT_THE_WEATHER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_A_PLAN_ABOUT_THE_WEATHER(v: Py) { ensure_init(); if let Ok(mut w) = _A_A_PLAN_ABOUT_THE_WEATHER.write() { *w = v; } }
pub fn g__A_A_TRAFFIC_LIGHT() -> Py { ensure_init(); _A_A_TRAFFIC_LIGHT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_A_TRAFFIC_LIGHT(v: Py) { ensure_init(); if let Ok(mut w) = _A_A_TRAFFIC_LIGHT.write() { *w = v; } }
pub fn g__A_BARE_ASK() -> Py { ensure_init(); _A_BARE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_BARE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = _A_BARE_ASK.write() { *w = v; } }
pub fn g__A_BARE_LIVE() -> Py { ensure_init(); _A_BARE_LIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_BARE_LIVE(v: Py) { ensure_init(); if let Ok(mut w) = _A_BARE_LIVE.write() { *w = v; } }
pub fn g__A_BARE_LIVE_WORDS() -> Py { ensure_init(); _A_BARE_LIVE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_BARE_LIVE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = _A_BARE_LIVE_WORDS.write() { *w = v; } }
pub fn g__A_BARE_RECORD() -> Py { ensure_init(); _A_BARE_RECORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_BARE_RECORD(v: Py) { ensure_init(); if let Ok(mut w) = _A_BARE_RECORD.write() { *w = v; } }
pub fn g__A_BARE_TAIL() -> Py { ensure_init(); _A_BARE_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_BARE_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = _A_BARE_TAIL.write() { *w = v; } }
pub fn g__A_CLASS_OF_THING() -> Py { ensure_init(); _A_CLASS_OF_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_CLASS_OF_THING(v: Py) { ensure_init(); if let Ok(mut w) = _A_CLASS_OF_THING.write() { *w = v; } }
pub fn g__A_COUNT() -> Py { ensure_init(); _A_COUNT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_COUNT(v: Py) { ensure_init(); if let Ok(mut w) = _A_COUNT.write() { *w = v; } }
pub fn g__A_DAY_OR_TIME() -> Py { ensure_init(); _A_DAY_OR_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_DAY_OR_TIME(v: Py) { ensure_init(); if let Ok(mut w) = _A_DAY_OR_TIME.write() { *w = v; } }
pub fn g__A_DAY_WORD() -> Py { ensure_init(); _A_DAY_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_DAY_WORD(v: Py) { ensure_init(); if let Ok(mut w) = _A_DAY_WORD.write() { *w = v; } }
pub fn g__A_DEFINITION() -> Py { ensure_init(); _A_DEFINITION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_DEFINITION(v: Py) { ensure_init(); if let Ok(mut w) = _A_DEFINITION.write() { *w = v; } }
pub fn g__A_DETERMINER() -> Py { ensure_init(); _A_DETERMINER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_DETERMINER(v: Py) { ensure_init(); if let Ok(mut w) = _A_DETERMINER.write() { *w = v; } }
pub fn g__A_DID_YOU_SEE() -> Py { ensure_init(); _A_DID_YOU_SEE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_DID_YOU_SEE(v: Py) { ensure_init(); if let Ok(mut w) = _A_DID_YOU_SEE.write() { *w = v; } }
pub fn g__A_FOLLOW_UP_WORDS() -> Py { ensure_init(); _A_FOLLOW_UP_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_FOLLOW_UP_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = _A_FOLLOW_UP_WORDS.write() { *w = v; } }
pub fn g__A_FRESHENING() -> Py { ensure_init(); _A_FRESHENING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_FRESHENING(v: Py) { ensure_init(); if let Ok(mut w) = _A_FRESHENING.write() { *w = v; } }
pub fn g__A_HAS_A_NUMBER() -> Py { ensure_init(); _A_HAS_A_NUMBER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HAS_A_NUMBER(v: Py) { ensure_init(); if let Ok(mut w) = _A_HAS_A_NUMBER.write() { *w = v; } }
pub fn g__A_HISTORY() -> Py { ensure_init(); _A_HISTORY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HISTORY(v: Py) { ensure_init(); if let Ok(mut w) = _A_HISTORY.write() { *w = v; } }
pub fn g__A_HISTORY_ITSELF() -> Py { ensure_init(); _A_HISTORY_ITSELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HISTORY_ITSELF(v: Py) { ensure_init(); if let Ok(mut w) = _A_HISTORY_ITSELF.write() { *w = v; } }
pub fn g__A_HOW_A_THING_MEASURES() -> Py { ensure_init(); _A_HOW_A_THING_MEASURES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HOW_A_THING_MEASURES(v: Py) { ensure_init(); if let Ok(mut w) = _A_HOW_A_THING_MEASURES.write() { *w = v; } }
pub fn g__A_HOW_IT_WORKS() -> Py { ensure_init(); _A_HOW_IT_WORKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HOW_IT_WORKS(v: Py) { ensure_init(); if let Ok(mut w) = _A_HOW_IT_WORKS.write() { *w = v; } }
pub fn g__A_HOW_MANY() -> Py { ensure_init(); _A_HOW_MANY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HOW_MANY(v: Py) { ensure_init(); if let Ok(mut w) = _A_HOW_MANY.write() { *w = v; } }
pub fn g__A_HOW_MUCH_A_FEELING() -> Py { ensure_init(); _A_HOW_MUCH_A_FEELING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HOW_MUCH_A_FEELING(v: Py) { ensure_init(); if let Ok(mut w) = _A_HOW_MUCH_A_FEELING.write() { *w = v; } }
pub fn g__A_HOW_MUCH_ENOUGH() -> Py { ensure_init(); _A_HOW_MUCH_ENOUGH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HOW_MUCH_ENOUGH(v: Py) { ensure_init(); if let Ok(mut w) = _A_HOW_MUCH_ENOUGH.write() { *w = v; } }
pub fn g__A_HOW_MUCH_IS_IN() -> Py { ensure_init(); _A_HOW_MUCH_IS_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HOW_MUCH_IS_IN(v: Py) { ensure_init(); if let Ok(mut w) = _A_HOW_MUCH_IS_IN.write() { *w = v; } }
pub fn g__A_HOW_MUCH_IT_HOLDS() -> Py { ensure_init(); _A_HOW_MUCH_IT_HOLDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HOW_MUCH_IT_HOLDS(v: Py) { ensure_init(); if let Ok(mut w) = _A_HOW_MUCH_IT_HOLDS.write() { *w = v; } }
pub fn g__A_HOW_MUCH_I_SPENT() -> Py { ensure_init(); _A_HOW_MUCH_I_SPENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HOW_MUCH_I_SPENT(v: Py) { ensure_init(); if let Ok(mut w) = _A_HOW_MUCH_I_SPENT.write() { *w = v; } }
pub fn g__A_HOW_MUCH_OF_A_UNIT() -> Py { ensure_init(); _A_HOW_MUCH_OF_A_UNIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HOW_MUCH_OF_A_UNIT(v: Py) { ensure_init(); if let Ok(mut w) = _A_HOW_MUCH_OF_A_UNIT.write() { *w = v; } }
pub fn g__A_HOW_OLD_IS_THE_WORLD() -> Py { ensure_init(); _A_HOW_OLD_IS_THE_WORLD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HOW_OLD_IS_THE_WORLD(v: Py) { ensure_init(); if let Ok(mut w) = _A_HOW_OLD_IS_THE_WORLD.write() { *w = v; } }
pub fn g__A_LINE_THAT_NAMES_A_PLACE() -> Py { ensure_init(); _A_LINE_THAT_NAMES_A_PLACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_LINE_THAT_NAMES_A_PLACE(v: Py) { ensure_init(); if let Ok(mut w) = _A_LINE_THAT_NAMES_A_PLACE.write() { *w = v; } }
pub fn g__A_MADE_OF() -> Py { ensure_init(); _A_MADE_OF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_MADE_OF(v: Py) { ensure_init(); if let Ok(mut w) = _A_MADE_OF.write() { *w = v; } }
pub fn g__A_MEASURE() -> Py { ensure_init(); _A_MEASURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_MEASURE(v: Py) { ensure_init(); if let Ok(mut w) = _A_MEASURE.write() { *w = v; } }
pub fn g__A_METHOD() -> Py { ensure_init(); _A_METHOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_METHOD(v: Py) { ensure_init(); if let Ok(mut w) = _A_METHOD.write() { *w = v; } }
pub fn g__A_MY_OWN_FIGURE() -> Py { ensure_init(); _A_MY_OWN_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_MY_OWN_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = _A_MY_OWN_FIGURE.write() { *w = v; } }
pub fn g__A_MY_OWN_PLAN() -> Py { ensure_init(); _A_MY_OWN_PLAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_MY_OWN_PLAN(v: Py) { ensure_init(); if let Ok(mut w) = _A_MY_OWN_PLAN.write() { *w = v; } }
pub fn g__A_PAST_YEAR() -> Py { ensure_init(); _A_PAST_YEAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_PAST_YEAR(v: Py) { ensure_init(); if let Ok(mut w) = _A_PAST_YEAR.write() { *w = v; } }
pub fn g__A_PLACE_THAT_KEEPS_HOURS() -> Py { ensure_init(); _A_PLACE_THAT_KEEPS_HOURS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_PLACE_THAT_KEEPS_HOURS(v: Py) { ensure_init(); if let Ok(mut w) = _A_PLACE_THAT_KEEPS_HOURS.write() { *w = v; } }
pub fn g__A_POST_HOLDER() -> Py { ensure_init(); _A_POST_HOLDER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_POST_HOLDER(v: Py) { ensure_init(); if let Ok(mut w) = _A_POST_HOLDER.write() { *w = v; } }
pub fn g__A_PRICE_ASK() -> Py { ensure_init(); _A_PRICE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_PRICE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = _A_PRICE_ASK.write() { *w = v; } }
pub fn g__A_RECORD() -> Py { ensure_init(); _A_RECORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_RECORD(v: Py) { ensure_init(); if let Ok(mut w) = _A_RECORD.write() { *w = v; } }
pub fn g__A_REPORTS() -> Py { ensure_init(); _A_REPORTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_REPORTS(v: Py) { ensure_init(); if let Ok(mut w) = _A_REPORTS.write() { *w = v; } }
pub fn g__A_REPORTS_THE_ASK() -> Py { ensure_init(); _A_REPORTS_THE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_REPORTS_THE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = _A_REPORTS_THE_ASK.write() { *w = v; } }
pub fn g__A_SAYS_NOW() -> Py { ensure_init(); _A_SAYS_NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_SAYS_NOW(v: Py) { ensure_init(); if let Ok(mut w) = _A_SAYS_NOW.write() { *w = v; } }
pub fn g__A_SERVICE() -> Py { ensure_init(); _A_SERVICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_SERVICE(v: Py) { ensure_init(); if let Ok(mut w) = _A_SERVICE.write() { *w = v; } }
pub fn g__A_SERVICE_STATE() -> Py { ensure_init(); _A_SERVICE_STATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_SERVICE_STATE(v: Py) { ensure_init(); if let Ok(mut w) = _A_SERVICE_STATE.write() { *w = v; } }
pub fn g__A_THE_CLOCK() -> Py { ensure_init(); _A_THE_CLOCK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_THE_CLOCK(v: Py) { ensure_init(); if let Ok(mut w) = _A_THE_CLOCK.write() { *w = v; } }
pub fn g__A_WEATHER_HERE() -> Py { ensure_init(); _A_WEATHER_HERE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_WEATHER_HERE(v: Py) { ensure_init(); if let Ok(mut w) = _A_WEATHER_HERE.write() { *w = v; } }
pub fn g__A_WORD_MEANS() -> Py { ensure_init(); _A_WORD_MEANS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_WORD_MEANS(v: Py) { ensure_init(); if let Ok(mut w) = _A_WORD_MEANS.write() { *w = v; } }
pub fn g__A_WORKING() -> Py { ensure_init(); _A_WORKING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_WORKING(v: Py) { ensure_init(); if let Ok(mut w) = _A_WORKING.write() { *w = v; } }
pub fn g__BARE_LIVE_LEAD() -> Py { ensure_init(); _BARE_LIVE_LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BARE_LIVE_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = _BARE_LIVE_LEAD.write() { *w = v; } }
pub fn g__CHANGING() -> Py { ensure_init(); _CHANGING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CHANGING(v: Py) { ensure_init(); if let Ok(mut w) = _CHANGING.write() { *w = v; } }
pub fn g__EVENT() -> Py { ensure_init(); _EVENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__EVENT(v: Py) { ensure_init(); if let Ok(mut w) = _EVENT.write() { *w = v; } }
pub fn g__HOURS() -> Py { ensure_init(); _HOURS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__HOURS(v: Py) { ensure_init(); if let Ok(mut w) = _HOURS.write() { *w = v; } }
pub fn g__I_SAID_IT() -> Py { ensure_init(); _I_SAID_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__I_SAID_IT(v: Py) { ensure_init(); if let Ok(mut w) = _I_SAID_IT.write() { *w = v; } }
pub fn g__MONEY() -> Py { ensure_init(); _MONEY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MONEY(v: Py) { ensure_init(); if let Ok(mut w) = _MONEY.write() { *w = v; } }
pub fn g__MONEY_ASKED() -> Py { ensure_init(); _MONEY_ASKED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MONEY_ASKED(v: Py) { ensure_init(); if let Ok(mut w) = _MONEY_ASKED.write() { *w = v; } }
pub fn g__MONEY_FOR() -> Py { ensure_init(); _MONEY_FOR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MONEY_FOR(v: Py) { ensure_init(); if let Ok(mut w) = _MONEY_FOR.write() { *w = v; } }
pub fn g__MONEY_NOUN() -> Py { ensure_init(); _MONEY_NOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MONEY_NOUN(v: Py) { ensure_init(); if let Ok(mut w) = _MONEY_NOUN.write() { *w = v; } }
pub fn g__NEWS() -> Py { ensure_init(); _NEWS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NEWS(v: Py) { ensure_init(); if let Ok(mut w) = _NEWS.write() { *w = v; } }
pub fn g__NIGHT_WISHES() -> Py { ensure_init(); _NIGHT_WISHES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NIGHT_WISHES(v: Py) { ensure_init(); if let Ok(mut w) = _NIGHT_WISHES.write() { *w = v; } }
pub fn g__NOT_A_FITTING_THAT_OPENS() -> Py { ensure_init(); _NOT_A_FITTING_THAT_OPENS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_A_FITTING_THAT_OPENS(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_A_FITTING_THAT_OPENS.write() { *w = v; } }
pub fn g__NOT_OPEN_TO_SOMEBODY() -> Py { ensure_init(); _NOT_OPEN_TO_SOMEBODY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_OPEN_TO_SOMEBODY(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_OPEN_TO_SOMEBODY.write() { *w = v; } }
pub fn g__RATE() -> Py { ensure_init(); _RATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__RATE(v: Py) { ensure_init(); if let Ok(mut w) = _RATE.write() { *w = v; } }
pub fn g__ROUTINE_SUBJECT() -> Py { ensure_init(); _ROUTINE_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ROUTINE_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = _ROUTINE_SUBJECT.write() { *w = v; } }
pub fn g__SAID_ON_ITS_OWN() -> Py { ensure_init(); _SAID_ON_ITS_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SAID_ON_ITS_OWN(v: Py) { ensure_init(); if let Ok(mut w) = _SAID_ON_ITS_OWN.write() { *w = v; } }
pub fn g__SEALED_ASKS() -> Py { ensure_init(); _SEALED_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SEALED_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = _SEALED_ASKS.write() { *w = v; } }
pub fn g__SERVICE_ON_A_DAY() -> Py { ensure_init(); _SERVICE_ON_A_DAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SERVICE_ON_A_DAY(v: Py) { ensure_init(); if let Ok(mut w) = _SERVICE_ON_A_DAY.write() { *w = v; } }
pub fn g__SETTLED() -> Py { ensure_init(); _SETTLED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SETTLED(v: Py) { ensure_init(); if let Ok(mut w) = _SETTLED.write() { *w = v; } }
pub fn g__SOUND_CUES() -> Py { ensure_init(); _SOUND_CUES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SOUND_CUES(v: Py) { ensure_init(); if let Ok(mut w) = _SOUND_CUES.write() { *w = v; } }
pub fn g__THANKS_WORDS() -> Py { ensure_init(); _THANKS_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THANKS_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = _THANKS_WORDS.write() { *w = v; } }
pub fn g__TIMETABLE() -> Py { ensure_init(); _TIMETABLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__TIMETABLE(v: Py) { ensure_init(); if let Ok(mut w) = _TIMETABLE.write() { *w = v; } }
pub fn g__UNITS() -> Py { ensure_init(); _UNITS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__UNITS(v: Py) { ensure_init(); if let Ok(mut w) = _UNITS.write() { *w = v; } }
pub fn g__UNITS_BOUGHT() -> Py { ensure_init(); _UNITS_BOUGHT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__UNITS_BOUGHT(v: Py) { ensure_init(); if let Ok(mut w) = _UNITS_BOUGHT.write() { *w = v; } }
pub fn g__WEATHER() -> Py { ensure_init(); _WEATHER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WEATHER(v: Py) { ensure_init(); if let Ok(mut w) = _WEATHER.write() { *w = v; } }
pub fn g__WISH_WORDS() -> Py { ensure_init(); _WISH_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WISH_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = _WISH_WORDS.write() { *w = v; } }
fn module_body() {
    let _ = py_str("What kind of line this is: the broad kind, the finer intent, and what it asks for.\n\n`kind` keeps the four values other lanes already read (empty, greeting, thanks, question,\nstatement). `intent` and `asks` say more, and every richer answer is a new key, so a module\nwritten against `kind` keeps working.\n\nThe classification is driven by small word sets, checked in order from the most specific\nreading to the most general, so adding a word to one set improves every line that uses it.\n");
    s_NEGATIONS(crate::gen::understanding::lexicon::g_NEGATIONS());
    s_PRONOUNS(call_builtin("frozenset", &[mcall(&py_str("i you we they he she it there that this these those one"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SUBJECT_NEGATIONS(call_builtin("frozenset", &[mcall(&py_str("no nobody none nothing neither nor"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GREETINGS(py_tuple(vec![py_str("hi"), py_str("hello"), py_str("hey"), py_str("hiya"), py_str("morning"), py_str("good morning"), py_str("good afternoon"), py_str("good evening"), py_str("evening"), py_str("yo"), py_str("howdy"), py_str("good day"), py_str("greetings"), py_str("salutations")]));
    s_FAREWELLS(py_tuple(vec![py_str("bye"), py_str("goodbye"), py_str("see you"), py_str("see ya"), py_str("later"), py_str("night"), py_str("good night"), py_str("talk later"), py_str("take care"), py_str("ttyl"), py_str("gtg"), py_str("cya"), py_str("best wishes"), py_str("best regards"), py_str("kind regards"), py_str("warm regards"), py_str("ttfn"), py_str("cheerio"), py_str("laters"), py_str("adios"), py_str("night night"), py_str("nighty night"), py_str("ta ta"), py_str("see you later"), py_str("see you soon"), py_str("talk to you later"), py_str("speak later"), py_str("speak to you later"), py_str("catch you later"), py_str("bye for now")]));
    s_THANKS(py_tuple(vec![py_str("thanks"), py_str("thank you"), py_str("thx"), py_str("cheers"), py_str("ty"), py_str("ta"), py_str("much appreciated"), py_str("appreciate"), py_str("appreciated"), py_str("appreciate it")]));
    s_QUESTION_WORDS(call_builtin("frozenset", &[mcall(&py_str("what why how when where who which whose"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_THANKS_ASK_NOUNS(call_builtin("frozenset", &[mcall(&py_str("advice tip tips idea ideas suggestion suggestions"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_THANKS_ASK_PREPS(call_builtin("frozenset", &[mcall(&py_str("for on about with"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_THANKS_ASK_WORDS(call_builtin("frozenset", &[mcall(&py_str("any need needs looking"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GIVEN_ADVICE(call_builtin("frozenset", &[mcall(&py_str("the your my our his her its their this that no"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_QUESTION_STARTS(py_set(vec![py_str("what"), py_str("why"), py_str("how"), py_str("when"), py_str("where"), py_str("who"), py_str("which"), py_str("whose"), py_str("can"), py_str("could"), py_str("should"), py_str("would"), py_str("is"), py_str("are"), py_str("do"), py_str("does"), py_str("did"), py_str("will"), py_str("have"), py_str("has"), py_str("am"), py_str("was"), py_str("were"), py_str("may"), py_str("might"), py_str("shall")]));
    s_YESNO_STARTS(sub(&g_QUESTION_STARTS(), &py_set(vec![py_str("what"), py_str("why"), py_str("how"), py_str("when"), py_str("where"), py_str("who"), py_str("which"), py_str("whose")])));
    s_AFFIRM(call_builtin("frozenset", &[mcall(&py_str("yes yeah yep yup yh yhh yhhh ye sure ok okay definitely absolutely correct right probably maybe perhaps likely true exactly indeed agreed fine cool great nice good perfect sounds works deal done sweet brilliant"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DENY(call_builtin("frozenset", &[mcall(&py_str("no nope nah never not nothing none neither"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ACK(call_builtin("frozenset", &[mcall(&py_str("ok okay cool nice great good fine sure right got understand understood thanks aha ah oh wow really hmm hm mmm alright k gotcha you me same maybe perhaps probably likely haha hehe lol fair true"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_LAUGH(call_builtin("frozenset", &[crate::gen::understanding::lexicon::g_LAUGHTER()]).unwrap_or_else(|e| exc::raise(e)));
    s_POLITE(call_builtin("frozenset", &[mcall(&py_str("please pls plz ta mate"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_RECOMMEND(call_builtin("frozenset", &[mcall(&py_str("recommend suggest recommendation suggestion advice idea ideas options option pick choose best better worth"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TIPS(stdlib_call("re", "compile", &[py_str("\\b(tips|hints|pointers|tip|hint|pointer)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TIPS_WANTED(stdlib_call("re", "compile", &[py_str("\\b(?:any|some|a few|few|got|need|needs|wanted|looking for|share|send|give(?: me)?)\\s+(?:\\w+\\s+){0,2}$")]).unwrap_or_else(|e| exc::raise(e)));
    s_TIPS_ABOUT(stdlib_call("re", "compile", &[py_str("\\s+(?:for|on|about|when|to)\\s+\\w")]).unwrap_or_else(|e| exc::raise(e)));
    s_TIPS_SUBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:i|you|we|they|he|she|it|who|that|this|people|everyone|nobody|someone)\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_TIPS_AMOUNT(stdlib_call("re", "compile", &[py_str("(?:\\d|%|\\b(?:a|the)\\s+\\w+\\s*%)\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_HELP_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:anything|something)\\s+(?:else\\s+)?(?:i|we|you)\\s+(?:can|could|should|might|would)\\s+(?:do|try|use|take|add|change)\\b|\\b(?:anything|something)\\s+(?:else\\s+)?(?:that|which)\\s+(?:helps|would help|might help|will help|works)\\s*[?.!]*\\s*$|\\bany\\s+(?:way|ways)\\s+(?:of|to|round)\\b|\\bis there any\\s+(?:way|ways|thing)\\s+(?:of|to|i|we|you|else)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HELP_ASK_SUBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:is|was|are|were|seems|seemed|feels|felt|sounds|sounded|helps|helped|works|worked|suits|suited|fits|fitted|does|did|its|thats|im|youre|(?:would|will|might|must|could|can|may) be)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HELP_ASK_TAIL_CLAUSE(stdlib_call("re", "compile", &[py_str("\\|\\s*\\w+\\s+\\w+")]).unwrap_or_else(|e| exc::raise(e)));
    s_HELP_ASK_NOT(stdlib_call("re", "compile", &[py_str("\\b(?:no|not|isnt|isn't|wasnt|wasn't|arent|aren't|cant|can't|couldnt|couldn't|dont|don't|doesnt|doesn't|didnt|didn't|never|nothing|without|hardly)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HELP_ASK_OWN_SUBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|his|her|their|your|the|a|an)\\s+[a-z]")]).unwrap_or_else(|e| exc::raise(e)));
    s_VERDICT_ASK(stdlib_call("re", "compile", &[py_str("(?:and |so |but |ok |okay |also |sorry |oh |hmm )*(?:(?:is|was|are|were|does|do|did) (?:that|this|it)(?: all)?(?: (?:really|actually|very|too|that|so))?(?: (?:bad|normal|serious|dangerous|ok|okay|fine|safe|common|a|much of a) ?(?:problem|worry|issue|big deal)?| (?:going to|gonna) (?:get|be) worse| (?:going to|gonna) (?:spread|get worse))|(?:is|was) (?:that|this|it)(?: (?:going to|gonna))? (?:get worse|getting worse|spread|serious)|(?:is|was) (?:he|she|they) right|(?:are|were|is|was) (?:they|these|those) (?:safe|ok|okay|fine|normal|bad|a problem|a risk|any good)|(?:should|do|ought) i (?:be )?(?:worried|worry|worrying|bother)|does (?:that|this|it) matter|will (?:that|this|it) (?:get|be) (?:worse|ok|okay|fine))(?: then| though| honestly)?")]).unwrap_or_else(|e| exc::raise(e)));
    s_QUESTION_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:why|when|where|how|who|whom|whose|which|what)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LIMIT_SHAPES(py_tuple(vec![py_tuple(vec![py_str("no"), stdlib_call("re", "compile", &[py_str("\\bno\\s+(?:more\\s+)?([a-z][a-z' -]*?)\\s*$")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("nothing"), stdlib_call("re", "compile", &[py_str("\\bnothing\\s+([a-z][a-z' -]*?)\\s*$")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("not"), stdlib_call("re", "compile", &[py_str("\\bnot\\s+(?:too|very|that|so|any)\\s+([a-z][a-z' -]*?)\\s*$")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("not_the"), stdlib_call("re", "compile", &[py_str("\\bnot\\s+the\\s+([a-z][a-z' -]*?)\\s+ones?\\s*$")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("without"), stdlib_call("re", "compile", &[py_str("\\bwithout\\s+(?:the\\s+|any\\s+)?([a-z][a-z' -]*?)\\s*$")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("skip"), stdlib_call("re", "compile", &[py_str("\\b(?:skip|hold|drop|minus|leave out)\\s+(?:the\\s+|any\\s+)?([a-z][a-z' -]*?)\\s*$")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("dont"), stdlib_call("re", "compile", &[py_str("\\bi\\s+(?:dont|do not|can't|cant|can not)\\s+(?:eat|drink|like|want|have)\\s+([a-z][a-z' -]*?)\\s*$")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("allergic"), stdlib_call("re", "compile", &[py_str("\\bi\\s+(?:am|m)\\s+allergic\\s+to\\s+([a-z][a-z' -]*?)\\s*$")]).unwrap_or_else(|e| exc::raise(e))])]));
    s_LIMIT_SOFT_BEFORE(stdlib_call("re", "compile", &[py_str("^(?:ok|okay|so|and|but|well|right|then|also|honestly|hmm|oh|ah|actually|btw)\\b[\\s,]*")]).unwrap_or_else(|e| exc::raise(e)));
    s_LIMIT_SOFT_AFTER(stdlib_call("re", "compile", &[py_str("[\\s,]*(?:please|pls|thanks|thank you|then|though|tbh|honestly|lol|for me|this time|at all|i guess|i suppose|to be honest|as well|instead)\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_LIMIT_ROOM(Py::Int(3));
    s_LIMIT_REFUSAL(call_builtin("frozenset", &[mcall(&py_str("idea clue way sure bad good luck doubt joke point matter rush hurry\nwonder worries worry problem bother chance need thanks ta worries offence offense kidding\nquestion answer option options choice change pain help helps well right myself yourself self\nleft yet though either sorry wait enough much many more less quite finished still now bake\nsince until after before"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_LIMIT_A_FACT(call_builtin("frozenset", &[mcall(&py_str("january february march april may june july august september october\nnovember december monday tuesday wednesday thursday friday saturday sunday today tomorrow\nyesterday morning afternoon evening night week weekend month year\none two three four five six seven eight nine ten eleven twelve twenty thirty forty fifty\nhundred thousand couple few half lot"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_LIMIT_NOT_A_START(call_builtin("frozenset", &[mcall(&py_str("with without about for from by like not probably maybe wait sorry\nenough too very much many more less just only at all and or but so then also"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST_ENDS(stdlib_call("re", "compile", &[py_str("^(?:[a-z]{3,}(?:ed|ought|aught)|went|got|came|said|saw|made|took|gave|found|lost|left|ran|broke|fell|kept|met|paid|put|sent|sat|stood|thought|told|won|wrote|bought|brought|caught|chose|drove|flew|forgot|heard|held|hit|knew|ate|drank|slept|woke|bit|spoke|spent|built|burnt|burned|grew|threw|been|being)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_NOT_A_CLAIM(call_builtin("frozenset", &[mcall(&py_str("works work helps help changes happens matters seems makes does goes\ncomes gets stops starts fixes fits runs moves shows feels looks sounds costs needs stays keeps turns\ngrows hurts adds counts means bothers lives takes gives brings appeals excites suits\nsatisfies interests improves settles lasts ends agrees sticks sorts arrives\n# The irregular pasts, which no ending rule reaches: \"nothing got sorted\", \"nothing came of it\",\n# \"nothing went right\", \"nothing did\". Each of those was answered with the answer described back\n# at the person - \"Nothing there was about got sorted, so the answer holds\" - so the verb is\n# listed in the tense it is said in as well as the one the list starts in.\ngot came went did was were"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CLAIM_ENDS(py_tuple(vec![py_tuple(vec![Py::Int(3), py_str("ing")]), py_tuple(vec![Py::Int(1), py_str("s")]), py_tuple(vec![Py::Int(2), py_str("ed")]), py_tuple(vec![Py::Int(1), py_str("d")])]));
    s_CLAIM_STEMS(call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_A_THING_NOT_A_CLAIM()) { let mut form: Py = __item.clone(); __out.push(if py_truthy(&{ let __b = mcall(&form.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[form.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) { slice_or_raise(&form.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None) } else { form.clone() }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_LIMIT_NOT_A_THING(call_builtin("frozenset", &[mcall(&py_str("i me my we you he she they it them us her him his hers its our\nours your yours their theirs mine whom is are was were be been\nam do does did have has had can could would should will just really very too so much any some\nthe a an of to for in on at and or but if that this these those there here what when where who\nsomething anything everything nothing other else matter way bit lot lots one ones thing things\nto"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_RULED_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:not\\s+(?:a|an|the)|rather than\\s+(?:a|an|the)?|instead of\\s+(?:a|an|the)?)\\s+([a-z][a-z'’-]*(?:\\s+[a-z][a-z'’-]*){0,2})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RULED_OUT_ENDS(call_builtin("frozenset", &[mcall(&py_str("and but or so because if when then please though although actually\nreally either too now tonight today anyway for with in on at to of is are was were be been do does\ndid have has had can could would should will"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_RULED_OUT_NO_THING(call_builtin("frozenset", &[mcall(&py_str("what about recommend recommends suggest show give find is are was\nwere do does did can could would should will you u ur me my i we us the a an of to for in on at\nand but or so if when then there here this that these those it its be been have has had any some\nplease thanks ok okays just also too more most than rather instead not no yes something anything\nanything"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_CLASS_OF_THING(py_list(vec![]));
    s_RULED_OUT_CORRECTION(stdlib_call("re", "compile", &[py_str("\\b(?:actually|i mean|sorry|make that|on second thoughts?|rather|instead|no,|nope)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECOMMEND_PHRASES(py_tuple(vec![py_str("any good"), py_str("something good"), py_str("anything good"), py_str("worth watching"), py_str("worth reading"), py_str("what should i watch"), py_str("what should i read"), py_str("any ideas"), py_str("what would you recommend"), py_str("what do you recommend"), py_str("help me choose"), py_str("help me pick"), py_str("any suggestions")]));
    s_OPINION(py_tuple(vec![py_str("what do you think"), py_str("what about you"), py_str("your favourite"), py_str("your favorite"), py_str("do you like"), py_str("do you prefer"), py_str("would you rather"), py_str("in your opinion"), py_str("what would you"), py_str("how about you")]));
    s_CHOICE(py_tuple(vec![py_str(" or "), py_str("better than"), py_str("instead of"), py_str("vs"), py_str("versus")]));
    s_OPPOSITE_PAIRS(py_tuple(vec![py_tuple(vec![py_str("early"), py_str("late")]), py_tuple(vec![py_str("earlier"), py_str("later")]), py_tuple(vec![py_str("sooner"), py_str("later")]), py_tuple(vec![py_str("soon"), py_str("late")]), py_tuple(vec![py_str("hot"), py_str("cold")]), py_tuple(vec![py_str("warm"), py_str("cold")]), py_tuple(vec![py_str("big"), py_str("small")]), py_tuple(vec![py_str("large"), py_str("small")]), py_tuple(vec![py_str("long"), py_str("short")]), py_tuple(vec![py_str("high"), py_str("low")]), py_tuple(vec![py_str("good"), py_str("bad")]), py_tuple(vec![py_str("better"), py_str("worse")]), py_tuple(vec![py_str("wet"), py_str("dry")]), py_tuple(vec![py_str("sunny"), py_str("rainy")]), py_tuple(vec![py_str("fast"), py_str("slow")]), py_tuple(vec![py_str("quick"), py_str("slow")]), py_tuple(vec![py_str("cheap"), py_str("expensive")]), py_tuple(vec![py_str("busy"), py_str("quiet")]), py_tuple(vec![py_str("loud"), py_str("quiet")]), py_tuple(vec![py_str("full"), py_str("empty")]), py_tuple(vec![py_str("deep"), py_str("shallow")]), py_tuple(vec![py_str("heavy"), py_str("light")]), py_tuple(vec![py_str("thick"), py_str("thin")]), py_tuple(vec![py_str("wide"), py_str("narrow")]), py_tuple(vec![py_str("hard"), py_str("soft")]), py_tuple(vec![py_str("early"), py_str("on time")]), py_tuple(vec![py_str("in"), py_str("out")])]));
    s_SCALE_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:is|are|was|were|will|would|should|when|what|how|which|be)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_REFUSED_PAIR(stdlib_call("re", "compile", &[py_str("^\\s*(?:no|not|never|without|neither)\\b|\\b(?:dont|don'?t|do not|doesnt|doesn'?t|didnt|didn'?t|wont|won'?t|wouldnt|wouldn'?t|cant|can'?t|cannot|shouldnt|shouldn'?t)\\s+(?:\\w+\\s+){0,2}?(?:want|wants|wanted|like|likes|love|eat|eating|have|having|drink|drinking|wear|do|doing|need|needs|use|fancy|stand|bother|manage)\\b|\\ballergic\\b|\\bhate\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_COMPARE(py_tuple(vec![py_str("difference between"), py_str("different from"), py_str("compare"), py_str("which one"), py_str("or is it")]));
    s_REQUEST(call_builtin("frozenset", &[mcall(&py_str("help need want trying wanna please tell show explain give write make how"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STEPS(py_tuple(vec![py_str("how do i"), py_str("how can i"), py_str("how should i"), py_str("what should i"), py_str("step by step"), py_str("walk me through"), py_str("get started"), py_str("how to")]));
    s_STEPS_WORD(stdlib_call("re", "compile", &[py_str("\\bsteps\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WRITE(py_tuple(vec![py_str("write"), py_str("word"), py_str("phrase"), py_str("say"), py_str("message"), py_str("text to"), py_str("email to"), py_str("letter"), py_str("poem"), py_str("speech"), py_str("bio"), py_str("caption"), py_str("wish")]));
    s_A_COMPARISON(call_builtin("frozenset", &[py_tuple(vec![py_str("worse")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SELF_ONLY_CUES(py_tuple(vec![py_str("empty"), py_str("flat"), py_str("hollow"), py_str("blank"), py_str("numb"), py_str("rubbish"), py_str("crap"), py_str("rotten"), py_str("rough"), py_str("lousy"), py_str("like a failure"), py_str("like a mess"), py_str("like rubbish"), py_str("like crap"), py_str("like a burden"), py_str("like a fraud"), py_str("a mess"), py_str("in a bad mood"), py_str("in a low mood"), py_str("in a foul mood"), py_str("in a mood"), py_str("in a state"), py_str("in a right state"), py_str("out of sorts"), py_str("off colour"), py_str("off form")]));
    s_CONFUSED_SELF_ONLY(call_builtin("frozenset", &[py_tuple(vec![py_str("confused"), py_str("confusing"), py_str("confusion"), py_str("muddled"), py_str("baffled"), py_str("bewildered"), py_str("puzzled")])]).unwrap_or_else(|e| exc::raise(e)));
    s_LABEL_ON_A_PERSON(call_builtin("frozenset", &[py_tuple(vec![py_str("boring"), py_str("annoying"), py_str("tiring"), py_str("exhausting"), py_str("draining"), py_str("knackering"), py_str("frustrating"), py_str("depressing"), py_str("embarrassing")])]).unwrap_or_else(|e| exc::raise(e)));
    s_THING_OR_PERSON_CUES(call_builtin("frozenset", &[py_tuple(vec![py_str("worn thin"), py_str("worn to a thread"), py_str("at my limit"), py_str("at breaking point"), py_str("at my wits end"), py_str("at my wits' end"), py_str("beside myself"), py_str("at the end of my patience"), py_str("reaching the end of my rope"), py_str("worrying"), py_str("stressing"), py_str("stressed out"), py_str("overthinking"), py_str("obsessing"), py_str("spiralling"), py_str("spiraling"), py_str("worn down")])]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_PERIOD_NOUNS(call_builtin("frozenset", &[mcall(&py_str("day days week weeks month months year years weekend weekends\nnight nights morning mornings afternoon afternoons evening evenings shift shifts term"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_PERIOD_FILLER(call_builtin("frozenset", &[mcall(&py_str("a an the one few couple of real proper whole entire bit those\nthese sort kind old long bad"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_PERIOD_HEADS(call_builtin("frozenset", &[mcall(&py_str("i we i've ive me today yesterday tonight"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_PERIOD_STOPS(call_builtin("frozenset", &[mcall(&py_str("and but because so though while if when than then that which\nwho whose after before since until"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THE_SPEAKERS_DAY(call_builtin("frozenset", &[py_tuple(vec![py_str("mad"), py_str("flat"), py_str("dead"), py_str("mental"), py_str("wild"), py_str("slow"), py_str("heavy")])]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_PERIOD_REACH(Py::Int(6));
    s_PERIOD_CUES(call_builtin("frozenset", &[py_tuple(vec![py_str("at my limit")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERIOD(stdlib_call("re", "compile", &[py_str("\\b(?:every|each|this|next|last|a|per)\\s+(?:(?:single|whole|full|other|coming|past|few|one|more)\\s+)*(?:month|week|fortnight|pay ?day|paycheck|wage)\\b|\\bmonthly\\b|\\bweekly\\b|\\bfortnightly\\b|\\bpay ?day\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MONTH_PERIOD(stdlib_call("re", "compile", &[py_str("\\b(?:month|monthly)\\b|\\bpay ?day\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MONEY_SUBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:rent|mortgage|bills?|electric(?:ity)?|gas|water|council tax|overdraft|credit card|loan|debt|debts|childcare|nursery fees|fees|fuel|heating)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LIMIT_ABOUT_PEOPLE(stdlib_call("re", "compile", &[py_str("\\b(?:with (?:the|my|our|their|his|her|these|those)\\s+(?:kids?|children|child|family|mum|mom|dad|parents|partner|husband|wife|boss|team)|at work|at school|at college)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LAUGHED_OFF_AFTER(stdlib_call("re", "compile", &[py_str("(?:with\\s+|in\\s+)?(?:laughter|laughing|laughs|cracking up|stitches)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SAD_PLAIN_CUES(py_tuple(vec![py_str("dead inside"), py_str("feel nothing"), py_str("like nothing matters"), py_str("not myself"), py_str("not feeling myself"), py_str("not been myself"), py_str("do not feel myself"), py_str("do not feel like myself"), py_str("have not felt myself"), py_str("have not been myself")]));
    s_SELF_ONLY_STATE(call_builtin("frozenset", &[mcall(&py_str("low awful hopeless worthless shattered wiped wasted spent tense pooped whacked zonked blue raging fuming seething incandescent"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_STATE_CUES(call_builtin("frozenset", &[py_tuple(vec![py_str("upset"), py_str("low"), py_str("awful"), py_str("rough"), py_str("rotten")])]).unwrap_or_else(|e| exc::raise(e)));
    s_IDIOM_STATES(stdlib_call("re", "compile", &[py_str("\\bflat\\s+(?:out|broke)\\b|\\bflat as (?:a|an) \\w+|\\bcheap\\s+and\\s+cheerful\\b|\\bin (?:a|the) mood for\\b|\\b(?:bit of a|a bit of a|a right|a proper)\\s+(?:mad|weird|strange|funny|odd|daft|curious)\\s+one\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HEART_DEGREES(py_str("(?:still|really|so|very|totally|completely|utterly|absolutely|just|all|now|quite|too|already|a bit|kind of|sort of)"));
    s_HEART_IN_A_STATE(stdlib_call("re", "compile", &[add(&add(&py_str("\\bheart(?:s|'s)?\\b(?:\\s+(?:is|was|feels|felt|seems|has|had|been))*(?:\\s+"), &g_HEART_DEGREES()), &py_str(")*\\s+(?:broken|shattered|torn|in pieces|aching|aches|breaks|heavy|breaking)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_BROKEN_HEARTED(stdlib_call("re", "compile", &[py_str("\\b(?:broken[\\s-]?hearted|heart[\\s-]?broken)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BROKEN_HEART(stdlib_call("re", "compile", &[py_str("\\b(?:a|an|my|our|her|his|their|your)\\s+broken\\s+heart\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BROKE_A_HEART(stdlib_call("re", "compile", &[py_str("\\b(?:broke|breaks|breaking|break)\\s+(?:my|our|her|his|their|your)\\s+heart\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_EMOTION_WORDS(py_dict(vec![(py_str("sad"), add(&add(&py_tuple(vec![py_str("sad"), py_str("unhappy"), py_str("miserable"), py_str("gutted"), py_str("heartbroken"), py_str("cry"), py_str("crying"), py_str("tears"), py_str("disappointed"), py_str("devastated"), py_str("weepy"), py_str("tearful"), py_str("homesick"), py_str("rotten"), py_str("lousy"), py_str("meh"), py_str("rough"), py_str("crap"), py_str("rubbish"), py_str("fed up"), py_str("numb"), py_str("blue"), py_str("at my wits end"), py_str("at my wits' end"), py_str("at my limit"), py_str("at breaking point"), py_str("at the end of my patience"), py_str("reaching the end of my rope"), py_str("glum"), py_str("mopey"), py_str("teary"), py_str("downhearted"), py_str("despondent"), py_str("blah"), py_str("bleh"), py_str("low"), py_str("upset"), py_str("awful"), py_str("hopeless"), py_str("worthless"), py_str("grieving"), py_str("mourning"), py_str("despair"), py_str("despairing"), py_str("in a dark place"), py_str("emotional")]), &g_SELF_ONLY_CUES()), &g_SAD_PLAIN_CUES())), (py_str("lonely"), py_tuple(vec![py_str("lonely"), py_str("alone"), py_str("isolated"), py_str("left out")])), (py_str("angry"), py_tuple(vec![py_str("angry"), py_str("annoyed"), py_str("furious"), py_str("cross"), py_str("irritated"), py_str("frustrated"), py_str("mad"), py_str("grumpy"), py_str("cheesed off"), py_str("cranky"), py_str("snappy"), py_str("tetchy"), py_str("ratty"), py_str("mardy"), py_str("livid"), py_str("seething"), py_str("fuming"), py_str("raging"), py_str("seeing red"), py_str("incandescent"), py_str("brassed off"), py_str("narked"), py_str("miffed"), py_str("in a strop")])), (py_str("anxious"), py_tuple(vec![py_str("anxious"), py_str("worried"), py_str("nervous"), py_str("scared"), py_str("afraid"), py_str("stressed"), py_str("stress"), py_str("panic"), py_str("panicking"), py_str("overwhelmed"), py_str("dreading"), py_str("bricking it"), py_str("bricking myself"), py_str("shitting myself"), py_str("crapping myself"), py_str("shitting it"), py_str("crapping it"), py_str("cacking myself"), py_str("cacking it"), py_str("shitting bricks"), py_str("butterflies"), py_str("on edge"), py_str("nervous wreck"), py_str("worried sick"), py_str("freaking out"), py_str("petrified"), py_str("anxiety"), py_str("terrified"), py_str("panicky"), py_str("in bits"), py_str("restless"), py_str("jittery"), py_str("antsy"), py_str("edgy"), py_str("tense"), py_str("apprehensive"), py_str("beside myself"), py_str("worrying"), py_str("stressing"), py_str("stressed out"), py_str("overthinking"), py_str("obsessing"), py_str("spiralling"), py_str("spiraling")])), (py_str("tired"), py_tuple(vec![py_str("tired"), py_str("exhausted"), py_str("knackered"), py_str("worn out"), py_str("drained"), py_str("no energy"), py_str("burned out"), py_str("burnt out"), py_str("shattered"), py_str("wiped out"), py_str("wiped"), py_str("zonked"), py_str("pooped"), py_str("whacked"), py_str("frazzled"), py_str("worn thin"), py_str("worn to a thread"), py_str("worn down"), py_str("fried"), py_str("think straight"), py_str("running on empty"), py_str("running on fumes"), py_str("head is mush"), py_str("heads mush"), py_str("keep my eyes open"), py_str("dead")])), (py_str("happy"), py_tuple(vec![py_str("happy"), py_str("glad"), py_str("delighted"), py_str("thrilled"), py_str("pleased"), py_str("excited"), py_str("proud"), py_str("relieved"), py_str("chuffed"), py_str("awesome"), py_str("hopeful"), py_str("cheerful")])), (py_str("grateful"), py_tuple(vec![py_str("grateful"), py_str("thankful"), py_str("appreciate")])), (py_str("bored"), py_tuple(vec![py_str("bored"), py_str("boring"), py_str("nothing to do")])), (py_str("confused"), py_tuple(vec![py_str("confused"), py_str("confusing"), py_str("confusion"), py_str("bewildered"), py_str("baffled"), py_str("muddled"), py_str("puzzled"), py_str("dont understand"), py_str("do not understand"), py_str("dont get it"), py_str("do not get it"), py_str("does not make sense"), py_str("makes no sense"), py_str("cant get my head round"), py_str("cant get my head around"), py_str("cant get my head straight"), py_str("cannot get my head round"), py_str("cannot get my head around"), py_str("can not get my head round"), py_str("can not get my head around"), py_str("cannot get my head straight"), py_str("can not get my head straight"), py_str("cant make head nor tail"), py_str("cannot make head nor tail"), py_str("can not make head nor tail"), py_str("cant make head or tail"), py_str("cannot make head or tail"), py_str("can not make head or tail"), py_str("struggling to understand"), py_str("struggling to follow")])), (py_str("unwell"), py_tuple(vec![py_str("hungover"), py_str("hung over"), py_str("hangover"), py_str("under the weather"), py_str("run down"), py_str("worse"), py_str("run-down"), py_str("jet lagged"), py_str("jet-lagged"), py_str("cream crackered"), py_str("crackered")])), (py_str("condition"), py_tuple(vec![py_str("bipolar"), py_str("schizophrenic"), py_str("anorexic"), py_str("bulimic"), py_str("narcoleptic"), py_str("psychotic"), py_str("infertile"), py_str("sterile"), py_str("paranoid"), py_str("transgender"), py_str("nonbinary"), py_str("hiv")]))]));
    s_ALWAYS_SELF_ONLY(call_builtin("frozenset", &[py_tuple(vec![py_str("dead"), py_str("fried"), py_str("running on empty"), py_str("running on fumes")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_AS_A_NEGATION(call_builtin("frozenset", &[py_tuple(vec![py_str("think straight"), py_str("keep my eyes open")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_MOOD_THERE(py_dict(vec![(py_str("dead"), stdlib_call("re", "compile", &[py_str("\\bdead\\s+(?:set|right|against|certain|last|heat|giveaway|cert|keen|to me|to the world)\\b|\\bso dead\\b")]).unwrap_or_else(|e| exc::raise(e))), (py_str("fried"), stdlib_call("re", "compile", &[py_str("\\bfried\\s+(?:rice|chicken|egg|eggs|bread|onion|potatoes|tomatoes|fish)\\b")]).unwrap_or_else(|e| exc::raise(e))), (py_str("think straight"), stdlib_call("re", "compile", &[py_str("\\b(?:he|she|they|my|his|her|their|its)\\b[^.|]{0,20}\\bthink straight\\b")]).unwrap_or_else(|e| exc::raise(e)))]));
    s_GOOD_EMOTION_NAMES(call_builtin("frozenset", &[py_tuple(vec![py_str("happy"), py_str("grateful")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NEGATIVE(call_builtin("frozenset", &[mcall(&py_str("sad unhappy miserable gutted heartbroken cry crying tears disappointed\ndevastated lonely alone angry annoyed furious frustrated mad anxious worried nervous scared\nafraid stressed panic overwhelmed dreading tired exhausted knackered drained sick ill hurt\npain ache sore worse worst hate awful terrible horrible bad fail failed failing lost lose\nhate rubbish useless stupid annoyed upset"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_POSITIVE(call_builtin("frozenset", &[mcall(&py_str("happy glad delighted thrilled pleased excited proud relieved grateful\nthankful love loved lovely great good nice brilliant wonderful perfect amazing fantastic\nfun funny enjoy enjoyed success succeeded won win calm relaxed fine okay"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TERMS_OF_ADDRESS(call_builtin("frozenset", &[mcall(&py_str("love mate pet hun honey sweetheart duck ducky flower sunshine\n    pal bud chick dear darling babe lass lad bella"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ADDRESSED_LINES(call_builtin("frozenset", &[mcall(&py_str("thanks thank cheers ta hello hi hiya hey morning afternoon\n    evening night bye goodbye yes no ok okay alright right please sorry"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OK_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("ok"), py_str("okay"), py_str("okey")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SAYS_HOW_THEY_ARE(call_builtin("frozenset", &[mcall(&py_str("alright alrighty grand sound chill content brill mint\n    sorted"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STATING_VERB(call_builtin("frozenset", &[mcall(&py_str("am is are was were feel feels feeling felt do does doing be been\n    seem seems look looking"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SPEAKER(call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("we")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NEWS(py_tuple(vec![py_str("i just"), py_str("i finally"), py_str("today i"), py_str("yesterday i"), py_str("we got"), py_str("we are getting"), py_str("i got"), py_str("i have just"), py_str("i have just"), py_str("guess what"), py_str("big news"), py_str("i start"), py_str("i started"), py_str("i finished"), py_str("i passed"), py_str("i failed"), py_str("i moved"), py_str("we moved")]));
    s_ABOUT_BOT(py_tuple(vec![py_str("are you a bot"), py_str("are you real"), py_str("are you human"), py_str("are you an ai"), py_str("are you alive"), py_str("are you ok"), py_str("do you have feelings"), py_str("your name"), py_str("do you remember"), py_str("can you remember"), py_str("how old are you"), py_str("what are you called"), py_str("are you a chatbot"), py_str("are you a person")]));
    s_ABOUT_BOT_END(py_tuple(vec![py_str("what are you"), py_str("who are you"), py_str("what are you exactly")]));
    s_ROBOT_WORDS(call_builtin("frozenset", &[mcall(&py_str("bot chatbot robot ai human person machine program real alive"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GENDER_WORDS(call_builtin("frozenset", &[mcall(&py_str("male female man woman men women boy girl"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TASTE_VERBS(call_builtin("frozenset", &[mcall(&py_str("like love prefer fancy enjoy hate dislike"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_LEARNED_ON(call_builtin("frozenset", &[mcall(&py_str("data internet"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASSISTANT_NAMES(call_builtin("frozenset", &[mcall(&py_str("claude chatgpt gpt gemini copilot siri alexa bard"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PRODUCT_ASK_WORDS(Py::Int(7));
    s_PRODUCT_ASK_TAIL(call_builtin("frozenset", &[mcall(&py_str("then really though now huh yeah or not at all literally exactly still please pls tho"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SELF_WORDS(call_builtin("frozenset", &[mcall(&py_str("you your yourself yours\nname called call\nmade make create created built build built trained train programmed program designed design\nwrote written\norigin creator\nhobbies hobby fun looking look looks\nplans plan\nbored busy tired awake\nfeel feelings dream dreams dreamt sleep eat\nreal alive human person bot robot ai chatgpt gpt claude gemini copilot siri alexa bard\nunderstand remember recall forget\nchat talking talk chats answering answer\ndoing get\nfrom tell\nheart soul souls emotion internet data male female man woman men women\nmodel models llm llms personality single married live learn learnt asleep\ncry laugh hungry drunk boy girl gender old young\nokay ok alright fine"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SELF_ASK(call_builtin("frozenset", &[mcall(&py_str("name called call made make create created built build trained train\nprogrammed program designed design wrote written origin creator hobbies hobby fun looking look\nlooks plans plan bored busy real alive human person bot robot ai chatgpt gpt claude gemini\ncopilot siri alexa bard understand\nremember recall forget get from tell heart soul souls emotion internet data\nmale female man woman men women personality single married live sleep eat dream dreams\nasleep cry laugh hungry drunk boy girl gender old young"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SELF_FILLER(call_builtin("frozenset", &[mcall(&py_str("what why how who where when which whose is are was were am do does\ndid can could will would shall should have has had a an the any some your you yourself yours\ni me my at from about like of for to with now then today tonight ever actually really even\nstill just so much very both all or either on in by using up and so then ok okay"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_AFTER_THE_BOT(call_builtin("frozenset", &[mcall(&py_str("you youre your u ur ya are is this it thats that"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_BEFORE_THE_BOT(call_builtin("frozenset", &[mcall(&py_str("you u ur ya your yourself yours"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_AUX(call_builtin("frozenset", &[mcall(&py_str("can could will would do does did are is was were am have has had\n    should shall"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CLAUSE_OPENERS(call_builtin("frozenset", &[mcall(&py_str("when whenever if once after before until till while since\n    as because unless though although"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CAPABILITY(py_tuple(vec![py_str("what can you do"), py_str("what do you do"), py_str("how do you work"), py_str("what can you help"), py_str("how can you help"), py_str("how do you help"), py_str("what can i ask you"), py_str("what are you good at"), py_str("what are you for"), py_str("what kind of things can you help")]));
    s_CAPABILITY_FILLER(call_builtin("frozenset", &[mcall(&py_str("actually exactly really just still even then now here please mate pal though anyway so also else other and but oh yeah ok okay well um hmm"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CAPABILITY_CARRIER(call_builtin("frozenset", &[mcall(&py_str("\n    with for about me us at in on during here around then now any anything\n    a an the living this that these those next last another more some all\n    usually often\n    tonight today tomorrow later evening morning afternoon night week weekend\n    time times day days evening evenings night nights week weeks weekend weekends\n    month months year years hour hours minute minutes second seconds while bit\n    one two three four five ten fifteen twenty thirty sixty couple few half\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CORRECTION(py_tuple(vec![py_str("i meant"), py_str("i mean"), py_str("sorry i meant"), py_str("rather than"), py_str("not "), py_str("actually i"), py_str("correction"), py_str("i said"), py_str("no wait"), py_str("wait no"), py_str("oops i meant"), py_str("scratch that")]));
    s_GOING_ON_WHEN(call_builtin("frozenset", &[mcall(&py_str("tonight today tomorrow later week weeks weekend morning evening\n    night saturday sunday monday tuesday wednesday thursday friday now"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DOING_FILLER(call_builtin("frozenset", &[mcall(&py_str("what whats u you ya r are is doing do up to too much been have\n    has lately now today tonight then at the moment anything something nothing going on this\n    week new else still with watcha whatcha wotcha watchu whatchu doin mate pal buddy love\n    lovey hon hun then friend"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BEEN_WHEN(call_builtin("frozenset", &[mcall(&py_str("yesterday today last night nights morning mornings afternoon afternoon\n    evening evenings weekend weekends week weeks day days these all the at over on in this since\n    so far lately recently"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BEEN_SAID(call_builtin("frozenset", &[mcall(&py_str("did done been were was had doing do up where get got"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SPENT_TIME(call_builtin("frozenset", &[mcall(&py_str("all whole day days night nights week weeks lately recently since far\n    these hiding"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_COMING(call_builtin("frozenset", &[mcall(&py_str("tomorrow tonight later next"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SMALL_TALK(py_tuple(vec![py_str("how are you"), py_str("how is it going"), py_str("how's it going"), py_str("how is your day"), py_str("how's your day"), py_str("nice to meet you"), py_str("what's up"), py_str("how do you do"), py_str("lovely weather"), py_str("freezing"), py_str("raining"), py_str("sunny"), py_str("long week"), py_str("busy day")]));
    s_ASK_KEYS(py_tuple(vec![py_str("how_long"), py_str("how_much"), py_str("how_many"), py_str("how_often"), py_str("what"), py_str("why"), py_str("how"), py_str("when"), py_str("where"), py_str("who"), py_str("which"), py_str("yesno"), py_str("choice"), py_str("recommend"), py_str("steps"), py_str("opinion"), py_str("compare"), py_str("definition"), py_str("write"), py_str("maths"), py_str("list")]));
    s_SHORT_REPLIES(py_tuple(vec![py_str("not much"), py_str("nothing much"), py_str("same here"), py_str("me too"), py_str("you too"), py_str("no idea"), py_str("not really"), py_str("sort of"), py_str("kind of"), py_str("fair enough"), py_str("sounds good"), py_str("sounds great"), py_str("good idea"), py_str("will do"), py_str("i will"), py_str("i agree"), py_str("me neither"), py_str("makes sense"), py_str("got it"), py_str("for sure"), py_str("why not"), py_str("of course"), py_str("i guess"), py_str("that is fair"), py_str("no worries"), py_str("all good"), py_str("very funny"), py_str("so true"), py_str("not bad"), py_str("not a lot"), py_str("not a clue"), py_str("no bother"), py_str("i suppose"), py_str("guess so"), py_str("u too"), py_str("you as well"), py_str("same to you"), py_str("back at you"), py_str("hbu"), py_str("wbu"), py_str("nmu"), py_str("true that"), py_str("fair point"), py_str("good point"), py_str("my pleasure"), py_str("any time"), py_str("not really sure"), py_str("i think so"), py_str("hopefully"), py_str("probably"), py_str("definitely"), py_str("absolutely"), py_str("exactly"), py_str("agreed"), py_str("correct"), py_str("right"), py_str("true"), py_str("i dont know"), py_str("dont know"), py_str("i dunno"), py_str("dunno"), py_str("who knows"), py_str("no clue"), py_str("search me"), py_str("you know"), py_str("not sure"), py_str("no idea to be honest"), py_str("i do not know"), py_str("do not know"), py_str("not much"), py_str("how about you"), py_str("i know right"), py_str("you are welcome"), py_str("no problem"), py_str("same old"), py_str("same as always"), py_str("what about you"), py_str("not too bad")]));
    s_AGREEMENT_IDIOMS(py_tuple(vec![py_str("tell me about it"), py_str("you are telling me"), py_str("do not i know it"), py_str("you can say that again"), py_str("you are not wrong"), py_str("preaching to the choir"), py_str("you and me both"), py_str("i know right"), py_str("i hear that"), py_str("amen to that")]));
    s_EVENT_WORDS(call_builtin("frozenset", &[mcall(&py_str("asked told said got went started finished had was were just\ntoday yesterday tomorrow week weekend month year moved passed failed lost found bought sold\npaid booked applied interview job trip visit party birthday meeting doctor dentist school\nwork flat house car dog cat baby wedding holiday"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MATH_WORDS(call_builtin("frozenset", &[mcall(&py_str("plus minus times divide divided percent percentage half double triple sum total average calculate equals"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DEFINITION(py_tuple(vec![py_str("what is a"), py_str("what is an"), py_str("what are"), py_str("what does"), py_str("difference between"), py_str("meaning of"), py_str("explain")]));
    s_ASKS_ABOUT_A_PERSON(stdlib_call("re", "compile", &[py_str("\\bwhat (?:is|are|r) (?:you|u|ya|we|us|they)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_DEFINITION(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|but|so|then|ok|okay|right|oh|well|now|sorry|hi|hello|please)\\s+)*what(?:'s|s| is| are| was)?\\s+(?:is|are|was)\\s+([a-z][a-z'\\u2019-]{1,28})(?![^?]*\\b(?:like|for|worth|good|cheap|cheapest|easier|easiest|better|best|worse|worst|date|time|day|weather|price|cost|news|forecast|temperature|showing|playing|opening|score|length|size)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_DEFINITION_HEAD(call_builtin("frozenset", &[mcall(&py_str("a an the my your our his her their its this that these\nthose it i you we they he she there on in up out off about wrong going happening happened next\nnew best better missing too more most so all any some not just for to with from of at by under\nover behind inside between near against than left coming showing playing occurring eating cooking\nred one none something anything nothing everything someone anyone everyone two three four five\nsix seven eight nine ten half quarter third quarter percent percent cheapest easiest quickest\nlongest shortest biggest smallest largest worth nice easy good double triple eleven twelve thirteen fourteen fifteen sixteen seventeen\neighteen nineteen twenty thirty forty\nfifty sixty seventy eighty ninety hundred thousand million"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SMALL_TALK_WORDS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SMALL_TALK()) { let mut phrase: Py = __item.clone(); __out.push(call_builtin("tuple", &[mcall(&mcall(&mcall(&phrase.clone(), "replace", &[py_str("'s"), py_str(" is")]), "replace", &[py_str("'"), py_str("")]), "split", &[])]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_ADDRESS_WORDS(call_builtin("frozenset", &[mcall(&py_str("love hun hunny hon dear darling duck pet chuck sweetheart sweetie\n    son sunshine mate mates friend friends guys folks"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SMALL_TALK_LEFTOVER(bitor(&bitor(&call_builtin("frozenset", &[mcall(&py_str("\na an the for of in on out up there here it its is are was were be been am has\nand or so but then now again today tonight tomorrow lately already still very really too\nquite just all right well good fine bad ok okay not no thanks thank you thx ta cheers ty\nu ya mate mates friend friends guys folks everybody everyone hope your way doing going\ngetting had have last this week month day morning evening night long busy how what about\nwith us we i around yet even ever only mostly to at by from much appreciated appreciate\nmy your our their both all two\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_ADDRESS_WORDS()), &call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_GREETINGS()) { let mut greeting: Py = __item.clone(); for __item in py_iter_or_raise(&mcall(&greeting.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    s_CUE_FOLLOW(call_builtin("frozenset", &[mcall(&py_str("at with about of by from for in on into onto upon out up down off\naway back over under than if as while since during after before until though although unless\naround through toward towards even again too also just still yet ever never always sometimes\noften usually mostly all any some every each both either neither most much more less least\nenough quite pretty really very so that when because and but or now today yesterday tonight\nthis these those tbh tho atm rn lol haha innit actually\ninside outside deep down lately\ntomorrow honestly though still lately recently afterwards anyway here there one\ni me my mine myself you your yours yourself he him his she her hers it its itself we us our\nours ourselves they them their theirs themselves who whom whose what why how where which\nwhether is are was were am be been being will would shall should can could may might must\nhave has had having do does did doing done dont didnt doesnt wont cant im ive ill id youre\ntheyre thats its gonna going gotta\nto as please pls thanks thank cheers sorry sick stiff silly rotten\nmate man bro dude guys folks everyone everybody buddy pal bud hon hun love sir madam team"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PHRASE_OPENERS(call_builtin("frozenset", &[mcall(&py_str("the a an this that these those my your his her its our their some\nany no every each another one"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CLAUSE_VERBS(call_builtin("frozenset", &[mcall(&py_str("went got came said saw made took gave found lost left ran broke fell\nkept met paid put sent sat stood thought told won wrote bought brought caught chose drove flew\nforgot heard held hit knew ate drank slept woke bit spoke spent built burnt burned grew threw"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CLAUSE_HELPERS(call_builtin("frozenset", &[mcall(&py_str("is are was were am be been being will would shall should can\ncould may might must have has had having do does did doing not never just still already yet"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STATE_NOUNS(call_builtin("frozenset", &[mcall(&py_str("mood state spirits temper frame mind head shape form headspace\nmindset humour humor moodstate"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STATE_TAIL(stdlib_call("re", "compile", &[py_str("\\b(?:in|of)\\s+(?:a|an|such\\s+a|a\\s+right|a\\s+real|a\\s+proper|the|this|that)\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_SPEAKER_TAIL(stdlib_call("re", "compile", &[py_str("\\bi (?:am|was|feel|felt|have been|had been|do feel|did feel|am feeling|was feeling|been feeling|have been feeling|had been feeling)$|\\bi(?: \\w+){1,3} (?:feel|feeling|felt)$|\\b(?:we|us) (?:are|were|feel|felt|have been|had been|do feel|did feel|are feeling|were feeling|been feeling)$|\\bwe(?: \\w+){1,3} (?:feel|feeling|felt)$|^were$")]).unwrap_or_else(|e| exc::raise(e)));
    s_ING_THINGS(call_builtin("frozenset", &[mcall(&py_str("morning evening ceiling building wedding pudding clothing shopping\n    offering meeting parking lighting heating bedding flooring roofing fencing bedding\n    thing something nothing anything everything king ring string spring wing sing bring swing\n    going being doing having making taking coming"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ING_TAKES_NO_OBJECT(call_builtin("frozenset", &[mcall(&py_str("\n    act acting appear appearing arrive arriving become becoming come coming fall falling feel\n    feeling get getting go going grow growing keep keeping lie lying live living look looking\n    remain remaining rest resting rise rising run running seem seeming sit sitting sleep sleeping\n    sound sounding stand standing stay staying turn turning wait waiting walk walking work\n    working"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("stomach tummy gut belly back leg legs arm arms head throat chest skin\neye eyes ear ears tooth teeth stomachs"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STATE_JOINERS(call_builtin("frozenset", &[mcall(&py_str("am is are was were be been being feel feels feeling felt still\nreally very so too pretty quite a bit little just now also always lately today tonight"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OBJECT_MARKERS(call_builtin("frozenset", &[mcall(&py_str("the a an my your his her their our this that these those\nanother every some any no"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME_MARKERS(call_builtin("frozenset", &[mcall(&py_str("this that these those every each"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TAIL_STOPS(call_builtin("frozenset", &[mcall(&py_str("after before since until when if and but so because though while\nthan then too also still now today tonight lately recently honestly about at in on of for to\nwith by from as like"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_VERB_ONLY_STATE(call_builtin("frozenset", &[mcall(&py_str("pooped"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_COPULA_BEFORE(call_builtin("frozenset", &[mcall(&py_str("im am is are was were be been being feel feels felt feeling\nget gets got gettin look looks looked seem seems seemed sound sounds sounded"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ABILITY_AT(stdlib_call("re", "compile", &[py_str("\\b(?:bad|crap|rubbish|rough|useless|hopeless|terrible|awful|lousy|poor|worse|worst)\\s+(?:at|with)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_EPISODE_NOUNS(call_builtin("frozenset", &[mcall(&py_str("attack attacks episode episodes spell spells"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STATE_AS_FEELING(call_builtin("frozenset", &[py_tuple(vec![py_str("down")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING_NOT_MOOD(call_builtin("frozenset", &[mcall(&py_str("for to on at"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING_FILLER(call_builtin("frozenset", &[mcall(&py_str("a an bit little lot kind kinda sort so too very really quite pretty\n    just still always again now today totally honestly also either all more most less as\n    enough well"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING_COPULA(call_builtin("frozenset", &[mcall(&py_str("am is are was were be been being feel feels feeling felt get gets\n    getting got seem seems seemed look looks looking sound sounds sounded"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING_CUES_BEFORE(bitor(&g_FEELING_COPULA(), &call_builtin("frozenset", &[mcall(&py_str("feel feels feeling felt"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_NOT_THEIR_OWN(call_builtin("frozenset", &[mcall(&py_str("her his their your mum mom mother dad father son daughter husband\n    wife partner friend sister brother child kid boss neighbour neighbour's nan gran"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_ELSE(call_builtin("frozenset", &[mcall(&py_str("mum mom mother dad father son daughter husband wife partner\n    friend sister brother child kid boss neighbour nan gran"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_IS_THE_SUBJECT(stdlib_call("re", "compile", &[py_str("^(?:he|she|they|him|them)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_POSSESSED_HEART(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|her|his|their|your|the|a|an)\\s+(?:\\w+'?s\\s+)?hearts?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HAVE_A_BROKEN_HEART(stdlib_call("re", "compile", &[py_str("\\b(?:i have|i've got|ive got|i got|i had)\\b[^.]{0,24}broken heart")]).unwrap_or_else(|e| exc::raise(e)));
    s_HEDGED_LOW(call_builtin("frozenset", &[py_tuple(vec![py_str("could be better"), py_str("could be better i suppose"), py_str("been better"), py_str("not so good"), py_str("not too good"), py_str("not so great"), py_str("not that good"), py_str("not that great"), py_str("so so")])]).unwrap_or_else(|e| exc::raise(e)));
    s_HEDGED_DOUBT(py_tuple(vec![py_str("i suppose"), py_str("suppose"), py_str("i guess"), py_str("guess"), py_str("ig"), py_str("i spose"), py_str("s'pose"), py_str("i gues")]));
    s_HEDGED_OK(call_builtin("frozenset", &[py_tuple(vec![py_str("ok"), py_str("okay"), py_str("fine"), py_str("alright"), py_str("all right"), py_str("good"), py_str("well"), py_str("grand"), py_str("not too shabby")])]).unwrap_or_else(|e| exc::raise(e)));
    s_HEDGED_WRAPPER(call_builtin("frozenset", &[mcall(&py_str("\n    im i am well yeh yeah yep oh ah eh hmm hm its it it's is honestly to be totally quite\n    thanks thank you ta tbh tbf lol ig i guess suppose today love mate and you you\n    honest really sure actually kinda sorta atm currently now very pretty\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WORK_ON_ME(py_dict(vec![(py_str("stresses"), py_str("anxious")), (py_str("stressing"), py_str("anxious")), (py_str("strains"), py_str("anxious")), (py_str("straining"), py_str("anxious")), (py_str("worries"), py_str("anxious")), (py_str("worrying"), py_str("anxious")), (py_str("worried"), py_str("anxious")), (py_str("frets"), py_str("anxious")), (py_str("fretting"), py_str("anxious")), (py_str("drains"), py_str("tired")), (py_str("draining"), py_str("tired")), (py_str("drained"), py_str("tired")), (py_str("tires"), py_str("tired")), (py_str("tiring"), py_str("tired")), (py_str("exhausts"), py_str("tired")), (py_str("exhausting"), py_str("tired")), (py_str("exhausted"), py_str("tired")), (py_str("wears"), py_str("tired")), (py_str("wearing"), py_str("tired")), (py_str("wore"), py_str("tired")), (py_str("worn"), py_str("tired")), (py_str("annoys"), py_str("angry")), (py_str("annoying"), py_str("angry")), (py_str("annoyed"), py_str("angry")), (py_str("irritates"), py_str("angry")), (py_str("irritating"), py_str("angry")), (py_str("irritated"), py_str("angry")), (py_str("frustrates"), py_str("angry")), (py_str("frustrating"), py_str("angry")), (py_str("frustrated"), py_str("angry")), (py_str("bugs"), py_str("angry")), (py_str("bugging"), py_str("angry")), (py_str("bugged"), py_str("angry")), (py_str("bothers"), py_str("angry")), (py_str("bothering"), py_str("angry")), (py_str("bothered"), py_str("angry")), (py_str("winds"), py_str("angry")), (py_str("winding"), py_str("angry")), (py_str("upsets"), py_str("sad")), (py_str("upsetting"), py_str("sad")), (py_str("depresses"), py_str("sad")), (py_str("depressing"), py_str("sad")), (py_str("depressed"), py_str("sad")), (py_str("scares"), py_str("anxious")), (py_str("scaring"), py_str("anxious")), (py_str("scared"), py_str("anxious")), (py_str("frightens"), py_str("anxious")), (py_str("frightening"), py_str("anxious")), (py_str("frightened"), py_str("anxious")), (py_str("confuses"), py_str("confused")), (py_str("confusing"), py_str("confused")), (py_str("confused"), py_str("confused"))]));
    s_ACT_ON_FILLER(call_builtin("frozenset", &[mcall(&py_str("so right really totally absolutely just always still all quite"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PARTS_OF_ME(bitor(&g_BODY_SUBJECT(), &call_builtin("frozenset", &[mcall(&py_str("knee knees shoulder shoulders hip hips neck ankle\n    ankles wrist wrists hand hands finger fingers toe toes elbow elbows foot feet jaw tooth\n    teeth sinuses earache toothache headache migraine backache"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_ACT_ON_LEAD(call_builtin("frozenset", &[mcall(&py_str("am is are was were be been being keep keeps kept get gets got\n    always every still just really totally so very all quite sometimes often daily"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__SOUND_CUES(Py::None);
    s_FEELING_SOUND_FRAME(stdlib_call("re", "compile", &[py_str("\\b(?:im|i am|i feel|i felt)\\s+(?!not\\b|never\\b|no\\b)(?:(?:so|really|very|totally|proper|dead|well|quite|pretty|slightly|kinda|kind of|a bit|a little|a tad)\\s+)*([a-z]{4,})\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SOUND_FOLLOWS(call_builtin("frozenset", &[mcall(&py_str("of at in with about over by again today tonight now already too\n    as anymore stiff out"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ACTS_ON_THE_SPEAKER(py_tuple(vec![py_tuple(vec![py_str("(?:stress|stresses|stressing|overwhelm|overwhelms|overwhelming|worry|worries|worrying|panic|panics|panicking)\\s+(?:me\\b|my head\\b)"), py_str("anxious")]), py_tuple(vec![py_str("(?:wear|wears|wearing|wore|tire|tires|tiring|exhaust|exhausts|exhausting|drain|drains|draining|kill|kills|killing|knacker|knackers|knackering)\\s+me\\b"), py_str("tired")]), py_tuple(vec![py_str("(?:get|gets|getting|got|bring|brings|bringing|brought)\\s+me\\s+(?:down|low)\\b"), py_str("sad")]), py_tuple(vec![py_str("(?:do|does|doing|done)\\s+my\\s+(?:head|brain|nut|swede)\\s+in\\b"), py_str("anxious")]), py_tuple(vec![py_str("(?:drive|drives|driving|drove)\\s+me\\s+(?:mad|crazy|nuts|mental|insane|up the wall|round the bend|round the twist)\\b"), py_str("angry")]), py_tuple(vec![py_str("(?:annoy|annoys|annoying|irritate|irritates|irritating|frustrate|frustrates|frustrating|bug|bugs|bugging|wind|winds|winding)\\s+me\\s+up\\b"), py_str("angry")])]));
    s_LAUGH_AT_THE_END(stdlib_call("re", "compile", &[py_str("\\b(?:lol|lmao|haha|hah|heh|jk|joking|kidding|honestly)\\b[\\s.!?]*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BODY_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("back neck knee knees feet foot leg legs arm arms hand hands shoulder shoulders hip hips ankle ankles wrist wrists elbow elbows joint joints muscle muscles tooth teeth jaw ear ears eye eyes head stomach chest ribs rib side spine sciatica period periods boob boobs tit tits throat shoulder-blade"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GETTING_WORSE(stdlib_call("re", "compile", &[py_str("\\b(i'?m|i am)\\s+getting\\s+(worse)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_BETTER(stdlib_call("re", "compile", &[py_str("\\bno\\s+better\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BETTER_OFF(stdlib_call("re", "compile", &[py_str("\\b(?:no\\s+)?better\\s+off\\b|\\bworse\\s+off\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GOOD_WORD_MISSING(stdlib_call("re", "compile", &[py_str("\\b(?:cant|can\\s*not|couldnt|could\\s*not|dont|do\\s*not|didnt|did\\s*not|havent|have\\s*not|hasnt|has\\s*not|hadnt|had\\s*not|never|not)\\b[^.?!;]{0,40}?\\b(?:remember|remembered|felt|feel|feeling|been|be|was|were)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GOOD_WORD_COMPARED(stdlib_call("re", "compile", &[py_str("\\b(?:this|so|that|as)\\s+(?:happy|glad|pleased|excited|proud|relieved|good|great|well|content|calm|fine)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GOOD_WORD_BY_COMPARISON(stdlib_call("re", "compile", &[py_str("\\b(?:better|best)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REFERRING(call_builtin("frozenset", &[mcall(&py_str("about for with to in on"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTED_AT(call_builtin("frozenset", &[mcall(&py_str("this that these those it them"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ME_ASKS(py_tuple(vec![py_str("what do you think of me"), py_str("what do you think about me"), py_str("what do you reckon of me"), py_str("what do you make of me"), py_str("do you like me"), py_str("do you love me"), py_str("do you care about me"), py_str("are you my friend"), py_str("are we friends"), py_str("am i your friend"), py_str("do you miss me"), py_str("would you miss me"), py_str("will you miss me"), py_str("do you hate me"), py_str("am i annoying you"), py_str("do you think about me"), py_str("do you think of me"), py_str("do you ever think about me"), py_str("are you annoyed with me"), py_str("do you like talking to me"), py_str("do you enjoy talking to me"), py_str("do you like chatting to me"), py_str("do you like me talking to you")]));
    s_ME_TAIL(py_tuple(vec![py_str("then"), py_str("though"), py_str("tho"), py_str("now"), py_str("honestly"), py_str("really"), py_str("lol"), py_str("haha"), py_str("tbh"), py_str("tbf"), py_str("ngl"), py_str("anyway"), py_str("still"), py_str("far"), py_str("so"), py_str("all"), py_str("at")]));
    s_ME_LEAD(py_tuple(vec![py_str("so"), py_str("ok"), py_str("okay"), py_str("oh"), py_str("yeah"), py_str("but"), py_str("well"), py_str("and"), py_str("honestly")]));
    s_WHO_TALKING(call_builtin("frozenset", &[mcall(&py_str("talk talking speak speaking chat chatting message messaging\n    text texting"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHO_A_THING(call_builtin("frozenset", &[mcall(&py_str("bot robot chatbot ai machine computer program human person"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHO_TAIL(call_builtin("frozenset", &[mcall(&py_str("then please pls anyway again"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHERE_BOT_TAIL(call_builtin("frozenset", &[mcall(&py_str("from based located at out of exactly then now though really actually\n    please pls mate live living staying originally today currently"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ANSWERING_A_QUOTED_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:how\\s+(?:do|should|would|can|might|to)\\s*(?:i|you|we)?|how\\s+to|what\\s+(?:do|should)\\s*(?:i|you|we)?)\\s*[^.?!]{0,20}?\\b(?:answer|say|reply|respond)\\b[^.?!]{0,40}?\\b(?:what|who|where|why|how|when|tell\\s+me|do\\s+you|are\\s+you|can\\s+you|your)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHILE_ASK(stdlib_call("re", "compile", &[py_str("\\bhow\\s+long\\s+(?:can|could|may|might|will|would|do|does)\\s+(?:we|i|you|u|this)\\s+(?:[a-z]+\\s+){0,2}?(?:talk|chat|keep\\s+(?:going|talking|chatting)|carry\\s+on|go\\s+on)\\b|\\bhow\\s+long\\s+(?:have|has|do|does|did)\\s+(?:you|u)\\s+(?:got|have)\\b(?=\\s*(?:$|[.!?]|for\\s+(?:you|me|us|now|today)\\b|on\\s+you\\b|today\\b|now\\b|then\\b|and\\b|so\\b|to\\s+(?:talk|chat|keep\\s+going|carry\\s+on|go\\s+on)\\b)|\\s+left\\s*[.!?]?$)|\\bhow\\s+long\\s+(?:are|will)\\s+(?:you|u)\\s+(?:be\\s+)?(?:here|around|on)\\b|\\bhave\\s+(?:you|u)\\s+got\\s+(?:all\\s+day|time|much\\s+time)\\b|\\b(?:can|could|shall|should|may)\\s+we\\s+keep\\s+(?:talking|going|chatting|on)\\b|\\bdo\\s+(?:you|u)\\s+mind\\s+if\\s+i\\s+(?:keep|carry|ramble)\\b|\\bhow\\s+much\\s+time\\s+(?:have|has|do)\\s+(?:you|u|we)\\s+(?:got|have)\\b|\\bhow\\s+many\\s+(?:messages?|questions?|things)\\s+can\\s+i\\s+(?:send|ask|give)\\b|\\blimit\\s+(?:on|to)\\s+how\\s+long\\b[\\s\\S]{0,24}?\\b(?:talk|chat|go\\s+on|carry\\s+on|keep\\s+going)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEPING_ASK(stdlib_call("re", "compile", &[py_str("\\bam i\\s+(?:boring\\s+(?:you|u)\\b|keeping\\s+(?:you|u)\\b|a\\s+bore\\b|boring\\b|(?:talking|talk|going\\s+on|rambling|wittering|waffling|droning)\\s+too\\s+much\\b|going\\s+on(?:\\s+(?:a\\s+bit|and\\s+on|too\\s+long|like\\s+this))?(?:\\s*[.!?]\\s*$|\\s*$)|rambling\\b|wittering\\b|waffling\\b|droning\\s+on\\b|wasting\\s+(?:your|ur)\\s+time\\b)|\\bis\\s+(?:this|it)\\s+(?:boring\\s+(?:you|u)\\b|boring\\b|dull\\b|too\\s+much\\s+detail\\b|going\\s+on\\s+a\\s+bit\\b)|\\bdo i\\s+(?:talk|go\\s+on|ramble|witter|waffle)\\s+too\\s+much\\b|\\bare (?:you|u)\\s+(?:bored\\s+(?:of|with)\\s+(?:me|us|this|it)\\b|sick\\s+of\\s+me\\b|fed\\s+up\\s+(?:of|with)\\s+me\\b|had\\s+enough\\s+of\\s+me\\b)|\\bdo you\\s+mind\\s+(?:me|my)\\s+(?:going\\s+on|talking|rambling|wittering)\\b|\\b(?:am i|are we|was i)\\s+wasting\\s+(?:your|ur)\\s+time\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_STOP_ASK(stdlib_call("re", "compile", &[py_str("(?:(?:do|would)\\s+(?:you|u)\\s+want\\s+me\\s+to|(?:do|would)\\s+(?:you|u)\\s+like\\s+me\\s+to|(?:shall|should)\\s+i|(?:can|could)\\s+i)\\s+(?:stop|quit|shut\\s+up|leave\\s+it)(?:\\s+(?:now|then|talking|chatting|going\\s+on))?|(?:do|would)\\s+(?:you|u)\\s+(?:want|like)\\s+me\\s+to\\s+go(?:\\s+now)?")]).unwrap_or_else(|e| exc::raise(e)));
    s_STOP_ASK_FILLER(call_builtin("frozenset", &[mcall(&py_str("so and but ok okay well now then sorry hmm erm um oh ah hey hi\nright alright just also btw honestly"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STATE_WORDS(call_builtin("frozenset", &[mcall(&py_str("\nlow down rough tough hard heavy flat empty rubbish crap awful terrible horrible grim bad good\nfine better worse ok okay upset sad miserable unhappy disappointed gutted stressed anxious\nworried nervous scared frightened angry annoyed furious irritated frustrated bored tired\nexhausted knackered shattered drained wiped sleepy swamped overwhelmed snowed fed unwell\npoorly sick ill\nmad cross fuming livid seething raging irate miffed peeved riled\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PERSON_NOUNS(call_builtin("frozenset", &[mcall(&py_str("\nday days week weeks month months year years night nights morning mornings evening weekend\nterm shift shifts sleep time times life mood work job school college uni class classes exam\nexams test deadline presentation interview head back leg legs arm arms hand hands shoulder\nshoulders knee knees foot feet ankle ankles neck chest stomach tummy hip hips throat eye eyes\near ears tooth teeth skin hair voice nerves body health\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OTHER_PEOPLE(call_builtin("frozenset", &[mcall(&py_str("\nhe she they him her them his hers their mum mom dad mother father son daughter kid kids baby\nbabies toddler toddlers child children infant newborn boy boys girl girls teenager teen lad\npartner husband wife boyfriend girlfriend friend friends brother sister family colleague boss\nneighbour neighbor uncle aunt cousin gran grandma grandad grandpa nan nana mate mates\ndog dogs puppy pup cat cats kitten doggy hamster rabbit rabbits guinea budgie parrot pet pets\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MOOD_NOUNS(call_builtin("frozenset", &[mcall(&py_str("anxiety stress worry worries nerves mood depression grief sadness\nanger panic fear energy confidence motivation appetite sleep loneliness doubt\nhead brain mind memory concentration focus"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PERSON_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("i im me us we myself ourselves nobody nothing everything things"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BEFORE_STATE(sub(&call_builtin("frozenset", &[mcall(&py_str("\nam is are was were be been being have has had having do does did feel feels feeling felt get\ngets getting got seem seems seemed look looks looked sound sounds sounded taste tastes tasted\na an the so very really quite pretty too bit little lot kind kinda touch just still always again now\n today honestly totally all as than and but then of at with about over on in for from by my your\nour their his its am i am not no never still even both either\nwill would shall should can could may might must wont cant cannot couldnt shouldnt wouldnt\nok okay right well cool sure yeah yep yes alright oh ah hmm hey hi so\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("i")])));
    s_PERSON_PREPOSITIONS(call_builtin("frozenset", &[mcall(&py_str("of with about in at on"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CONJUNCTIONS(call_builtin("frozenset", &[mcall(&py_str("since because when while until though although if after before whenever and but or yet"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BETWEEN_STATE_AND_NOUN(call_builtin("frozenset", &[mcall(&py_str("\na an the bit little lot of kind kinda up down out off on at with about over in for from by my\nyour our their his its so very really quite pretty too just still always again now today\nhonestly totally all as than\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME_FILLERS(call_builtin("frozenset", &[mcall(&py_str("\nmorning afternoon evening night tonight noon midnight week weekend month year lately recently\nafterwards afterward yesterday tomorrow nowadays midweek\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MOOD_TAIL(py_tuple(vec![py_str("to be honest"), py_str("if i am honest"), py_str("if im honest"), py_str("to be fair"), py_str("to tell the truth"), py_str("to be truthful"), py_str("to be blunt"), py_str("if anything"), py_str("i must say"), py_str("i suppose"), py_str("i guess")]));
    s_MOOD_TAIL_WORDS(call_builtin("frozenset", &[mcall(&py_str("tbh tbf ngl honestly though tho"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MOOD_TAIL_TAKEN(call_builtin("frozenset", &[mcall(&py_str("going go goes went want wants wanted need needs needed try tries\n    trying tried mean means meant promise promised learn learned learnt used rather hope hoping\n    hate hates love loves have has had like likes prefer plan planning"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PARTICLE_STATES(call_builtin("frozenset", &[mcall(&py_str("down up out off in on"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_COPULA_STATES(call_builtin("frozenset", &[mcall(&py_str("ill"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOUN_STATES(call_builtin("frozenset", &[mcall(&py_str("flat"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SUPPLY_WORDS(call_builtin("frozenset", &[mcall(&py_str("low short"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SUPPLY_PREPOSITIONS(call_builtin("frozenset", &[mcall(&py_str("on in of"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_RESERVES(call_builtin("frozenset", &[mcall(&py_str("energy patience confidence motivation enthusiasm inspiration hope\n    sleep focus concentration attention steam willpower batteries charge ideas"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DEGREE(call_builtin("frozenset", &[mcall(&py_str("bit little lot kind sort very really so too pretty quite more most less\nas enough"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DETERMINERS(call_builtin("frozenset", &[mcall(&py_str("the a an my your our their his her its this that these those it them"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MOTION_WORDS(call_builtin("frozenset", &[mcall(&py_str("go goes going went gone walk walks walking walked wander wandering\n    head heads heading headed run runs running ran drive drives driving drove ride rides riding\n    rode cycle cycles cycling cycled pop pops popping popped come comes coming came hurry hurries\n    hurrying stumble stumbling jog jogs jogging climb climbing jump jumping fall falls falling\n    fell race racing rush rushing dash dashing nip nipping shuffle shuffling off"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME_AFTER_A_PARTICLE(call_builtin("frozenset", &[mcall(&py_str("week weeks month months year years morning mornings\n    afternoon evenings evening night nights weekend weekends time times term terms day days\n    while bit moment minute minutes hour hours"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MANIPULATE(call_builtin("frozenset", &[mcall(&py_str("turn turns turned turning keep keeps kept keeping put puts putting set\nsets setting hold holds holding let lets letting shut shuts shutting slow slows slowing calm\ncalms calming take takes taking bring brings bringing"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_COPULA(call_builtin("frozenset", &[mcall(&py_str("am is are was were be been being feel feels feeling felt get gets getting\ngot seem seems look looks sound sounds still very really quite pretty so too bit little a"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NUMBER_WORDS(call_builtin("frozenset", &[mcall(&py_str("one two three four five six seven eight nine ten eleven twelve"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_EVENT_NOUNS(call_builtin("frozenset", &[mcall(&py_str("funeral christening wake christmas easter birthday wedding anniversary\nexam exams interview scan operation surgery birth labour induction move housewarming holiday\nholidays honeymoon funeral term semester deadline appraisal review probation"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_RELATIVE_SUBJECTS(call_builtin("frozenset", &[py_tuple(vec![py_str("that"), py_str("which"), py_str("who")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BE_FORMS(call_builtin("frozenset", &[mcall(&py_str("am is are was were"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ECHO_CARRIERS(call_builtin("frozenset", &[mcall(&py_str("so and but yet or then as well too also just same here yes\nme i we us im m"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BOT_LIFE(call_builtin("frozenset", &[mcall(&py_str("\nfamily families kid kids child children son daughter mum mom mother father dad daddy parents\nparent brother brothers sister sisters sibling siblings gran grans grandma grandmas grandad\ngrandads grandpa grandparents grandparent uncle uncles aunt aunts auntie aunties cousin cousins\nniece nephews relatives relative partner husband wife girlfriend boyfriend friends friend\nhome house flat apartment town hometown address pets pet dog dogs cat cats kitten puppy hamster\nrabbit budgie garden\njob jobs work boss manager managers employer colleague colleagues coworker coworkers wages wage\nsalary paycheck pay paid money bank account mortgage bills rent\nbody face hands feet hair\nbirthday age born\nname nickname\ncar cars\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BOT_LIFE_FILLER(call_builtin("frozenset", &[mcall(&py_str("\na an any some the your you u ur yourself yours own much many really actually ever then now\nyet already also too about of for at in on like and or there both coming come came best\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BOT_LIFE_ASK(stdlib_call("re", "compile", &[py_str("\n    (?: (?:do|does|did)\\s+(?:you|u|ya)\\s+(?:have|has|got|get|own|need|want)\n      | (?:have|has)\\s+(?:you|u|ya)\\s+got\n      | (?:what|whats|what's|who|whos|who's|whose|which|how)\\s+\n        (?:is|are|was|were|s|does|do)?\\s*(?:your|ur)\n      | (?:are|is|was|were)\\s+(?:you|u)\\s+(?:a|an)\n      # \"were your mum and dad there\", \"was your brother there too\", \"are your parents\n      # coming\": the thing asked about is the bot's own family, said from the front with no\n      # \"do you have\" in it. Read as an ordinary question it reached the search hedge - \"I\n      # don't know enough to answer that honestly\" to somebody asking whether the bot's mum\n      # was at a party - when the bot knows the answer perfectly well: no family at all. The\n      # rest-gate below still has to pass, so \"is your shop open\" and \"were your prices\n      # online\" name something outside the bot's own life and are left alone.\n      | (?:are|is|was|were)\\s+(?:your|ur)\n    )\\s*\n"), stdlib_attr("re", "VERBOSE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ITSELF_LONG(stdlib_call("re", "compile", &[py_str("\n    ^\\s*(?:so\\s+|ok\\s+|okay\\s+|ah\\s+|oh\\s+|and\\s+|but\\s+)*\n    (?: how\\s+(?:long|many\\s+years)\n      | how\\s+old\n      | since\\s+when\n    )\n    \\s+(?:have|has|did)\\s+(?:you|u|ya)\\s+(?:been|be|start|started|go|going|got|get)\n    \\b\n"), stdlib_attr("re", "VERBOSE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ITSELF_LIFE(call_builtin("frozenset", &[mcall(&py_str("\ngoing around round about here there online alive running working doing this that it stuff\nthings talking chatting answering people folks folk to with us in on at now\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ITSELF_WHEN(stdlib_call("re", "compile", &[py_str("\n    ^\\s*(?:so\\s+|ok\\s+|okay\\s+|ah\\s+|oh\\s+|and\\s+|but\\s+)*\n    (?: when\\s+did\\s+you\\s+(?:start|begin|first\\s+start)\n      | when\\s+were\\s+you\\s+(?:made|built|started|made\\s+up)\n      | how\\s+long\\s+ago\\s+did\\s+you\\s+(?:start|begin)\n    )\n    \\b\n"), stdlib_attr("re", "VERBOSE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ITSELF_WHEN_TAIL(call_builtin("frozenset", &[mcall(&py_str("\ngoing out this that it stuff then today now here online working answering helping\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BOT_DOINGS(call_builtin("frozenset", &[mcall(&py_str("\ntalk talked talking speaks spoke spoken speaking chat chatted chatting answer answered\nanswering ask asked asking help helped helping heard listening sent send messages messaged\nmet meet meets\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BOT_COUNTED(call_builtin("frozenset", &[mcall(&py_str("\npeople person persons conversation conversations chat chats message messages question questions\nfolk folks\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNT_OPENERS(call_builtin("frozenset", &[mcall(&py_str("so and but then also"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BOT_HAVE(call_builtin("frozenset", &[mcall(&py_str("have has had having"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BOT_CHATS(call_builtin("frozenset", &[mcall(&py_str("\nchat chats conversation conversations talk talks discussion discussions catch-up\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BOT_DOERS(call_builtin("frozenset", &[mcall(&py_str("\nyou u ya yourself people person folk folks anyone anybody somebody someone everybody everyone\nthey customers users guests strangers visitors readers\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNT_FILLERS(call_builtin("frozenset", &[mcall(&py_str("\nhave has had having do does did done been being be is are was were ever already just really\nalso still not never once yet actually even only so far before since\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_EXPERIENCE_ASK(stdlib_call("re", "compile", &[py_str("^(?:(?:and|so|but|oh|well|ok|okay|right|hey|hi|yeah|actually|\nhonestly|just)\\s+)*(?:have|has|did|were|was)\\s+(?:you|u|ya)\\s+(?:ever|once)\\b"), stdlib_attr("re", "VERBOSE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_EXPERIENCE_WORDS(call_builtin("frozenset", &[mcall(&py_str("\nbeen be being was were had have got get went gone go going done do did seen see saw felt feel\ntried try made make met meet ate eaten eat drank drunk smoked ridden rode driven drove worn wore\nlived live cry cried laugh laughed hurt love loved hate hated kiss kissed miss missed win won lose\nlost give gave given giving keep kept run ran swim swam cook cooked\nup down through ill sick unwell happy sad tired lonely scared frightened angry upset worried\nglad proud bored excited drunk lost\nday days night nights week weeks month months year years time times life lived world holiday\nholidays\nthing things something anything nothing one ones kind sort bit lot lots\nit them that this these those there before once ever really actually\nmyself yourself\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_PROBLEM(call_builtin("frozenset", &[mcall(&py_str("\nmy our me us mine ours happen happens happened occurring seems broken broke leaking dripping noisy\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_KIN(call_builtin("frozenset", &[mcall(&py_str("family families kid kids child children son daughter mum mom mother father dad\n    daddy parents parent brother brothers sister sisters sibling siblings gran grans grandma\n    grandmas grandad grandads grandpa grandparents grandparent uncle uncles aunt aunts auntie\n    aunties cousin cousins niece nephews relatives relative partner husband wife girlfriend\n    boyfriend"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_YOUR_KIN(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("\\b(?:(?:was|were|are|is)\\s+(?:your|ur|yours)\\s+(?:"), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[g_KIN()]).unwrap_or_else(|e| exc::raise(e))])), &py_str(")|(?:do|does|did|have|has)\\s+(?:your|ur)\\s+(?:")), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[g_KIN()]).unwrap_or_else(|e| exc::raise(e))])), &py_str("))\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_YOUR_OWN_THING(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:(?:what|how)\\s+about|and)\\s+(?:your|ur)\\s+(?:"), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[g_BOT_LIFE()]).unwrap_or_else(|e| exc::raise(e))])), &py_str(")(?:\\s+(?:too|then|either|as\\s+well))?\\s*[?.!]*$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HOW_MANY_OF_THE_BOTS_OWN(stdlib_call("re", "compile", &[py_str("\\b(?:how|what)\\s+many\\s+\\w+(?:\\s+\\w+)?\\s+(?:have|has|do|does)\\s+(?:you|u|ya)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WHAT_ITS_LIKE(stdlib_call("re", "compile", &[py_str("\\b(?:what|whats|what's)\\b.*\\blike\\s+being\\s+(?:you|u|ya)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_JOIN_OPENERS(call_builtin("frozenset", &[mcall(&py_str("and"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_AFTER_A_POSSESSIVE(call_builtin("frozenset", &[mcall(&py_str("boy girl son daughter child kid baby man woman\n    brother sister mum mom dad mother father friend wife husband partner"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_LEARNING_WORDS(call_builtin("frozenset", &[mcall(&py_str("learn learns learnt learned learning remember remembers\n    remembered recall recalls forget forgets pick picks take takes keep keeps"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HAVING(call_builtin("frozenset", &[mcall(&py_str("have has had having got gets getting want wants wanted expecting\n    trying planning"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CHILD_NOUNS(call_builtin("frozenset", &[mcall(&py_str("boy girl son daughter child kid kids baby babies twins"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_POSSESSIVE_WORDS(call_builtin("frozenset", &[mcall(&py_str("your my our his her their"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MAKER_WORDS(call_builtin("frozenset", &[mcall(&py_str("made make makes making created create built build built trained programmed program designed wrote written"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__I_SAID_IT(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|ok|okay|right|now|then|wait)[,. ]+)*(?:what|which|when|where)\\b[^.?!]*?\\bi\\b[^.?!]*?\\b(?:said|say|told|tell|mentioned|mention|asked|ask)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BEFORE_COURTESY(call_builtin("frozenset", &[mcall(&py_str("aw aww ah ahh oh ohh ooh awe ha haha um umm er hmm"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_COURTESY_THING(call_builtin("frozenset", &[mcall(&py_str("card cards letter letters message messages note notes mail\n    email emails text texts speech poem poem story banner sign tag tags gift gifts present\n    presents cake wine chocolates flowers box bag"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NEEDS_ANSWER(call_builtin("frozenset", &[mcall(&py_str("hurt hurts hurting hurt ache aches aching pain painful sore agony\n    worse worst not never wont cant cannot dont doesnt didnt isnt arent wasnt weren kept\n    killing killed\n    keeps stopped stops broken broke breaks leaking leaks leak stuck died dead die rash\n    swollen bleeding bled sick unwell slow noisy noise smells burning stings itches\n    problem problems wrong stuck rubbish awful terrible worried worrying scared"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_WRONG(stdlib_call("re", "compile", &[py_str("\n    \\b(?:\n        (?!cache\\b)\\w*ache | fever(?:ish)? | cough(?:ing)? | dizzy | nauseous? | vomit(?:ing)?\n      | shiver(?:s|ing|y)? | chills | dehydrat\\w+\n      | (?:have|has|had|got|gotten|running|caught|catch|my|his|her|their|your)\n        \\s+(?:an?\\s+)?(?:high\\s+|slight\\s+|bit\\s+of\\s+a\\s+|touch\\s+of\\s+a\\s+)?temperature\n    )\\b\n"), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FOUND(call_builtin("frozenset", &[mcall(&py_str("blog article post page site website video photo picture recipe thread\n    comment question quiz idea story podcast episode book film answer deal bargain\n    mind"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SLIPPED_FROM(call_builtin("frozenset", &[mcall(&py_str("across upon into through by away past"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FALL(call_builtin("frozenset", &[mcall(&py_str("fallen fainted"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FELL_WORDS(bitor(&g_FALL(), &call_builtin("frozenset", &[mcall(&py_str("fell fall falls falling tripped tripping slipped slipping\n    stumbled stumbling collapsed collapse"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_GREETING_TAIL(call_builtin("frozenset", &[mcall(&py_str("there again all folks friends everyone everybody guys you your\n    mate quick question hope hopes hoping doing day today going well good thanks thank then\n    too also just so how are is it things up sup life ok okay alright fine here back"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_INTRODUCES(py_tuple(vec![py_tuple(vec![py_str("my"), py_str("name"), py_str("is")]), py_tuple(vec![py_str("the"), py_str("name"), py_str("is")]), py_tuple(vec![py_str("i"), py_str("am")]), py_tuple(vec![py_str("im")]), py_tuple(vec![py_str("call"), py_str("me")]), py_tuple(vec![py_str("this"), py_str("is")]), py_tuple(vec![py_str("its")])]));
    s_FAREWELL_TAIL(call_builtin("frozenset", &[mcall(&py_str("then ok okay right well so all everyone everybody guys folks mates\n    now for to soon later tomorrow tonight night good xx xxx love hun"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__SAID_ON_ITS_OWN(call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&add(&g_THANKS(), &g_FAREWELLS())) { let mut phrase: Py = __item.clone(); for __item in py_iter_or_raise(&mcall(&phrase.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s__THANKS_WORDS(call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_THANKS()) { let mut phrase: Py = __item.clone(); for __item in py_iter_or_raise(&mcall(&phrase.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s__NIGHT_WISHES(py_tuple(vec![py_str("sleep well"), py_str("sleep tight"), py_str("sleep good"), py_str("sweet dreams"), py_str("rest well"), py_str("night night")]));
    s__WISH_WORDS(call_builtin("frozenset", &[mcall(&mcall(&py_str(" "), "join", &[g__NIGHT_WISHES()]), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMING_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("called"), py_str("named"), py_str("name"), py_str("names")])]).unwrap_or_else(|e| exc::raise(e)));
    s_HELP_WAS_GOOD(call_builtin("frozenset", &[py_tuple(vec![py_str("that helps"), py_str("this helps"), py_str("that really helps"), py_str("this really helps"), py_str("that helps a lot"), py_str("that helps loads"), py_str("that helps so much"), py_str("that helped"), py_str("this helped"), py_str("that really helped"), py_str("this really helped"), py_str("that helped a lot"), py_str("that helped loads"), py_str("that helped so much"), py_str("thats helpful"), py_str("that is helpful"), py_str("that was helpful"), py_str("this is helpful"), py_str("this was helpful")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BEFORE_HELP_WAS_GOOD(call_builtin("frozenset", &[mcall(&py_str("ok okay yeah yep yes ah oh right cool so"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_AFTER_HELP_WAS_GOOD(call_builtin("frozenset", &[mcall(&py_str("actually tbh honestly really though tho anyway now then\n    mind cheers thanks"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_AFTER_HELP_WAS_GOOD_SAID_OUT(py_tuple(vec![py_str("to be honest"), py_str("to be fair"), py_str("if im honest"), py_str("if i am honest"), py_str("in all honesty"), py_str("to tell the truth")]));
    s_THANKS_ABOUT(call_builtin("frozenset", &[mcall(&py_str("that this it they them those these which"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BETTER(call_builtin("frozenset", &[mcall(&py_str("better best glad happy pleased relieved easier lighter improved gone\n    tried try trying worked works"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SAYS_ABOUT_YOU(call_builtin("frozenset", &[mcall(&py_str("you your yours u ya"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PRAISING(call_builtin("frozenset", &[mcall(&py_str("perfect great brilliant lovely delicious gorgeous superb spot amazing\n    excellent wonderful nice good marvellous marvelous fantastic smashing"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PRAISING_VERB(call_builtin("frozenset", &[mcall(&py_str("loved love loves liked like likes enjoyed enjoy enjoys adored\n    devoured demolished wolfed"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PRAISE_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:went|gone)\\s+down\\s+(?:well|a\\s+treat|great|nicely)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TURNED_ROUND(call_builtin("frozenset", &[mcall(&py_str("but though although except however"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STEP_VERBS(call_builtin("frozenset", &[mcall(&py_str("cleared restarted rebooted reinstalled uninstalled deleted removed\n    emptied cleaned washed rinsed changed swapped reset updated charged plugged poured soaked\n    drained defrosted tried gave followed did put set"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STEP_ANY_OBJECT(call_builtin("frozenset", &[mcall(&py_str("put set did gave changed"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STEP_WRONG(call_builtin("frozenset", &[mcall(&py_str("wrong mistake error errors failed fails broken broke wont cant\n    stopped worse nothing none no not still again"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SIGNING_OFF(stdlib_call("re", "compile", &[py_str("\n    \\b(?:i'?m|im|i\\s+am|we'?re|were|we\\s+are)\\s+(?:off|away|going|turning\\s+in|heading)\\b\n    | \\b(?:good\\s*night|goodnight|bedtime|off\\s+to\\s+bed|turning\\s+in)\\b\n    | \\bnothing\\s+(?:else|more|left)\\b | \\bnothing\\s+to\\s+do\\b\n    "), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AGREE_TAIL(bitor(&call_builtin("frozenset", &[mcall(&py_str("it that this these those all so much very quite a an lot bit really\n    pretty good great fine nice cool ok okay lovely brilliant magic perfect spot better best\n    well helps helped works worked working doing do does is was been am are you your not no\n    too to and now enough still same here there just also then\n    love loved loving like liked liking righto alright ace grand decent pleased glad happy\n    kind kindest thoughtful generous sweet\n    one ones of"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_AFFIRM()));
    s_AGREE_AFTER_NOT(call_builtin("frozenset", &[mcall(&py_str("bad half wrong shabby"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_FILLER(call_builtin("frozenset", &[mcall(&py_str("too that so very much all quite a an to even really feeling"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_THANKS_NOISE(call_builtin("frozenset", &[mcall(&py_str("oh ah aha aah ahh ohh ooh aw aww mm mmm"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ADDRESSED(call_builtin("frozenset", &[mcall(&py_str("love mate pet hun duck sunshine hen flower chuck dear pal buddy\n    babes shug lovey"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OPENING_FRAME(stdlib_call("re", "compile", &[py_str("\n    \\b(?:can|could|may|might)\\s+i\\s+ask\\s+(?:you\\s+)?(?:something|somethin|a\\s+question|one\\s+thing|advice|for\\s+advice|your\\s+advice)\\b\n  | \\b(?:i|ive)?\\s*(?:have|got|has)\\s+(?:a\\s+|one\\s+|quick\\s+)?question\\b\n  | \\bquick\\s+question\\b\n  | \\b(?:i|we)\\s+(?:need|want|would\\s+like|could\\s+use)\\s+(?:(?:some|a\\s+bit\\s+of)\\s+)?(?:your\\s+)?advice\\b\n  | \\b(?:can|could|would|will)\\s+you\\s+help\\s+me\\s+(?:with\\s+something|with\\s+a\\s+thing|with\\s+one\\s+thing|out\\s+with\\s+something)\\b\n  | \\bmind\\s+if\\s+i\\s+ask\\s+(?:you\\s+)?(?:something|a\\s+question)\\b\n  | \\bi\\s+(?:want|need)\\s+to\\s+ask\\s+you\\s+(?:something|a\\s+question)\\b\n"), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OPENING_WORDS(call_builtin("frozenset", &[mcall(&py_str("can could may might i ask you something somethin a one thing\n    question quick have got has ive we need want would like use some bit of for your advice\n    help me with out will to if mind is it"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OPENING_MAX(Py::Int(9));
    s_TALK_ASK_VERBS(py_str("(?:vent|rant|moan|whinge|grumble|offload|unload)"));
    s_TALK_TAIL_WORDS(py_str("(?:\\s+(?:a\\s+bit|a\\s+minute|a\\s+sec|a\\s+moment|one\\s+thing|for\\s+a\\s+minute|for\\s+a\\s+bit|about\\s+something|about\\s+it|about\\s+this|to\\s+you|to\\s+someone|on\\s+you|here|first))*"));
    s_TALK_ASK_OPENERS(py_str("(?:(?:can|could|may)\\s+(?:i|we)|do\\s+you\\s+mind\\s+if\\s+i|would\\s+you\\s+mind\\s+if\\s+i|is\\s+it\\s+ok\\s+if\\s+i|is\\s+it\\s+okay\\s+if\\s+i)"));
    s_TALK_OPENING(stdlib_call("re", "compile", &[mcall(&py_str("|"), "join", &[py_tuple(vec![add(&add(&py_str("(?:i\\s+|we\\s+)?(?:just\\s+|really\\s+|honestly\\s+|properly\\s+)?(?:am\\s+|are\\s+)?(?:(?:want|wanna|need|have|got|gotta|planning|going|gonna)\\s+)?(?:to\\s+)?"), &g_TALK_ASK_VERBS()), &g_TALK_TAIL_WORDS()), add(&add(&py_str("(?:can|could|may)\\s+(?:i|we)\\s+(?:just\\s+)?(?:have\\s+a\\s+)?"), &g_TALK_ASK_VERBS()), &g_TALK_TAIL_WORDS()), add(&py_str("(?:i\\s+|we\\s+)?(?:just\\s+)?(?:am\\s+|are\\s+)?(?:going\\s+to\\s+|gonna\\s+)?(?:(?:want|wanna|need|have|got|gotta)\\s+(?:to\\s+)?)?(?:have\\s+)?a\\s+(?:proper\\s+|good\\s+|quick\\s+|little\\s+)?(?:vent|rant|moan|whinge|grumble)"), &g_TALK_TAIL_WORDS()), py_str("(?:i\\s+)?(?:just\\s+)?(?:(?:want|wanna|need|have|got|gotta)\\s+(?:to\\s+)?|(?:can|could|may)\\s+i\\s+|let\\s+me\\s+)(?:get\\s+)?(?:something|somethin|this|it)\\s+off\\s+my\\s+chest"), add(&py_str("(?:i\\s+|we\\s+)?(?:just\\s+|really\\s+)?(?:am\\s+|are\\s+)?(?:(?:want|wanna|need|have|got|gotta|going)\\s+(?:to\\s+)?|(?:can|could|may)\\s+(?:i|we)\\s+)talk"), &g_TALK_TAIL_WORDS()), py_str("(?:i\\s+)?(?:need|want|would\\s+like)\\s+(?:someone|somebody|anyone|anybody)\\s+to\\s+talk\\s+to"), py_str("(?:can|could|may)\\s+i\\s+(?:have\\s+a\\s+word)"), py_str("(?:i\\s+)?(?:need|want)\\s+a\\s+word"), add(&add(&g_TALK_ASK_OPENERS(), &py_str("\\s+(?:run|bounce|try|talk\\s+through)\\s+(?:something|somethin|this|that|it|an?\\s+idea|one\\s+thing|a\\s+couple\\s+of\\s+things|a\\s+few\\s+things|some\\s+ideas?)\\s+(?:by|past|off)\\s+you")), &g_TALK_TAIL_WORDS()), add(&add(&g_TALK_ASK_OPENERS(), &py_str("\\s+(?:pick\\s+your\\s+brains?|bend\\s+your\\s+ear)")), &g_TALK_TAIL_WORDS()), add(&add(&g_TALK_ASK_OPENERS(), &py_str("\\s+(?:tell\\s+you\\s+what\\s+happened|share\\s+(?:something|this|it)\\s+with\\s+you|have\\s+a\\s+second\\s+opinion|get\\s+a\\s+second\\s+opinion)")), &g_TALK_TAIL_WORDS())])])]).unwrap_or_else(|e| exc::raise(e)));
    s_TALK_HEAD(call_builtin("frozenset", &[mcall(&py_str("right ok okay so look well honestly listen alright now then"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TALK_TAIL(py_tuple(vec![py_tuple(vec![py_str("to"), py_str("be"), py_str("honest")]), py_tuple(vec![py_str("if"), py_str("that"), py_str("is"), py_str("ok")]), py_tuple(vec![py_str("if"), py_str("thats"), py_str("ok")]), py_tuple(vec![py_str("now")]), py_tuple(vec![py_str("please")]), py_tuple(vec![py_str("then")]), py_tuple(vec![py_str("thanks")]), py_tuple(vec![py_str("right")]), py_tuple(vec![py_str("actually")]), py_tuple(vec![py_str("quickly")]), py_tuple(vec![py_str("quick")]), py_tuple(vec![py_str("mate")])]));
    s_COMMIT_OPEN(call_builtin("frozenset", &[mcall(&py_str("will wont would gonna going sall ill il id weve well lets let"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_COMMIT_TRY(call_builtin("frozenset", &[mcall(&py_str("try give go see have take"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_COMMIT_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("i we"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_COMMIT_ALONE(py_tuple(vec![py_str("give it a go"), py_str("give it a try"), py_str("give that a go"), py_str("give that a try"), py_str("give it a shot"), py_str("have a go"), py_str("have a try"), py_str("give it a whirl"), py_str("try that"), py_str("try it"), py_str("try this")]));
    s_COMMIT_ALONE_TAIL(py_tuple(vec![py_str("let you know"), py_str("sounds like a plan")]));
    s_HEDGED_COMMIT(call_builtin("frozenset", &[mcall(&py_str("might could may"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_HEDGED_ALONE(add(&g_COMMIT_ALONE(), &py_tuple(vec![py_str("do that"), py_str("do it"), py_str("go for it"), py_str("ring them"), py_str("call them")])));
    s_COMMIT_NO_HARM(py_tuple(vec![py_str("worth a try"), py_str("worth a go"), py_str("worth trying"), py_str("worth a shot"), py_str("cant hurt"), py_str("cannot hurt"), py_str("no harm in"), py_str("nothing to lose")]));
    s_HEDGE_LEAD_ONE(call_builtin("frozenset", &[mcall(&py_str("maybe probably suppose guess honestly"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_HEDGE_LEAD_TWO(py_tuple(vec![py_str("i suppose"), py_str("i guess"), py_str("i think"), py_str("i reckon")]));
    s_NO_HARM_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("it its that this there"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_COMMIT_STOP(call_builtin("frozenset", &[mcall(&py_str("you u ya your yourself yall"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_THE_ROOM(call_builtin("frozenset", &[mcall(&py_str("anyone anybody somebody someone everyone everybody any1 some1\n    every1 evry1 no1"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_THE_ROOM_ANY(call_builtin("frozenset", &[mcall(&py_str("anyone anybody any1"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_THE_ROOM_WANTS(call_builtin("frozenset", &[mcall(&py_str("where if how what which whether why who whom whose\n    a an any some"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_THE_ROOM_VERB(call_builtin("frozenset", &[mcall(&py_str("know knows knew got have has had recommend recommends\n    suggest suggests tried try seen see use uses found find want wants need needs"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TAIL_ASKS(call_builtin("frozenset", &[mcall(&py_str("ideas idea thoughts thought advice tips tip suggestions suggestion\n    recommendations recommendation help pointers"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TAIL_ASK_FILLER(call_builtin("frozenset", &[mcall(&py_str("any some more your please a quick few"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SETUP_MOST_WORDS(Py::Int(45));
    s_SETUP_NOTHING(call_builtin("frozenset", &[mcall(&py_str("thanks thank you ta cheers hi hello hey morning evening goodnight ok\n    okay right so well and but oh ah hmm um yeah yes no anyway alright lol haha lmao"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SETUP_CONNECTIVES(call_builtin("frozenset", &[mcall(&py_str("since because cos cause and but so if when while although\n    though as then also plus or with for to of in on at by from that it its this these those\n    there here neither nor either both too same me"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FRAME_NOT_A_SETUP(stdlib_call("re", "compile", &[py_str("^(?: (?:and |but |oh |well |so |ok |okay |right |no )?(?:never mind|nevermind|i mean|i meant|you know what i mean|i dunno|dunno|not sure|no idea|honestly|sorry|one more thing|one thing|another thing|quick one|back to \\w+(?: \\w+){0,2}|of course|fair enough|me too|same here|no worries|anyway|just wondering|just asking|quick question|random question|random one|hi|hey|hello|hiya|good question)\\b(?: .{0,12})?)+$")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FRAME_IN_FRONT(stdlib_call("re", "compile", &[py_str("^(?:(?:hi|hiya|hey|hello|morning|evening|ok|okay|right|well|also|i mean|i meant|honestly|sorry|cheers|thanks|thank you|no worries|quick one|quick question|random question|random one|one more thing|another thing|just wondering|just asking|lol|haha|lmao|anyway|bit of a random one|bit random|random one|and|but|so)(?:[\\s,.-]+|$))+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TIPS_FRAME(call_builtin("frozenset", &[mcall(&py_str("any some few got get need needs needed wanted looking look for\n    share send give me us please pls thanks thank you tips tip hints hint pointers pointer\n    advice a an the and or to on about this that it is are do have has i u my"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_QUESTION_NOUNS(call_builtin("frozenset", &[mcall(&py_str("q qs question questions query queries"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKING_VERBS(call_builtin("frozenset", &[mcall(&py_str("ask asks asked asking"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_ABOUT(call_builtin("frozenset", &[mcall(&py_str("about abt bout re regarding on with something anything"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_FRAMES(call_builtin("frozenset", &[mcall(&py_str("a an and another ask asked asking but can could daft got have has\n    hey hi i ive just little may mind need one please pls quick random silly small sorry stupid\n    wanna want we would will"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST_ASK(call_builtin("frozenset", &[mcall(&py_str("asked told answered repeated mentioned"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ALREADY_ASKED(call_builtin("frozenset", &[mcall(&py_str("what why how when where who which whose whether"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_MODALS(call_builtin("frozenset", &[mcall(&py_str("can could may might want wants wanted wanna need needs needed"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_AUX(call_builtin("frozenset", &[mcall(&py_str("am is are was were im be been being just only still actually ever also\n    always"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_TARGET(call_builtin("frozenset", &[mcall(&py_str("you u something somethin anyone anybody"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_HAND_OVER_VERBS(call_builtin("frozenset", &[mcall(&py_str("give gives gave send sends sent throw throws threw chuck chucks\n    hit hits fire fires read reads bring brings brought pass passes offer offers ask asks quiz\n    quizzes test tests"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_HAND_OVER_TO(call_builtin("frozenset", &[mcall(&py_str("me us"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_HAND_OVER_MINE(call_builtin("frozenset", &[mcall(&py_str("knowledge trivia quiz quizzes iq memory brain"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_HAND_OVER_LEADS(call_builtin("frozenset", &[mcall(&py_str("please pls plz kindly just maybe perhaps ok okay so and then now\n    also quick quickly"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_HAND_OVER_MODALS(call_builtin("frozenset", &[mcall(&py_str("can could will would should"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_HAND_OVER_SUBJECTS(call_builtin("frozenset", &[mcall(&py_str("you u ya"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_YOU_WORDS(call_builtin("frozenset", &[mcall(&py_str("you u ya ur dya"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_VERBS(call_builtin("frozenset", &[mcall(&py_str("think reckon guess recommend suggest mind"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_FOLLOW(call_builtin("frozenset", &[mcall(&py_str("i we us ill im id it its that this these those she he they them\nthere my your our their should can could will would shall is are was were do does did if\nwhether what when where who which why how any anything something someone"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ANY_ANSWER_WANTED(call_builtin("frozenset", &[mcall(&py_str("word words luck idea ideas chance chances\n    joy sign signs hint hints intel new else"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ANY_ANSWER_LINKS(call_builtin("frozenset", &[mcall(&py_str("on about with from of for regarding concerning re"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ANY_ALREADY_ASKS(call_builtin("frozenset", &[py_tuple(vec![py_str("idea"), py_str("ideas")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_FOR_A_NAME(call_builtin("frozenset", &[mcall(&py_str("know knows knew recommend recommends suggest suggests\n    found find"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_COMPARATIVE(call_builtin("frozenset", &[py_tuple(vec![py_str("best"), py_str("better")])]).unwrap_or_else(|e| exc::raise(e)));
    s_DESCRIBES(call_builtin("frozenset", &[py_tuple(vec![py_str("feel"), py_str("feels"), py_str("feeling"), py_str("felt"), py_str("do"), py_str("does"), py_str("did"), py_str("doing"), py_str("done"), py_str("get"), py_str("gets"), py_str("getting"), py_str("got"), py_str("look"), py_str("looks"), py_str("looking"), py_str("seem"), py_str("seems"), py_str("am"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("be"), py_str("being"), py_str("been"), py_str("much"), py_str("any"), py_str("lot"), py_str("sleep"), py_str("sleeping"), py_str("slept"), py_str("eat"), py_str("eating"), py_str("ate"), py_str("cope"), py_str("coping"), py_str("manage"), py_str("managing")])]).unwrap_or_else(|e| exc::raise(e)));
    s_AFTER_COMPARATIVE(call_builtin("frozenset", &[py_tuple(vec![py_str("thanks"), py_str("thank"), py_str("today"), py_str("tomorrow"), py_str("yesterday"), py_str("now"), py_str("tonight"), py_str("though"), py_str("again"), py_str("yet"), py_str("already"), py_str("honestly"), py_str("actually"), py_str("here"), py_str("since"), py_str("after")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_AFTER(call_builtin("frozenset", &[mcall(&py_str("\n    friend friends friend's friends' mate mates mate's pal pals man men woman women half buddy\n    bud fella lad matey mother father sister brother son daughter partner wife husband\n    wishes regards\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_NAMED(bitor(&bitor(&bitor(&g_A_PERSON_AFTER_A_POSSESSIVE(), &g_KIN()), &g_A_PERSON_AFTER()), &call_builtin("frozenset", &[mcall(&py_str("boss bosses manager managers colleague colleagues\n                      neighbour neighbours neighbor neighbors teacher teachers doctor\n                      doctors gp dentist dentist's"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_OWNS_A_PERSON(call_builtin("frozenset", &[py_tuple(vec![py_str("my"), py_str("our"), py_str("his"), py_str("her"), py_str("your"), py_str("their"), py_str("me")])]).unwrap_or_else(|e| exc::raise(e)));
    s_COMPARISON(stdlib_call("re", "compile", &[py_str("(?:\\s+or\\s+|\\bbetter than\\b|\\binstead of\\b|\\bversus\\b|\\bvs\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_IT_BACK(py_tuple(vec![py_str("how about you"), py_str("what about you"), py_str("how about yourself"), py_str("what about yourself"), py_str("and you"), py_str("and yourself"), py_str("you yourself"), py_str("what about u"), py_str("how about u"), py_str("how about yours"), py_str("what about yours"), py_str("and yours")]));
    s_FILLER_BEFORE(py_tuple(vec![py_str("so"), py_str("ok"), py_str("okay"), py_str("oh"), py_str("yeah"), py_str("but"), py_str("well")]));
    s_FILLER_AFTER(py_tuple(vec![py_str("then"), py_str("though"), py_str("now"), py_str("anyway"), py_str("huh"), py_str("eh")]));
    s_BOUT(py_dict(vec![(py_str("bout"), py_str("about"))]));
    s_GREETING_ASK_WORDS(call_builtin("frozenset", &[mcall(&py_str("\n    how what whats is are was were am be been r u ya you your yours yourself yourselves the\n    things everything life day days week weekend evening morning night\n    going doing doin do been getting keeping saying say much\n    today tonight then now lately anyway mate love friend\n    all alright ok okay good fine well too so it have has having\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GREETING_ASK_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("you u ya your yours yourself yourselves things everything\n    life day days week weekend evening morning night"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GREETING_ASK_STATE(call_builtin("frozenset", &[mcall(&py_str("ok okay alright good fine well"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GREETING_ASK_ABOUT_SOMETHING(call_builtin("frozenset", &[mcall(&py_str("with about to for if when on at in"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GREETING_ASK_SECOND_PERSON(call_builtin("frozenset", &[mcall(&py_str("you u ya your yours yourself yourselves"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GREETING_WHAT_SAYING(call_builtin("frozenset", &[mcall(&py_str("what are is r you u ya saying say sayin then now there\n    mate all"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TAG_RIGHT_NOT(call_builtin("frozenset", &[mcall(&py_str("i me my mine you your yours he him his she her hers it its we us our\n    ours they them their theirs this that these those there here what who which whose\n    all any anything everything something nothing none both each either neither another\n    such same very quite too so just still even only also always never ever really pretty\n    probably maybe perhaps definitely certainly nearly almost exactly absolutely totally\n    completely obviously clearly simply possibly surely indeed actually basically honestly\n    literally truly about around much many more most less least enough not no yes yeah ok\n    okay right\n    is are was were am be been being do does did done have has had having can could will\n    would shall should may might must need ought feel feels felt look looks looked seem\n    seems seemed sound sounds sounded taste tastes tasted smell smells smelled get gets got\n    gotten go goes went gone keep keeps kept turn turns turned make makes made put puts set\n    sets work works worked\n    off on out up down in back over through round around at\n    fine ok okay alright good better worse ready done sorted"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TAG_ENDINGS(call_builtin("frozenset", &[py_tuple(vec![py_str("right"), py_str("innit")])]).unwrap_or_else(|e| exc::raise(e)));
    s_TAG_PLAN(bitor(&g_GOING_ON_WHEN(), &call_builtin("frozenset", &[mcall(&py_str("day days dinner lunch breakfast tea supper later soon\n    next"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_TAG_TIME_NUMBER(call_builtin("frozenset", &[mcall(&py_str("one two three four five six seven eight nine ten eleven twelve\n    half"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TAG_CHECKABLE(call_builtin("frozenset", &[mcall(&py_str("should shouldnt shall can cant could couldnt will wont would wouldnt\n    may might must ought needs need gets said says told tells"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BOT_DAY_WORDS(mcall(&py_str("|"), "join", &[py_tuple(vec![py_str("day"), py_str("weekend"), py_str("week"), py_str("evening"), py_str("morning"), py_str("night")])]));
    s_BOT_DAY_SHAPES(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("(?:how|what) about (?:your|ur) (?:{day})(?: (?:been|going|gone|so far))?"), py_str("(?:how|what) about yours"), py_str("and (?:your|ur) (?:{day})(?: (?:been|going|gone|so far))?"), py_str("and yours"), py_str("(?:how|what)(?:s|'s| is| was| are| were) (?:your|ur) (?:{day})(?: (?:been|going|gone|good|ok|okay|fine|busy|alright|so far))?"), py_str("(?:was|were|is|are|did) (?:your|ur) (?:{day})(?: (?:good|ok|okay|fine|busy|alright|nice))?"), py_str("(?:did|have|had) you have a (?:good|nice|ok|okay|busy|long|lovely|great) (?:{day})"), py_str("hope (?:you had|youre having|you are having) a (?:good|nice|lovely|great|ok|okay) (?:{day})")])) { let mut pattern: Py = __item.clone(); __out.push(stdlib_call("re", "compile", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("day".to_string(), g_BOT_DAY_WORDS())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "format", args))(&__args) }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_IF_ANYTHING(call_builtin("frozenset", &[mcall(&py_str("anything something nothing owt"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SPENDS_ITS_TIME(stdlib_call("re", "compile", &[py_str("\\b(?:how|what)\\b[^.]{0,24}?\\b(?:spend|spends|spending|fill|fills|filling|use|uses)\\b[^.]{0,20}?\\b(?:your|you)\\b[^.]{0,16}\\b(?:time|day|days|evening|evenings|hours?|life|nights?)\\b|\\b(?:what do|what does)\\s+you\\s+do\\s+(?:all\\s+)?(?:day|with your|during the|in the)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ABOUT_AT_A_TIME(call_builtin("frozenset", &[mcall(&py_str("tonight tomorrow today later weekend weekends morning afternoon\n    evening night now soon week"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FREE_OR_BUSY(call_builtin("frozenset", &[mcall(&py_str("busy free"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FIRST_PERSON(call_builtin("frozenset", &[mcall(&py_str("i im id ive me my mine we us our"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MANNERS_WORDS(call_builtin("frozenset", &[mcall(&py_str("up doing much keeps keeping"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MANNERS_OPENERS(call_builtin("frozenset", &[mcall(&py_str("what whats wats"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_IN_THE_ASK(call_builtin("frozenset", &[mcall(&py_str("later tomorrow morning afternoon evening night weekend weekends\n    week weekdays this these days coming soon next"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SAYS_WHAT_ARE_YOU(call_builtin("frozenset", &[mcall(&py_str("watcha whatcha wotcha watchu whatchu watchya whatchya"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEPS_BUSY_SHAPE(call_builtin("frozenset", &[mcall(&py_str("what whats wats is keeps keeping you u ya busy so then mate\n    today tonight now lately these days though anyway"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NEW_ASK_WORDS(call_builtin("frozenset", &[mcall(&py_str("what whats is was has have u you ya your yours with in on going\n    happening been new anything something nothing else tell me us mate pal friend hey hi hello\n    so and now then lately world side day days weekend week"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NEW_FRAME(call_builtin("frozenset", &[mcall(&py_str("new going happening been"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NEW_ASK_AHEAD(call_builtin("frozenset", &[mcall(&py_str("tonight tomorrow later next saturday sunday monday tuesday\n    wednesday thursday friday"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GRATITUDE_OBJECT(call_builtin("frozenset", &[mcall(&py_str("a an the all any some your his her their our my that this these\n    those lot lots loads much very so really again both few bit kind sort great good helpful\n    useful nice lovely brilliant ever indeed awfully terribly truly too help support"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GRATITUDE_JOINERS(call_builtin("frozenset", &[py_list(vec![py_str("and"), py_str("plus"), py_str(","), py_str("&")])]).unwrap_or_else(|e| exc::raise(e)));
    s_GRATITUDE_ASK_AGAIN(call_builtin("frozenset", &[mcall(&py_str("any some what how which more another"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TIP_WORDS(call_builtin("frozenset", &[mcall(&py_str("tips hints pointers tip hint pointer"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_COURTESY_WORD(call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_THANKS()) { let mut phrase: Py = __item.clone(); for __item in py_iter_or_raise(&mcall(&phrase.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DATE_FIGURE(stdlib_call("re", "compile", &[py_str("\\b\\d{1,2}(?:st|nd|rd|th)\\b|\\b\\d{1,2}\\s+(?:of\\s+)?(?:january|february|march|april|may|june|july|august|september|october|november|december)\\b|\\b(?:january|february|march|april|may|june|july|august|september|october|november|december)\\s+\\d{1,2}\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOUN_REQUEST_MAX(Py::Int(8));
    s_PLACE_WORD(call_builtin("frozenset", &[mcall(&py_str("in at near around from to"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOUN_REQUEST_SELF(call_builtin("frozenset", &[mcall(&py_str("i me my mine we our us myself your you it its he she they\n    there here that this"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_VERBISH(call_builtin("frozenset", &[mcall(&py_str("is are was were am be been being have has had do does did done go goes\n    going went gone plan planning planned visit visiting visited stay staying live living work\n    working like love hate want need get got take took make made book booked fly flying drive\n    driving rent rented see saw seen eat ate eaten drink drank cook cooked watch watched play\n    played read come came coming think thought know knew say said tell told try tried start\n    started stop stopped leave left move moved should would could will can may might must"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_VERBS(call_builtin("frozenset", &[mcall(&py_str("do give get go try keep sort call tell know feel be been have wait\n    stop start find leave come put use take see say think love like want"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_VERB_FORMS(bitor(&g_BARE_VERBS(), &call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&sub(&g_VERBISH(), &py_set(vec![py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("am"), py_str("be"), py_str("been"), py_str("being")]))) { let mut stem: Py = __item.clone(); if py_truthy(&{ let __b = gt(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { mcall(&stem.clone(), "isalpha", &[]) } }) {  __out.push(if py_truthy(&mcall(&stem.clone(), "endswith", &[py_str("e")])) { add(&slice_or_raise(&stem.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None), &py_str("ing")) } else { add(&stem.clone(), &py_str("ing")) }); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    s_VERB_THIRD(call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&bitor(&g_VERBISH(), &g_BARE_VERBS())) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = mcall(&word.clone(), "isalpha", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_set(vec![py_str("is"), py_str("are"), py_str("am"), py_str("be"), py_str("been"), py_str("being")]), &word.clone())) } }) {  __out.push(_third_person(&[word.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST_AUX(call_builtin("frozenset", &[mcall(&py_str("was were been had has have"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_INTEREST_HEAD(stdlib_call("re", "compile", &[py_str("^\\W*(?:i\\s*m|im|i am|i\\s*ve|ive|i have|we\\s*re|were|we are|i\\s+got|we\\s+got)\\s+(?:really\\s+|so\\s+|very\\s+much\\s+|very\\s+|well\\s+|quite\\s+|now\\s+|also\\s+|totally\\s+|still\\s+)?(?:(?:getting|got|started)\\s+(?:back\\s+)?|back\\s+)?(?:into|in to)\\s+(?P<thing>[a-z' -]+?)\\s*[.!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_INTEREST_NOT(call_builtin("frozenset", &[mcall(&py_str("not a an the this that these those some any my our his her their your\n    him her them us me you it fight fights argument arguments row rows trouble debt debts\n    hospital bed prison it him"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_INTEREST_TAIL(stdlib_call("re", "compile", &[py_str("\\s+(?:at the moment|right now|now|these days|lately|recently|a lot|lots|again|too|as well|also|on\\s+(?:mon|tues|wednes|thurs|fri|satur|sun)days?|every\\s+(?:day|week|weekend|morning|evening|night|mon|tues|wednes|thurs|fri|satur|sun)days?|at\\s+the\\s+weekend|after\\s+work|in\\s+the\\s+evenings?)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STARTED_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:got|getting|started)\\s+(?:back\\s+)?into\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BACK_INTO(stdlib_call("re", "compile", &[py_str("\\bback\\s+into\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WH_CONTINUES(call_builtin("frozenset", &[mcall(&py_str("which who whom whose that"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_SUBJECT(call_builtin("frozenset", &[add(&add(&add(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&add(&add(&g_GREETINGS(), &g_THANKS()), &g_FAREWELLS())) { let mut phrase: Py = __item.clone(); for __item in py_iter_or_raise(&mcall(&phrase.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }, &call_builtin("list", &[g_ACK()]).unwrap_or_else(|e| exc::raise(e))), &call_builtin("list", &[g_LAUGH()]).unwrap_or_else(|e| exc::raise(e))), &mcall(&py_str("sorry please oh ah well right yes no yeah yep nope hmm and so but also just"), "split", &[]))]).unwrap_or_else(|e| exc::raise(e)));
    s_SUBJECT_OF_ITS_OWN(bitor(&g_PRONOUNS(), &call_builtin("frozenset", &[mcall(&py_str("anyone someone everyone nobody anybody somebody one"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_NEGATORS(call_builtin("frozenset", &[mcall(&py_str("not no never neither nor hardly barely cannot cant dont doesnt didnt\n    isnt arent wasnt werent wont wouldnt shouldnt couldnt aint nothing"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NEGATION_SKIP(call_builtin("frozenset", &[mcall(&py_str("really very so too much quite pretty true honestly actually\n    totally completely absolutely seriously still even at all bit little kind sort feeling\n    feel felt a an the"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DENY_BETWEEN(bitor(&g_NEGATION_SKIP(), &call_builtin("frozenset", &[mcall(&py_str("in at on for under any more most all about much\n    having had havent been being be am is are was were got get getting a an the"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_DENY_BEFORE(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[g_NEGATORS()]).unwrap_or_else(|e| exc::raise(e))])), &py_str(")\\b(?:\\s+(?:")), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[g_DENY_BETWEEN()]).unwrap_or_else(|e| exc::raise(e))])), &py_str(")){0,4}\\s*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SUMMONS_WHO(call_builtin("frozenset", &[mcall(&py_str("\nteacher teachers head headteacher headmaster headmistress school nursery college tutor lecturer\nboss manager supervisor employer hr payroll council housing landlord lettings letting agency\ndwp benefits tax hmrc bank gp doctor surgery dentist hospital consultant midwife police\nprobation court solicitor immigration ofsted office\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SUMMONS_WHO_PHRASES(py_tuple(vec![py_str("social services"), py_str("health visitor"), py_str("home office"), py_str("tax office"), py_str("job centre"), py_str("jobcentre"), py_str("letting agent"), py_str("letting agents"), py_str("the council"), py_str("the school"), py_str("the office"), py_str("the bank"), py_str("the head")]));
    s_SUMMONS_SEE(py_tuple(vec![py_str("a word"), py_str("a quick word"), py_str("to see"), py_str("to talk"), py_str("a chat"), py_str("a meeting"), py_str("to meet"), py_str("to discuss"), py_str("to speak"), py_str("to come in"), py_str("asked me in"), py_str("called me in"), py_str("for a chat"), py_str("an appointment"), py_str("to interview")]));
    s_SUMMONS_WRITTEN(py_tuple(vec![py_str("written to me"), py_str("wrote to me"), py_str("sent me a letter"), py_str("sent a letter"), py_str("a letter from"), py_str("letter arrived"), py_str("letter came"), py_str("in writing")]));
    s_LETTER_RECEIVED(call_builtin("frozenset", &[mcall(&py_str("got received had came come arrives arrived"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_LETTER_ASKED_FOR(call_builtin("frozenset", &[mcall(&py_str("need needs wanted want wants asked ask asking sent send\n    sending wrote write writing expecting waiting chase chased requested requesting\n    wanting"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_LETTER_SOMEONE_ELSES(call_builtin("frozenset", &[mcall(&py_str("my his her their our"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_LETTER_ITSELF(call_builtin("frozenset", &[mcall(&py_str("letter letters note email reply"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SUMMONS_RANG(py_tuple(vec![py_str("rang me"), py_str("phoned me"), py_str("called me"), py_str("left a message"), py_str("rang about"), py_str("phoned about"), py_str("called about"), py_str("rang the house")]));
    s_SUMMONS_CALL_DONE(call_builtin("frozenset", &[mcall(&py_str("back today yesterday tonight morning afternoon evening\n    earlier already just then"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SUMMONS_SAID(call_builtin("frozenset", &[mcall(&py_str("said says told tells explained apologised apologized asked confirmed"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SUMMONS_WANTS(py_tuple(vec![py_str("want"), py_str("wants"), py_str("wanted"), py_str("needs"), py_str("needed"), py_str("asked"), py_str("asking"), py_str("would like"), py_str("requested"), py_str("told me")]));
    s_SUMMONS_VERB(bitor(&call_builtin("frozenset", &[g_SUMMONS_WANTS()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[mcall(&py_str("written wrote rang phoned called sent\n    letter message"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_SUMMONS_NOT(call_builtin("frozenset", &[mcall(&py_str("not hasnt havent didnt doesnt dont wont never no still yet"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STOMACH_WORDS(call_builtin("frozenset", &[mcall(&py_str("stomach stomachs tummy tummies belly bellies gut guts bowel bowels\n    insides"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_STATE_WORDS(call_builtin("frozenset", &[mcall(&py_str("upset"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BACK_AT(call_builtin("frozenset", &[mcall(&py_str("you yourself u urself ya"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BACK_JOIN(call_builtin("frozenset", &[mcall(&py_str("and about"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BACK_ASKING(call_builtin("frozenset", &[py_tuple(vec![py_str("what"), py_str("how")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BACK_NOT_A_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("me us him her them i we my our your"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BACK_AFTER(call_builtin("frozenset", &[mcall(&py_str("thanks cheers ta ok okay alright fine good great well grand sound\n    chill brill mint content lovely nice brilliant wonderful perfect amazing fantastic happy glad\n    relaxed calm not bad tired rough meh shattered knackered wrecked awful terrible so very\n    really doing feeling mate man\n    complain same always\n    working work busy off home out just finishing finishing up sorted done"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MEANT_APPLIED_TO(stdlib_call("re", "compile", &[py_str("\\bfor\\s+(?:my|our)\\s+([a-z'\\u2019-]+)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VERDICT_NOUNS(call_builtin("frozenset", &[mcall(&py_str("joke nightmare disaster shambles mess farce ripoff con\n    circus zoo mockery sham travesty letdown disappointment pain drag headache minefield\n    slog waste fiasco scandal pantomime horror"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_COPULAS(call_builtin("frozenset", &[mcall(&py_str("is are was were am be being been becomes became"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_VERDICT_FILLER(call_builtin("frozenset", &[mcall(&py_str("a an the total complete absolute utter right proper real massive\n    huge bloody bit of on one big fat"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_VERDICT_ADJECTIVES(call_builtin("frozenset", &[mcall(&py_str("useless pointless hopeless rubbish crap shit shite garbage\n    trash pathetic"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PRONOUN_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("it that this there he she they we i you"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKING_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("what which who whom whose how why when where whatever whoever\n    whichever whenever wherever"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_THING(call_builtin("frozenset", &[mcall(&py_str("week weekend day month year morning evening night afternoon term\n    shift holiday trip time"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PICKING(call_builtin("frozenset", &[mcall(&py_str("best better cheapest cheaper good great nice decent place places way ways\n    time times thing things option options spot spots area areas"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TO_A_VERB(stdlib_call("re", "compile", &[py_str("\\bto\\s+[a-z]+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAYS_WHAT_IT_IS(call_builtin("frozenset", &[mcall(&py_str("is are was were am im i'm its it's be been"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OPENS_A_FRAME(call_builtin("frozenset", &[mcall(&py_str("the a an any some and also best better cheapest cheaper good great\n    nice decent"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PICKING_QUALITY(call_builtin("frozenset", &[mcall(&py_str("best better cheapest cheaper good great nice decent"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ROUTINE_SHAPE(stdlib_call("re", "compile", &[py_str("\\b(?:every (?:day|week|month|morning|evening|afternoon|night|weekend|other day)|every (?:mon|tues|wednes|thurs|fri|satur|sun)day|each (?:day|week|morning|evening)|most (?:days|weeks|mornings|evenings)|(?:once|twice|three times|four times|a few times) a (?:day|week|month)|on (?:a|an) (?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)|at the weekend)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ROUTINE_IS_A_FAULT(stdlib_call("re", "compile", &[py_str("\\b(?:pain|pains|\\w*ache|\\w*aches|aching|hurts?|hurting|sore|sick|ill|unwell|worse|worst|worried|anxious|panic|cry|crying|cried|shout|shouting|shouted|argue|arguing|fight|fighting|bleed|bleeding|blood|swollen|rash|fever|cough|coughing|vomit|vomiting|dizzy|numb|sneeze|sneezing|stopped|stopping|wont stop|won't stop|leak|leaking|broken|broke|craving|crave)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ROUTINE_IN_THE_PAST(stdlib_call("re", "compile", &[py_str("\\b(?:used to|didn'?t use to|would (?:always |often )?\\w+|back then|years ago|in those days)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ROUTINE_SUBJECT(stdlib_call("re", "compile", &[py_str("^\\s*(?:and\\s+|so\\s+|well\\s+|oh\\s+)?(?:i|i(?:'m|ve|d)|we|we(?:'re|ve))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SHIFT_LATE(stdlib_call("re", "compile", &[py_str("\\b(?:(?:i|we|im|i'm|we're|were)\\s+)?(?:am|are|'m|'re|was|were)?\\s*(?:on|back on|working|work|doing|do)\\s+(?:the\\s+)?(?:lates|late shifts?)\\b|\\bon (?:the )?lates\\b|\\blates (?:all|this|next|every|for)\\b|\\blate shifts?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SHIFT_EARLY(stdlib_call("re", "compile", &[py_str("\\b(?:(?:i|we|im|i'm|we're|were)\\s+)?(?:am|are|'m|'re|was|were)?\\s*(?:on|back on|working|work|doing|do)\\s+(?:the\\s+)?(?:earlies|early shifts?)\\b|\\bon (?:the )?earlies\\b|\\bearlies (?:all|this|next|every|for)\\b|\\bearly shifts?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SHIFT_NIGHT(stdlib_call("re", "compile", &[py_str("\\b(?:night shifts?|on nights|work(?:ing)? nights|nights this week)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REPORTED_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:said|says|say|told|tells|tell|reckons?|reckoned|thinks?|thought|suggests?|suggested|recommends?|recommended|advises?|advised|insists?|insisted|keeps? (?:saying|telling me)|wants? me to|wanted me to|nagged? me to|pushing me to|going on at me to)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SUGGESTED_CLAUSE(stdlib_call("re", "compile", &[py_str("\\b(?:i|we) (?:should|shouldnt|shouldn't|ought|need|needed|have) (?:to )?|\\b(?:told|asked|tells|asks|wants?|wanted|nagged|pushing|pushed) me to\\b|\\b(?:i would|id|i'd) (?:like|love|rather|be better|better off)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TOLD_TO(stdlib_call("re", "compile", &[py_str("\\b(?:was|were|am|im|been|be|got) told to\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NOT_A_SUGGESTION(stdlib_call("re", "compile", &[py_str("\\b(?:shouldnt|shouldn't|should not|ought not|dont bother|don't bother|no point in|not worth (?:it|the|bothering)|wait and see|sit tight|hold off|hang on|leave it for now|think about it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SAID_BY(stdlib_call("re", "compile", &[py_str("\\b(?:my|the|a|an|our|his|her|their) (?:[\\w'-]+ ){0,2}?(?:mate|friend|mum|mom|mam|mother|dad|father|sister|brother|wife|husband|partner|son|daughter|gran|nan|grandma|grandad|neighbour|neighbor|colleague|coworker|boss|manager|doctor|gp|nurse|physio|physiotherapist|consultant|surgeon|midwife|dentist|pharmacist|therapist|counsellor|counselor|trainer|coach|teacher|tutor|landlord|lads|girls|bloke|fella|chap|someone|somebody)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SAID_BY_ANYONE(stdlib_call("re", "compile", &[py_str("\\b(?:everyone|everybody|people|they|he|she)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_HOME_PLACE(stdlib_call("re", "compile", &[py_str("\\b(?:in|into|at)\\s+(?:a|the)\\s+(?:care\\s+|nursing\\s+|residential\\s+)?home\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_HOME_NOT(stdlib_call("re", "compile", &[py_str("\\b(?:work|works|working|worked|stay|staying)\\s+from\\s+home\\b|\\bhome\\s+(?:town|office|made|birth|baked)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_HOME_KIN(call_builtin("frozenset", &[mcall(&py_str("nan nana mam mammy mate mates grannie grandaddy mother-in-law\n    father-in-law sister-in-law brother-in-law son-in-law daughter-in-law"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_HOME_MOVED(stdlib_call("re", "compile", &[py_str("\\b(?:moved|moves|moving|went|goes|gone|going|put|puts|putting|decided|decision|agreed|arranged|sorted)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_HOME_VISIT(stdlib_call("re", "compile", &[py_str("\\b(?:visit|visits|visiting|visited|see|seeing|saw|bring|brings|bringing|brought|take her|take him|take them)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_HOME_GUILT(stdlib_call("re", "compile", &[py_str("\\b(?:guilt|guilty|selfish|should have|shouldve|my fault|blame|couldnt cope|could not cope|no choice|heartbroken|awful)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_HOME_BRING_ASKS(stdlib_call("re", "compile", &[py_str("\\b(?:what|anything|something|whats|should i|shall i|can i|could i|do i|did i|is there anything)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_HOME_BRING_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:bring|brings|bringing|take|takes|taking)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_HOME_BRING_WHO(stdlib_call("re", "compile", &[py_str("\\b(?:her|him|them|she|he|they)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__MONEY_NOUN(stdlib_call("re", "compile", &[py_str("\\b(?:price[sd]?|pricing|cost[s]?|costing|charge[sd]?|fare[s]?|fee[s]?|rent|rental|salary|salaries|wage[s]?|expensive|cheaper|cheapest|cheap|how\\s+much\\s+money)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PRICE_ASK(stdlib_call("re", "compile", &[py_str("\\bhow\\s+much\\s+(?:is|are|was|were|will|would|does|do|did|should|can|could|might|is\\s+it|do\\s+they|do\\s+you|do\\s+i|does\\s+it)\\b|\\bprice[sd]?\\s+(?:of|for|on)\\b|\\bcost\\s+of\\b|\\bwhats?\\s+(?:the\\s+)?(?:price|cost|charge|fare|fee|rent)\\b|\\bwhat\\s+(?:is|are)\\s+the\\s+(?:price|cost|charge|fare|fee|rent)\\b|\\bwhat\\s+(?:does|do|will|would|should)\\b[^?]{0,30}?\\b(?:cost|charge)\\b|\\bwhats?\\s+(?:it|that|this|they|one)\\s+worth\\b|\\bhow\\s+much\\s+(?:money|for)\\b|\\bcheapest\\b|\\bdeals?\\s+on\\b|\\bon\\s+offer\\b|\\bdiscount(?:s|ed)?\\b|\\b(?:anything|something|any|some)\\s+(?:a\\s+bit\\s+|much\\s+)?cheaper\\b|\\bis\\s+(?:there|that|it)\\s+cheaper\\b|\\bcheaper\\s+(?:than|option|ones?|deal)\\b|\\bprice\\s+match\\b|\\b(?:cheaper|more\\s+expensive|dearer|pricier|costlier|better\\s+value)\\b|\\b(?:going|market|typical|average|usual|standard|ballpark)\\s+(?:rate|price|cost|fee|charge)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_POST_HOLDER(stdlib_call("re", "compile", &[py_str("\\bwho(?:s|\\s+is|\\s+are)?\\s+(?:the\\s+|our\\s+)?(?:prime\\s+minister|pm|president|chancellor|home\\s+secretary|first\\s+minister|mayor|mp|meps?|ceo|chief\\s+executive|head\\s+teacher|manager|coach|owner|chair(?:man|woman|person)?)\\b(?![^?]*\\b(?:of\\s+the\\s+year|ever|all\\s+time|alive|dead)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HOW_MUCH_I_SPENT(stdlib_call("re", "compile", &[py_str("\\bhow\\s+much\\s+(?:did|have|has|do)\\s+(?:i|we)\\s+(?:spend|spent|pay|paid|earn|earned|save|saved)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_MY_OWN_FIGURE(stdlib_call("re", "compile", &[py_str("\\b(?:am|do|will|would|should|have|did|can)\\s+i\\s+(?:be\\s+)?(?:paying|charged|billed|spending|getting\\s+charged)\\b|\\bmy\\s+(?:own\\s+)?(?:bill|bills|subscription|subscriptions|account|card|statement|tariff|contract|plan|direct\\s+debit|standing\\s+order|invoice|receipt|payslip|quota|allowance)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__MONEY(stdlib_call("re", "compile", &[py_str("\\b(?:dollars?|euros?|quid|cents?|yen|rupees?|pence|money|cash)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__UNITS(py_str("g|kg|kilos?|kilograms?|grams?|oz|ounces?|lbs?|pounds?|ml|l|millilit(?:re|er)s?|litres?|liters?|cups?|tbsp|tsp|tablespoons?|teaspoons?|celsius|fahrenheit|centigrade|degrees|stone|inches?|inch|cm|mm|metres?|meters?|feet|foot|ft|miles?|km|yards?|pints?|quarts?|gallons?"));
    s__UNITS_BOUGHT(py_str("g|kg|kilos?|kilograms?|grams?|oz|ounces?|lbs?|ml|l|millilit(?:re|er)s?|litres?|liters?|cups?|tbsp|tsp|tablespoons?|teaspoons?|celsius|fahrenheit|centigrade|degrees|stone|inches?|inch|cm|mm|metres?|meters?|feet|foot|ft|miles?|km|yards?"));
    s__A_AN_AMOUNT_OF_A_THING(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("\\b\\d+\\s*(?:"), &g__UNITS()), &py_str(")\\b\\s+(?:of|in)\\b|\\b(?:a|an|one|two|three|half|quarter)\\s+(?:")), &g__UNITS_BOUGHT()), &py_str(")\\s+of\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_A_CONVERSION(stdlib_call("re", "compile", &[add(&add(&py_str("\\bhow\\s+(?:much|many)\\b[^?]{0,30}?\\b(?:\\d+)?\\s*(?:"), &g__UNITS()), &py_str(")\\b\\s+(?:in|to|into)\\s+\\w"))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HOW_MUCH_OF_A_UNIT(stdlib_call("re", "compile", &[add(&add(&py_str("\\bhow\\s+much\\s+(?:is|are|was)\\s+(?:a\\s+|an\\s+|one\\s+)?[\\d\\w\\s]{0,12}?\\b(?:"), &g__UNITS_BOUGHT()), &py_str(")\\b(?!\\s+of\\b)"))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_THE_CLOCK(stdlib_call("re", "compile", &[py_str("\\bwhat\\s+time\\s+is\\s+it\\b|\\bwhats\\s+the\\s+time\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HOW_MUCH_IT_HOLDS(stdlib_call("re", "compile", &[py_str("\\bhow\\s+much\\s+(?:\\w+\\s+){0,2}(?:does|do|is|are|will|would|should|must|can)\\b[^?]{0,60}?\\b(?:weigh|weighs|hold|holds|contain|contains|measure|measures|store|stores|sleep|sleeps|eat|eats|drink|drinks|feed|feeds|lift|lifts|take|takes|need|needs|use|uses|burn|burns|grow|grows|train|exercise|practise|practice|study|water|feed|give|have|put|add)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HOW_MUCH_ENOUGH(stdlib_call("re", "compile", &[py_str("\\bhow\\s+much\\b[^?]{0,40}?\\b(?:enough|too\\s+much|too\\s+little|of\\s+it)\\b|\\bhow\\s+much\\s+of\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HOW_MUCH_IS_IN(stdlib_call("re", "compile", &[py_str("\\bhow\\s+much\\s+(?:is|are)\\s+(?:there\\s+)?left\\b|\\bhow\\s+much\\s+(?:is|are)\\s+(?:in|left\\s+in)\\b|\\bhow\\s+much\\s+(?:\\w+\\s+)?in\\s+(?:a|an|one|the|\\d)\\b|\\bhow\\s+much\\s+(?:\\w+\\s+){0,2}for\\s+(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|twenty)\\b[^?]{0,20}\\b(?:people|persons?|portions?|servings?|guests?|adults?|kids?)\\b|\\bhow\\s+much\\s+(?:\\w+\\s+){0,2}per\\s+(?:person|head|portion|serving)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HOW_MUCH_A_FEELING(stdlib_call("re", "compile", &[py_str("\\bhow\\s+much\\s+(?:do|does|did)\\s+(?:you|i|we|he|she|they)\\s+(?:love|care|miss)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_ARITHMETIC(stdlib_call("re", "compile", &[py_str("\\d\\s*(?:\\+|\\*|x|/|plus|minus|times|divided\\s+by|multiplied\\s+by)\\s*\\d|\\d\\s+-\\s+\\d|\\b(?:percent|per\\s?cent)\\b|\\d\\s*%|\\bwhats\\s+\\d+\\s*%|\\b(?:plus|minus|divided\\s+by|multiplied\\s+by)\\s+(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|twenty|thirty|fifty|hundred|thousand)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HAS_A_NUMBER(stdlib_call("re", "compile", &[py_str("\\d|\\b(?:one|two|three|four|five|six|seven|eight|nine|ten)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_A_DIFFERENCE(stdlib_call("re", "compile", &[py_str("\\b(?:whats|what\\s+is|what\\s+was)\\s+the\\s+difference\\b|\\bdifference\\s+between\\b|\\bhow\\s+(?:do|does)\\s+\\w+\\s+differ\\b|\\bhow\\s+(?:is|are|was|were)\\b[^?]{0,40}?\\b(?:different|differs?|distinct)\\s+(?:from|to|than)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_DID_YOU_SEE(stdlib_call("re", "compile", &[py_str("\\bdid\\s+you\\s+(?:see|hear|watch|read|catch|catch\\s+up\\s+on)\\b|\\bhave\\s+you\\s+(?:seen|heard|watched|read)\\b|\\bdid\\s+you\\s+know\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_BARE_RECORD(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|so|but|then)\\s+)?(?:the|whats\\s+the|what\\s+is\\s+the|which\\s+is\\s+the)?\\s*(?:biggest|largest|tallest|longest|smallest|deepest|highest|oldest|fastest|richest|most\\s+populous)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PAST_YEAR(stdlib_call("re", "compile", &[py_str("\\b(?:back\\s+)?in\\s+(?:1[0-9]{3}|20[0-2][0-9])\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HISTORY_ITSELF(stdlib_call("re", "compile", &[py_str("\\b(?:first|second|third|1st|2nd|world)\\s+world\\s+war\\b|\\bworld\\s+war\\s+(?:one|two|1|2|ii)\\b|\\bthe\\s+(?:great\\s+)?war\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HOW_A_THING_MEASURES(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:hot|cold|warm|deep|tall|high|heavy|big|large|long|wide|fast|strong)\\s+(?:is|are|was|were)\\s+(?!it\\b|outside\\b|today\\b|tonight\\b|this\\b|the\\s+weather\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_A_TRAFFIC_LIGHT(stdlib_call("re", "compile", &[py_str("\\btraffic\\s+lights?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_A_PLAN_ABOUT_THE_WEATHER(stdlib_call("re", "compile", &[py_str("\\b(?:the\\s+)?weather\\s+(?:plan|plans|permitting|dependent|depending)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_WEATHER_HERE(stdlib_call("re", "compile", &[py_str("\\b(?:is|will)\\s+it\\s+(?:be\\s+)?(?:going\\s+to\\s+)?(?:rain|snow|sunny|windy|hot|cold)\\b|\\bhow\\s+(?:hot|cold|warm|windy)\\s+(?:is|will)\\s+it\\b|\\b(?:today|tonight|tomorrow|this\\s+(?:week|weekend|morning|afternoon|evening)|outside|right\\s+now|at\\s+the\\s+moment)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HOW_OLD_IS_THE_WORLD(stdlib_call("re", "compile", &[py_str("\\bhow\\s+old\\s+is\\s+the\\s+(?:earth|world|universe|planet|sun|moon)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__MONEY_ASKED(stdlib_call("re", "compile", &[py_str("\\bhow\\s+much\\s+(?:is|are|was|were|will|would|does|do)\\b(?!\\s+(?:too|enough|of)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s__MONEY_FOR(stdlib_call("re", "compile", &[py_str("\\bhow\\s+much\\s+for\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PLACE_THAT_KEEPS_HOURS(stdlib_call("re", "compile", &[py_str("\\b(?:shops?|stores?|supermarkets?|gyms?|pools?|librar(?:y|ies)|chemists?|pharmac(?:y|ies)|surger(?:y|ies)|doctors?|dentists?|banks?|garages?|stations?|offices?|centres?|centers?|clinics?|cafes?|pubs?|restaurants?|takeaways?|chippys?|markets?|museums?|halls?|churches?|schools?|colleges?|tips?|dumps?|post\\s+office|leisure\\s+centre|swimming\\s+pool|garden\\s+centre|car\\s+park)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_LINE_THAT_NAMES_A_PLACE(add(&add(&py_str("(?=[^\\n]*"), &gattr(&g__A_PLACE_THAT_KEEPS_HOURS(), "pattern")), &py_str(")")));
    s__NOT_A_FITTING_THAT_OPENS(py_str("(?![^\\n]*\\b(?:doors?|gates?|windows?|lids?|drawers?|eyes?|mouth|flaps?|valves?|caps?|tins?|jars?|boxes?)\\s+(?:open|opens|closed|closes|shut|shuts)\\b)"));
    s__A_DAY_WORD(py_str("(?:today|tonight|tomorrow|now|this\\b|on\\b|at\\b|until\\b|till\\b|late\\b|for\\s+the\\s+day|,)"));
    s__NOT_OPEN_TO_SOMEBODY(py_str("(?![^\\n]*\\b(?:open|opens)\\s+(?:to|for)\\s+(?!(?:today|tonight|tomorrow|now|late|the\\s+day|the\\s+weekend|the\\s+summer|christmas|easter)\\b))"));
    s__HOURS(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&py_str("\\bopening\\s+(?:hours?|times?)\\b|\\bclosing\\s+time\\b|\\blate\\s+opening\\b|\\bbank\\s+holiday\\s+hours\\b|\\b(?:open|opens|closed?|closes|shuts?)\\s+(?:until|till|at|on|late|early)\\b|"), &g__A_LINE_THAT_NAMES_A_PLACE()), &g__NOT_A_FITTING_THAT_OPENS()), &g__NOT_OPEN_TO_SOMEBODY()), &py_str("\\b(?:is|are|was|were|does|do|will)\\s+(?:[\\w']+\\s+){0,4}?(?:open|opens)\\s+(?:today|tonight|tomorrow|now|this\\s+\\w+)\\b|")), &g__A_LINE_THAT_NAMES_A_PLACE()), &g__NOT_A_FITTING_THAT_OPENS()), &g__NOT_OPEN_TO_SOMEBODY()), &py_str("\\b(?:is|are|was|were)\\s+(?:[\\w']+\\s+){0,3}?(?:open|opens|closed|closes|shut|shuts)\\b(?=\\s*")), &g__A_DAY_WORD()), &py_str(")|")), &g__NOT_A_FITTING_THAT_OPENS()), &g__NOT_OPEN_TO_SOMEBODY()), &py_str("\\b(?:is|are|was|were|does|do)\\s+(?:the\\s+|a\\s+)?(?:\\w+\\s+){0,2}?")), &gattr(&g__A_PLACE_THAT_KEEPS_HOURS(), "pattern")), &py_str("\\s+(?:open|opens|closed|closes|shut|shuts)\\b|")), &g__NOT_A_FITTING_THAT_OPENS()), &g__NOT_OPEN_TO_SOMEBODY()), &gattr(&g__A_PLACE_THAT_KEEPS_HOURS(), "pattern")), &py_str("\\s+(?:\\w+\\s+){0,2}?(?:open|opens|closed|closes|shut|shuts)\\b(?=\\s*")), &g__A_DAY_WORD()), &py_str(")|")), &g__NOT_OPEN_TO_SOMEBODY()), &py_str("\\b(?:is|are|was|were)\\s+(?:it|they|there(?:\\s+anywhere)?|(?:the|this|that|my|our|your|any)\\s+\\w+|\\w+)\\s+open\\b|\\bwhat\\s+times?\\s+(?:does|do|is|are|will)\\b|\\b(?:when|what\\s+times?)\\b[^?]{0,30}?\\b(?:open|opens|close|closes|closed|shut|shuts)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s__WEATHER(stdlib_call("re", "compile", &[py_str("\\bweather\\b|\\bforecast\\b|\\bumbrella\\b|\\bhow\\s+(?:hot|cold|warm|windy)\\s+(?:is|will)\\s+it\\b|\\b(?:is|will)\\s+it\\s+(?:be\\s+)?(?:going\\s+to\\s+)?(?:rain|snow|be\\s+sunny|be\\s+windy|be\\s+stormy|be\\s+hot|be\\s+cold)\\b|\\bit\\s+(?:is|will|should|might)\\s+(?:be\\s+)?(?:rain|snow|sunny|windy|stormy)\\b|\\bheat\\s?wave\\b|\\bcold\\s+snap\\b|\\btemperature\\s+(?:today|tomorrow|outside)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__NEWS(stdlib_call("re", "compile", &[py_str("\\bnews\\b|\\bheadlines?\\b|\\b(?:whats|what\\s+is|what\\s+was|anything|any|the\\s+latest|latest|breaking|got\\s+any)\\s+(?:the\\s+)?(?:news|headlines?)\\b|\\b(?:latest|update[s]?)\\s+(?:on|about)\\b|\\b(?:news|press|newspaper)\\s+stor(?:y|ies)\\s+(?:on|about)\\b|\\b(?:the|any|more)\\s+word\\s+(?:on|about)\\b|\\bwhats\\s+going\\s+on\\s+(?:in|at|with)\\s+the\\s+(?:world|news|country|city)\\b|\\bwhats\\s+(?:happened|happening)\\b|\\bwhat\\s+(?:is|was)\\s+happening\\b|\\belections?\\s+(?:today|tonight|this\\s+(?:week|year)|results?)\\b|\\bwho\\s+won\\b|\\bwho(?:s|\\s+is)\\s+winning\\b|\\bdid\\s+(?:\\w+\\s+){0,3}win\\b|\\bhas\\s+(?:\\w+\\s+){0,3}(?:won|finished|played)\\b|\\bthe\\s+score\\b|\\bwhats\\s+the\\s+score\\b|\\bleague\\s+table\\b|\\bthe\\s+table\\b|\\bmatch\\s+result\\b|\\b(?:the\\s+)?results?\\s+(?:of|from)\\s+the\\s+(?:match|game)\\b|\\btransfers?\\s+(?:news|today|latest)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__AVAILABILITY(stdlib_call("re", "compile", &[py_str("\\b(?:in\\s+stock|stock\\s+levels?|available|availability|unavailable|sold\\s+out|vacanc(?:y|ies)|hiring|waiting\\s+list|bookings?\\s+(?:open|available|left|for)|(?:make|get|book|check|change|cancel|move|confirm)\\s+(?:a\\s+|my\\s+|the\\s+)?booking\\b|(?:tickets?|places?|spaces?|seats?|rooms?|jobs?|appointments?)\\s+(?:\\w+\\s+){0,2}(?:left|going|available|free)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s__TIMETABLE(stdlib_call("re", "compile", &[py_str("\\b(?:timetables?|next\\s+(?:train|bus|flight|boat|ferry|coach)|departures?|arrivals?|delay(?:s|ed)?|roadworks?|bus\\s+times?|train\\s+times?|flight\\s+times?|traffic\\s+(?:report|news|today|now|on\\s+the)|\\bare\\s+(?:the\\s+)?(?:trains|buses|tubes|flights|ferries|trams)\\s+running\\b|\\bis\\s+the\\s+(?:m\\d+|a\\d+|motorway|road|bridge|tunnel)\\s+(?:closed|open|blocked)\\b|what\\s+time\\s+(?:is|does)\\s+the\\s+(?:train|bus|flight))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_SERVICE(stdlib_call("re", "compile", &[py_str("\\b(?:trains?|buses?|bus|trams?|tubes?|underground|metro|flights?|ferry|ferries|coach(?:es)?|platform|departures?|arrivals?)\\b|\\bthe\\s+\\d{1,3}(?:[:.]\\d{2})?(?=\\s*(?:running|run|runs|leaves|leaving|departs?|departing|due|arrives?|arriving|go|goes|going|take|takes|train|bus|flight))")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_DAY_OR_TIME(stdlib_call("re", "compile", &[py_str("\\b(?:today|tonight|tomorrow|yesterday|this\\s+(?:morning|afternoon|evening|weekend|week)|next\\s+(?:week|weekend)|monday|tuesday|wednesday|thursday|friday|saturday|sunday|weekends?|bank\\s+holiday|christmas|easter)\\b|\\b\\d{1,2}[:.]\\d{2}\\b|\\bo'?clock\\b|\\b\\d{1,2}\\s?(?:am|pm)\\b|\\bhalf\\s+past\\b|\\bhow\\s+often\\b")]).unwrap_or_else(|e| exc::raise(e)));
    register_class_method("_AServiceOnADay", "__init__", py_func("_AServiceOnADay.__init__", |args: &[Py]| _AServiceOnADay____init__(args)));
    register_class_method("_AServiceOnADay", "search", py_func("_AServiceOnADay.search", |args: &[Py]| _AServiceOnADay__search(args)));
    s__A_SERVICE_STATE(stdlib_call("re", "compile", &[py_str("\\b(?:trains?|buses?|bus|flights?|coach(?:es)?|ferry|ferries|trams?)\\b.{0,40}?\\b(?:late|delayed|cancell?ed|on\\s+time|on\\s+strike|diverted|full\\s+up)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__SERVICE_ON_A_DAY(_AServiceOnADay(&[g__A_SERVICE(), g__A_DAY_OR_TIME()]));
    s__RATE(stdlib_call("re", "compile", &[py_str("\\b(?:interest\\s+rate[s]?|exchange\\s+rate[s]?|tax\\s+rate[s]?|inflation|mortgage\\s+rate[s]?|minimum\\s+wage|price\\s+cap|energy\\s+cap|national\\s+insurance\\s+(?:rate|contribution[s]?|percent|%)|stamp\\s+duty|vat|base\\s+rate)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__EVENT(stdlib_call("re", "compile", &[py_str("\\b(?:whats\\s+on|what\\s+is\\s+on|anything\\s+on(?!\\s+(?:a|an|the|my|your|about|for)\\b)|events?\\s+on|gig\\s+guide|show\\s+times?|cinema\\s+times?|fixtures?|opening\\s+night|when\\s+is\\s+the\\s+\\w+\\s+on\\b)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_FRESHENING(stdlib_call("re", "compile", &[py_str("\\b(?:latest|current|newest|most\\s+recent|up[\\s-]to[\\s-]date)\\s+(?!(?:on|about|of|in|from|at|with)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s__CHANGING(py_tuple(vec![py_tuple(vec![py_str("rate"), g__RATE()]), py_tuple(vec![py_str("price"), g__A_PRICE_ASK()]), py_tuple(vec![py_str("post"), g__A_POST_HOLDER()]), py_tuple(vec![py_str("timetable"), g__SERVICE_ON_A_DAY()]), py_tuple(vec![py_str("timetable"), g__A_SERVICE_STATE()]), py_tuple(vec![py_str("timetable"), g__TIMETABLE()]), py_tuple(vec![py_str("hours"), g__HOURS()]), py_tuple(vec![py_str("weather"), g__WEATHER()]), py_tuple(vec![py_str("news"), g__NEWS()]), py_tuple(vec![py_str("availability"), g__AVAILABILITY()]), py_tuple(vec![py_str("event"), g__EVENT()])]));
    s__A_HOW_IT_WORKS(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:do|does|did)\\b|\\bwhy\\s+(?:do|does|did)\\b|\\bwhat\\s+causes?\\b|\\bwhat\\s+caused\\b|\\bhow\\s+come\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_MY_OWN_PLAN(stdlib_call("re", "compile", &[py_str("\\b(?:when|what\\s+time)\\s+(?:(?:should|shall|do|does|can|could|will|would|must|might|had\\s+better)\\s+)?(?:i|we|you)\\b(?!\\s+(?:open|opens|close|closes|shut|shuts|serve|serves|trade|trades)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_AN_OCCASION_OF_THEIR_OWN(stdlib_call("re", "compile", &[py_str("\\b(?:what|which)\\s+time\\s+(?:is|are|was|were|does|do|did|will|would)\\b.{0,25}?\\b(?:funeral|wake|meeting|appointment|interview|scan|operation|procedure|checkup|check-up|assessment|induction|shift|session|court|hearing|viewing)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_SAYS_NOW(stdlib_call("re", "compile", &[py_str("\\b(?:today|tonight|tomorrow|right\\s+now|at\\s+the\\s+moment|this\\s+(?:morning|afternoon|evening|week|weekend)|currently|still)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HOW_MANY(stdlib_call("re", "compile", &[py_str("\\bhow\\s+many\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_COUNT(stdlib_call("re", "compile", &[py_str("\\bhow\\s+many\\b|\\bhow\\s+far\\s+is\\b|\\bhow\\s+long\\s+is\\b|\\bhow\\s+much\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_RECORD(stdlib_call("re", "compile", &[py_str("\\bthe\\s+(?:biggest|largest|tallest|oldest|youngest|longest|shortest|highest|lowest|deepest|shallowest|fastest|slowest|smallest|richest|poorest|wealthiest|heaviest|lightest|widest|narrowest|thinnest|strongest|busiest|rarest|commonest|most\\s+common|most\\s+popular|most\\s+\\w+|least\\s+\\w+)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_DEFINITION(stdlib_call("re", "compile", &[py_str("\\bwhat\\s+(?:is|are|was|were)\\s+(?:(?:a|an|the)\\s+)?(?!(?:latest|newest|current|best|worst|main|right|usual|cheapest|nearest|the|a|an|it|that|this|they|these|those)\\b)\\w+|\\bwhat\\s+does\\s+\\w+\\s+mean\\b|\\bwhat\\s+is\\s+the\\s+meaning\\b|\\bwhat\\s+does\\s+(?:a|an|the)?\\s*\\w+\\s+do\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_WORKING(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:does|do)\\b.{0,40}\\bwork\\b|\\bwhy\\s+(?:does|do|is|are|did)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HISTORY(stdlib_call("re", "compile", &[py_str("\\bwhen\\s+(?:was|were|did)\\b.*\\b(?:built|invented|discovered|founded|written|composed|painted|made|begin|began|start|end|ended|happen|happened|become)\\b|\\bwho\\s+(?:invented|discovered|founded|wrote|composed|built|painted)\\b|\\bhistory\\s+of\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_MEASURE(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:tall|high|deep|wide|long|heavy|big|far)\\s+is\\b|\\bhow\\s+far\\s+away\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_MADE_OF(stdlib_call("re", "compile", &[py_str("\\bmade\\s+(?:of|from|out\\s+of)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_METHOD(stdlib_call("re", "compile", &[py_str("\\bhow\\s+to\\b|\\bhow\\s+(?:do|can|should)\\s+(?:i|you|we)\\b|\\bhow\\s+long\\s+(?:do|does|should|to)\\s+.*\\b(?:cook|boil|bake|roast|keep|last|take|soak|chill|freeze)\\b|\\bhow\\s+often\\s+should\\s+i\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_WORD_MEANS(stdlib_call("re", "compile", &[py_str("\\bwhat\\s+(?:does|do|did)\\b.{1,40}?\\bmean\\b\\s*[?!.]*\\s*$|\\bwhats?\\s+(?:is\\s+)?the\\s+meaning\\s+of\\b|\\bwhat\\s+is\\s+(?:the\\s+)?meaning\\b|\\bwhat\\s+is\\s+the\\s+definition\\s+of\\b|\\bwhat\\s+do\\s+you\\s+mean\\s+by\\b|\\bdefine\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__SETTLED(py_tuple(vec![g__A_COUNT(), g__A_RECORD(), g__A_DEFINITION(), g__A_WORKING(), g__A_HISTORY(), g__A_MEASURE(), g__A_MADE_OF(), g__A_METHOD()]));
    s__SEALED_ASKS(call_builtin("frozenset", &[py_tuple(vec![py_str("maths")])]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS_WITH_A_QUESTION_WORD(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|but|so|then|also|ok|okay|right|oh|well|now|sorry|ta|hi|hello|excuse\\s+me)\\s+)*(?:what|whats|what's|when|whens|where|which|who|whose|why|how|hows|how's)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS_MID_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|do|does|did|can|could|will|would|shall|should|am|have|has|had)\\s+(?:i|we|you|they)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS_AFTER_A_COMMA(stdlib_call("re", "compile", &[py_str(",\\s*(?:(?:and|but|so|then|also)\\s+)?(?:is|are|was|were|do|does|did|can|could|will|would|shall|should|am|have|has|had)\\s+(?:i|you|we|they|it|he|she|there|the\\s+\\w+)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS_AFTER_A_COMMA_WORD(stdlib_call("re", "compile", &[py_str(",\\s*(?:(?:and|but|so|then|also)\\s+)?(?:what|whats|when|whens|where|which|who|why|how|hows)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS_WITH_AN_AUXILIARY(stdlib_call("re", "compile", &[py_str("^(?:is|are|was|were|do|does|did|can|could|will|would|shall|should|has|have|had|any|anything|anyone|anyway|tell)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_REPORTS_THE_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|but|so|then|also|well|oh|anyway)\\s+)*(?:i|we|they|he|she|everyone|someone)\\s+(?:already\\s+|still\\s+|just\\s+|now\\s+|never\\s+|definitely\\s+)?(?:know|knows|knew|told|tell|tells|said|says|explained|explains|mentioned|mentions|remember|remembers|reminded|found\\s+out)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__BARE_LIVE_LEAD(py_str("^\\s*(?:(?:and|so|but|then|also|ok|okay|right|oh)\\s+|(?:what|how)\\s+about\\s+)?"));
    s__A_BARE_LIVE(py_tuple(vec![py_tuple(vec![py_str("news"), stdlib_call("re", "compile", &[add(&g__BARE_LIVE_LEAD(), &py_str("(?:the\\s+|any\\s+|got\\s+any\\s+)?(?:news|headlines?)\\b"))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("hours"), stdlib_call("re", "compile", &[add(&g__BARE_LIVE_LEAD(), &py_str("(?:the\\s+|any\\s+)?(?:opening\\s+(?:hours|times)|opening|closing\\s+time|bank\\s+holiday\\s+hours)\\b"))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("timetable"), stdlib_call("re", "compile", &[add(&g__BARE_LIVE_LEAD(), &py_str("(?:the\\s+|any\\s+)?(?:delays?|roadworks?|traffic|bus\\s+times?|train\\s+times?|departures|arrivals|timetable)\\b"))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("weather"), stdlib_call("re", "compile", &[add(&g__BARE_LIVE_LEAD(), &py_str("(?:the\\s+|any\\s+)?(?:weather|forecast)\\b"))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("price"), stdlib_call("re", "compile", &[add(&g__BARE_LIVE_LEAD(), &py_str("(?:the\\s+|any\\s+)?(?:price|prices|cost|costs|fares?|ticket\\s+prices?)\\b"))]).unwrap_or_else(|e| exc::raise(e))])]));
    s__A_REPORTS(stdlib_call("re", "compile", &[py_str("\\b(?:looks?|looked|seems?|seemed|sounds?|sounded|feels?|felt|was|were|is|are|has|have|had|said|says|told|hear|heard|got|get|gets|makes?|made|turns?|turned|went|goes|coming|comes|hit|hits|does|doesnt|did|didnt|wont|cant|helps?|ruined|spoils?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_BARE_ASK(Py::Int(4));
    s__A_BARE_LIVE_WORDS(Py::Int(9));
    s__A_FOLLOW_UP_WORDS(Py::Int(7));
    s__A_BARE_TAIL(stdlib_call("re", "compile", &[py_str("^(?:for|on|at|in|this|next|today|tonight|tomorrow|now|here|there|please|pls|thanks|ta|anyone|anywhere)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_DETERMINER(call_builtin("frozenset", &[py_tuple(vec![py_str(""), py_str("the"), py_str("a"), py_str("an"), py_str("any"), py_str("your"), py_str("this"), py_str("our"), py_str("these")])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _asks_for_help(args: &[Py]) -> Py {
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
    let mut tail: Py = Py::None;
    let _ = py_str("True when the line asks for help in a shape that does not open like a question.\n\n    \"anything i can do about it\", \"any way of making it easier\", \"anything that helps\". The ask\n    has to be the line's own question, so a negation before it (\"there isnt anything i can do\")\n    or a copula after it (\"anything i can do is never enough\") leaves the line a statement.\n    ");
    if py_truthy(&mcall(&g_HELP_ASK_OWN_SUBJECT(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_HELP_ASK(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&mcall(&g_HELP_ASK_NOT(), "search", &[slice_or_raise(&text.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None)])) {
    continue;
    }
    tail = slice_or_raise(&text.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None);
    if py_truthy(&{ let __b = mcall(&g_HELP_ASK_SUBJECT(), "search", &[tail.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_HELP_ASK_TAIL_CLAUSE(), "search", &[tail.clone()]) } }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _asks_a_verdict(args: &[Py]) -> Py {
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
    let mut flat: Py = Py::None;
    let mut start: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line asks for a judgement of the thing just said.\n\n    The verdict is usually the whole line (\"is that bad\"), and the cap below is what keeps a line\n    with a subject of its own out. But the same question is asked after the thing it is about, and\n    that line carries no reading at all by the whole-line form: \"my resting heart rate is 90 is\n    that bad\", \"i left the chicken out all night is it safe\", \"they said two weeks is that normal\"\n    (48 of the deck's 60,260 turns got no shape of their own this way, and 18 more got a wrong one).\n    The tail is read by the same pattern, and the cap stays on the *verdict clause* rather than on\n    the line, so a short statement in front of the verdict does not hide it.\n    ");
    flat = stdlib_call("re", "sub", &[py_str("[^a-z0-9 ]+"), py_str(" "), mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    flat = mcall(&stdlib_call("re", "sub", &[py_str("\\s+"), py_str(" "), flat.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&flat.clone()))) {
    return Py::Bool(false);
    }
    words = mcall(&flat.clone(), "split", &[]);
    if py_truthy(&gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(30))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_is(&mcall(&g_VERDICT_ASK(), "fullmatch", &[flat.clone()]), &Py::None))) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(1), sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    start = __item.clone();
    if py_truthy(&gt(&sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &start.clone()), &Py::Int(7))) {
    continue;
    }
    if py_truthy(&mcall(&g_QUESTION_WORD(), "search", &[mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::None, &start.clone(), &Py::None)])])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_is(&mcall(&g_VERDICT_ASK(), "fullmatch", &[mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &start.clone(), &Py::None, &Py::None)])]), &Py::None))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _claim_forms(args: &[Py]) -> Py {
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
    let mut _suffix: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut form: Py = Py::None;
    let mut forms: Py = Py::None;
    let _ = py_str("The stems a word could be, so \"changed\", \"changes\" and \"change\" all answer to one.");
    forms = py_set(vec![word.clone()]);
    let __iter1 = py_iter_or_raise(&g_CLAIM_ENDS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    cut = __unpk2[0].clone();
    _suffix = __unpk2[1].clone();
    if py_truthy(&gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &add(&cut.clone(), &Py::Int(2)))) {
    let _ = mcall(&forms.clone(), "add", &[slice_or_raise(&word.clone(), &Py::None, &neg(&cut.clone()), &Py::None)]);
    }
    }
    if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("ing")])) {
    let _ = mcall(&forms.clone(), "add", &[add(&slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(3)), &Py::None), &py_str("e"))]);
    }
    let __iter3 = py_iter_or_raise(&call_builtin("list", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    form = __item.clone();
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[form.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&form.clone(), &neg(&Py::Int(1))), &geti(&form.clone(), &neg(&Py::Int(2))))) } }) {
    let _ = mcall(&forms.clone(), "add", &[slice_or_raise(&form.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)]);
    }
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&forms.clone()) { let mut form: Py = __item.clone(); __out.push(if py_truthy(&{ let __b = mcall(&form.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[form.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) { slice_or_raise(&form.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None) } else { form.clone() }); } py_set(__out) })() };
    Py::None
}
pub fn names_a_claim(args: &[Py]) -> Py {
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
    let mut form: Py = Py::None;
    let _ = py_str("True when the word after \"nothing\" is the verb of a clause with its object left out.");
    word = mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&g_A_THING_NOT_A_CLAIM(), &word.clone()))) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_claim_forms(&[word.clone()])) { let mut form: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_CLAIM_STEMS(), &form.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_class_of_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut members: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words of a class a refusal can be handed back inside (\"not a thriller\", \"not tablets\").");
    if py_truthy(&Py::Bool(!py_truthy(&g__A_CLASS_OF_THING()))) {
    let mut RULED_OUT_CLASS: Py = crate::gen::compose::ruledout::g_RULED_OUT_CLASS();
    let mut RULED_OUT_INSTANCES: Py = crate::gen::compose::ruledout::g_RULED_OUT_INSTANCES();
    words = call_builtin("set", &[RULED_OUT_CLASS.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&RULED_OUT_INSTANCES.clone(), "values", &[]));
    for __item in __iter1 {
    members = __item.clone();
    let __aug2 = bitor(&words.clone(), &call_builtin("set", &[members.clone()]).unwrap_or_else(|e| exc::raise(e)));
    words = __aug2;
    }
    let _ = mcall(&g__A_CLASS_OF_THING(), "append", &[call_builtin("frozenset", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    }
    return geti(&g__A_CLASS_OF_THING(), &Py::Int(0));
    Py::None
}
pub fn _ruled_out_of(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut gone: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut said_before: Py = Py::None;
    let mut same_thing: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The words of a thing the line says it does not mean, for the reader to skip.\n\n    \"for a holiday not a house\" hands over [\"house\"]; \"a laptop not a tablet\" hands over\n    [\"tablet\"]. A cue with no article in front of it is a verb being denied (\"i could not\n    sleep\"), not a thing ruled out, so nothing comes of it, and a cue with nothing in front of it\n    but the ask rules nothing out: \"recommend a film\" / \"not a series\" is a question about films,\n    and its reader needs the word \"series\" to tell one from the other.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return py_list(vec![]);
    }
    gone = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g_RULED_OUT(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    said_before = slice_or_raise(&text.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None);
    before = slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&said_before.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(" .,!?;:'’")]), "lower", &[])); } py_list(__out) })() }, &neg(&Py::Int(3)), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&before.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_RULED_OUT_NO_THING(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() }))) {
    continue;
    }
    if py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&_a_class_of_thing(&[]), &mcall(&word.clone(), "lower", &[])))) {  __out.push(word.clone()); } } py_list(__out) })() }) {
    continue;
    }
    same_thing = Py::Bool(py_contains(&py_tuple(vec![py_str("one"), py_str("ones")]), &mcall(&geti(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "split", &[]), &neg(&Py::Int(1))), "lower", &[])));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&same_thing.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_RULED_OUT_CORRECTION(), "search", &[said_before.clone()]))) } }) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "split", &[]));
    for __item in __iter2 {
    word = __item.clone();
    word = mcall(&word.clone(), "strip", &[py_str(" .,!?;:'’“”")]);
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    break;
    }
    if py_truthy(&Py::Bool(py_contains(&g_RULED_OUT_ENDS(), &mcall(&word.clone(), "lower", &[])))) {
    break;
    }
    let _ = mcall(&gone.clone(), "append", &[word.clone()]);
    if py_truthy(&ge(&call_builtin("len", &[gone.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    break;
    }
    }
    }
    return gone.clone();
    Py::None
}
pub fn _limit_of(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut _name: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut before: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut shorter: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing the line rules out, or None when the line says more than that.\n\n    The whole line has to be the limit, with only the softeners people put round one. \"no\n    horror\" is a limit; \"no horror but i liked the last one\" is a line of its own, and \"no\n    thanks\" rules out nothing at all.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::None;
    }
    ask = mcall(&mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "replace", &[py_str("|"), py_str(" ")]), "split", &[])]), "strip", &[py_str(" .,!?")]), "lower", &[]);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(3)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    _u = __item.clone();
    shorter = mcall(&g_LIMIT_SOFT_BEFORE(), "sub", &[py_str(""), ask.clone()]);
    if py_truthy(&Py::Bool(!py_eq(&shorter.clone(), &ask.clone()))) {
    ask = shorter.clone();
    }
    shorter = mcall(&mcall(&g_LIMIT_SOFT_AFTER(), "sub", &[py_str(""), ask.clone()]), "strip", &[py_str(" .,!?")]);
    if py_truthy(&Py::Bool(!py_eq(&shorter.clone(), &ask.clone()))) {
    ask = shorter.clone();
    }
    if py_truthy(&Py::Bool(py_eq(&ask.clone(), &mcall(&mcall(&g_LIMIT_SOFT_BEFORE(), "sub", &[py_str(""), mcall(&g_LIMIT_SOFT_AFTER(), "sub", &[py_str(""), ask.clone()])]), "strip", &[py_str(" .,!?")])))) {
    break;
    }
    }
    let __iter2 = py_iter_or_raise(&g_LIMIT_SHAPES());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    _name = __unpk3[0].clone();
    shape = __unpk3[1].clone();
    match_ = mcall(&shape.clone(), "search", &[ask.clone()]);
    if py_truthy(&Py::Bool(py_is(&match_.clone(), &Py::None))) {
    continue;
    }
    thing = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "split", &[])]), "strip", &[py_str(" .,!?")]);
    words = mcall(&thing.clone(), "split", &[]);
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("one"), py_str("ones")]), &geti(&words.clone(), &neg(&Py::Int(1))))) } }) {
    thing = mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)]);
    words = mcall(&thing.clone(), "split", &[]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_LIMIT_ROOM()) } }) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_LIMIT_NOT_A_THING(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_LIMIT_REFUSAL(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_LIMIT_NOT_A_START(), &geti(&words.clone(), &Py::Int(0))))) {
    continue;
    }
    if py_truthy(&mcall(&g_PAST_ENDS(), "match", &[geti(&words.clone(), &neg(&Py::Int(1)))])) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&ask.clone(), "startswith", &[py_str("nothing")]); if !py_truthy(&__b) { __b } else { names_a_claim(&[geti(&words.clone(), &neg(&Py::Int(1)))]) } }) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_LIMIT_A_FACT(), &word.clone())); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "isdigit", &[]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    before = mcall(&slice_or_raise(&ask.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "strip", &[py_str(" .,!?")]);
    if py_truthy(&{ let __b = before.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&Py::Bool(py_eq(&mcall(&g_LIMIT_SOFT_BEFORE(), "sub", &[py_str(""), add(&before.clone(), &py_str(" "))]), &py_str(""))))) } }) {
    continue;
    }
    return thing.clone();
    }
    return Py::None;
    Py::None
}
pub fn two_ends_of_one_scale(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut one: Py = Py::None;
    let mut other: Py = Py::None;
    let mut second: Py = Py::None;
    let _ = py_str("True when the \"or\" in the line joins opposites rather than two things to pick between.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_SCALE_ASK(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("sooner or later"))) } }))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&g_OPPOSITE_PAIRS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    one = __unpk2[0].clone();
    other = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![one.clone(), other.clone()]), py_tuple(vec![other.clone(), one.clone()])]));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    first = __unpk4[0].clone();
    second = __unpk4[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[{ let mut __s = String::new(); __s.push_str("\\b(?:the |a |an )?"); __s.push_str(&fstr(&first.clone(), -1, "")); __s.push_str("\\s+or\\s+(?:the |a |an )?"); __s.push_str(&fstr(&second.clone(), -1, "")); __s.push_str("\\b"); py_string(__s) }, text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _turns_the_pair_down(args: &[Py]) -> Py {
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
    let _ = py_str("True when the thing before the \"or\" turns both sides down rather than asking which.");
    head = geti(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[py_str(" or ")]), &Py::Int(0));
    return call_builtin("bool", &[mcall(&g_REFUSED_PAIR(), "search", &[head.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _laughed_off(args: &[Py]) -> Py {
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
    let mut cue: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cue") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cue"))) };
    let mut match_: Py = Py::None;
    let _ = py_str("True when the phrase is followed by laughter: \"im beside myself laughing\".");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[cue.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&mcall(&g_LAUGHED_OFF_AFTER(), "match", &[mcall(&slice_or_raise(&text.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[])])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_person_they_like(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("True when a sex word is what the bot is asked to like, not what it is.");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_TASTE_VERBS(), &word.clone())); if !py_truthy(&__b) { __b } else { bitand(&call_builtin("set", &[slice_or_raise(&norm.clone(), &add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &g_GENDER_WORDS()) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_thing_they_own(args: &[Py]) -> Py {
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
    let _ = py_str("True when the data or the internet in the line is not what the bot was made of.");
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_LEARNED_ON())))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&bitand(&call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &sub(&g_SELF_ASK(), &g_LEARNED_ON()))));
    Py::None
}
pub fn agreement_of(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut flat: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The idiom of agreement the whole line is, or None: \"tell me about it\", \"you and me both\".\n\n    The idiom has to be the whole line, or the whole of its first clause: \"tell me about it, i\n    had one last year\" agrees and then says why, and the normaliser writes the clause mark that\n    tells the two apart. Anything tighter is the ask the words look like - \"tell me about the\n    boiler\", and also \"tell me about it properly\", which wants a fuller answer and not a nod.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&text.clone(), "lower", &[]), "replace", &[py_str(","), py_str(" ")]), "split", &[])]);
    first = mcall(&py_str(" "), "join", &[mcall(&geti(&mcall(&said.clone(), "split", &[py_str("|")]), &Py::Int(0)), "split", &[])]);
    flat = mcall(&py_str(" "), "join", &[mcall(&mcall(&said.clone(), "replace", &[py_str("|"), py_str(" ")]), "split", &[])]);
    return call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_AGREEMENT_IDIOMS()) { let mut phrase: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_eq(&flat.clone(), &phrase.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&first.clone(), &phrase.clone())) } }) {  __out.push(phrase.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _has_any(args: &[Py]) -> Py {
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
    let mut phrases: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrases") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrases"))) };
    let mut phrase: Py = Py::None;
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrases.clone()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn small_talk_only(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut size: Py = Py::None;
    let _ = py_str("True when the line is a greeting or a word about the weather and nothing else.\n\n    The greeting is taken out first and what is left has to be filler, which is what keeps the\n    phrase from matching inside a longer thought: \"raining\" is small talk in \"raining again\"\n    and weather in \"raining cats and dogs\"; \"how are you\" is a greeting, and \"how are you\n    supposed to defrost a turkey\" is a question about a turkey.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&norm.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(10)) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&g_SMALL_TALK_WORDS());
    for __item in __iter1 {
    phrase = __item.clone();
    size = call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[add(&sub(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &size.clone()), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    at = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("tuple", &[slice_or_raise(&norm.clone(), &at.clone(), &add(&at.clone(), &size.clone()), &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &phrase.clone()))) {
    continue;
    }
    rest = bitor(&call_builtin("set", &[slice_or_raise(&norm.clone(), &Py::None, &at.clone(), &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[slice_or_raise(&norm.clone(), &add(&at.clone(), &size.clone()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&Py::Bool(!py_truthy(&sub(&rest.clone(), &g_SMALL_TALK_LEFTOVER())))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn cue_about_person(args: &[Py]) -> Py {
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
    let mut cue: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cue") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cue"))) };
    let mut following: Py = Py::None;
    let mut found: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("True when this feeling word says how someone feels, not what something was like.\n\n    The word is found whole (\"mad\" is not in \"made\"), and it counts when it ends a clause or\n    is followed by a preposition, a connective or a time word: \"im mad\", \"she was cross with\n    me\", \"driving me mad\", \"mad at my sister\", \"im mad now\". It does not count when it comes\n    straight before another word: \"a mad day\", \"mad busy\", \"a sad film\", \"happy birthday\".\n    ");
    if py_truthy(&Py::Bool(py_contains(&g_PARTICLE_STATES(), &cue.clone()))) {
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[cue.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    found = __item.clone();
    if py_truthy(&_a_direction(&[text.clone(), mcall(&found.clone(), "start", &[])])) {
    return Py::Bool(false);
    }
    }
    }
    if py_truthy(&Py::Bool(py_contains(&g_CUE_FOLLOW(), &geti(&mcall(&cue.clone(), "split", &[]), &neg(&Py::Int(1)))))) {
    return call_builtin("bool", &[stdlib_call("re", "search", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[cue.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    }
    let __iter2 = py_iter_or_raise(&stdlib_call("re", "finditer", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[cue.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    match_ = __item.clone();
    if py_truthy(&_a_verb_holds_it(&[text.clone(), mcall(&match_.clone(), "start", &[])])) {
    continue;
    }
    rest = mcall(&slice_or_raise(&text.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "lstrip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_str(".,!?;:-|"), &geti(&rest.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(true);
    }
    following = mcall(&geti(&mcall(&rest.clone(), "split", &[]), &Py::Int(0)), "strip", &[py_str(".,!?;:-|")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&following.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CUE_FOLLOW(), &mcall(&following.clone(), "lower", &[]))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&slice_or_raise(&following.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&crate::gen::understanding::lexicon::g_COMMON(), &mcall(&following.clone(), "lower", &[]))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_STATE_NOUNS(), &mcall(&following.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { _state_before(&[text.clone(), mcall(&match_.clone(), "start", &[])]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PHRASE_OPENERS(), &mcall(&following.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { _clause_follows(&[rest.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = _speaker_before(&[text.clone(), mcall(&match_.clone(), "start", &[])]); if !py_truthy(&__b) { __b } else { _verb_after(&[rest.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_verb_holds_it(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut last: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the word in front of a feeling word is a verb doing something to it.\n\n    \"im moving flat on saturday\" is a move to a flat and \"im renting flat next month\" is about\n    the tenancy, but both read as a low mood because a feeling word followed by a preposition\n    counted as the person's own state. An -ing word in front of it is the verb that takes it as\n    its object, wherever the person cue sits - but only a verb that can take an object at all\n    (ING_TAKES_NO_OBJECT), because \"im sitting sad\" is the person's own state with a verb going\n    on beside it, not a thing done to the sadness. A copula or a feeling verb is let through,\n    because those are what make the word describe the person: \"i was feeling flat\", \"my tomato\n    plants are looking sad\".\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&slice_or_raise(&text.clone(), &Py::None, &at.clone(), &Py::None), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:-|\"'")]), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    last = geti(&words.clone(), &neg(&Py::Int(1)));
    return { let __b = mcall(&last.clone(), "endswith", &[py_str("ing")]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_FEELING_CUES_BEFORE(), &last.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_ING_THINGS(), &last.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_ING_TAKES_NO_OBJECT(), &last.clone())) } } } } } };
    Py::None
}
pub fn _state_before(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the words before a feeling word say the state is the speaker's own.\n\n    \"im in a mad mood\", \"i was in such a low state\", \"in a right temper\": a preposition and an\n    article in front of the feeling word are what make the state noun next to it the person's.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&slice_or_raise(&text.clone(), &Py::None, &at.clone(), &Py::None), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:-|\"'")]), "lower", &[])); } py_list(__out) })() };
    return call_builtin("bool", &[mcall(&g_STATE_TAIL(), "search", &[mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &neg(&Py::Int(4)), &Py::None, &Py::None)])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _speaker_before(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut recent: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the words in front of a feeling word make it the speaker's own state.\n\n    \"i am gutted\", \"i was scared\", \"i feel low\", with a degree word allowed in between\n    (\"im so gutted\"). It says nothing about \"i have a mad idea\" or \"we had a mad time\".\n\n    The words are read as typed, so the short forms are spelled out first: people write \"im\"\n    far more often than \"i am\", and \"im gutted mum cancelled\" is the same feeling as \"i am\n    gutted my mum cancelled\".\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&slice_or_raise(&text.clone(), &Py::None, &at.clone(), &Py::None), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:-|\"'")]), "lower", &[])); } py_list(__out) })() };
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_FEELING_FILLER(), &geti(&words.clone(), &neg(&Py::Int(1))))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    tail = mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &neg(&Py::Int(5)), &Py::None, &Py::None)]);
    tail = stdlib_call("re", "sub", &[py_str("\\bi'?m\\b"), py_str("i am"), tail.clone()]).unwrap_or_else(|e| exc::raise(e));
    tail = stdlib_call("re", "sub", &[py_str("\\bi'?ve\\b"), py_str("i have"), tail.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g_SPEAKER_TAIL(), "search", &[tail.clone()])) {
    return Py::Bool(true);
    }
    recent = slice_or_raise(&words.clone(), &neg(&Py::Int(4)), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(py_contains(&recent.clone(), &py_str("my")))) {
    let __iter1 = py_iter_or_raise(&slice_or_raise(&recent.clone(), &add(&mcall(&recent.clone(), "index", &[py_str("my")]), &Py::Int(1)), &Py::None, &Py::None));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_MOOD_NOUNS(), &word.clone()))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _verb_after(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a clause follows a feeling word: a helper or a verb in the next few words.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&mcall(&rest.clone(), "split", &[]), &Py::None, &Py::Int(5), &Py::None)) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:-|\"'")]), "lower", &[])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_CLAUSE_HELPERS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CLAUSE_VERBS(), &word.clone())) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_tuple(vec![py_str("ed"), py_str("ing")])]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _clause_follows(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut end: Py = Py::None;
    let mut i: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the words after a feeling word are a clause: \"the dog ate it\", \"my sister\n    forgot my birthday\". False for a bare phrase that describes the thing: \"day at work\".\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&rest.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str(".,!?;:-|'\"")])); } py_list(__out) })() };
    end = call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    i = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_str(".,!?;:-|"), &geti(&word.clone(), &neg(&Py::Int(1))))) } }) {
    end = add(&i.clone(), &Py::Int(1));
    break;
    }
    }
    clause = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::None, &call_builtin("min", &[end.clone(), Py::Int(8)]).unwrap_or_else(|e| exc::raise(e)), &Py::None)) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(mcall(&word.clone(), "lower", &[])); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&clause.clone()))) {
    return Py::Bool(false);
    }
    let __iter3 = py_iter_or_raise(&slice_or_raise(&clause.clone(), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_CUE_FOLLOW(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_CLAUSE_VERBS(), &word.clone())); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("ed")]) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_body_part_before(args: &[Py]) -> Py {
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
    let mut cue: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cue") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cue"))) };
    let mut before: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True for \"my stomach is upset\", where the word before the cue is a body part.");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[cue.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    match_ = __item.clone();
    before = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&slice_or_raise(&text.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:-|\"'")]), "lower", &[])); } py_list(__out) })() };
    before = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&before.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_FEELING_FILLER(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&word.clone(), &py_str("is"))) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = before.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BODY_SUBJECT(), &geti(&before.clone(), &neg(&Py::Int(1))))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _takes_an_object(args: &[Py]) -> Py {
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
    let mut end: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "end") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: end"))) };
    let mut after: Py = Py::None;
    let mut pos: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when what follows a state cue turns it into an action with an object.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&slice_or_raise(&text.clone(), &end.clone(), &Py::None, &Py::None), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:-|\"'")]), "lower", &[])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pos = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_TAIL_STOPS(), &word.clone())) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g_OBJECT_MARKERS(), &word.clone()))) {
    after = slice_or_raise(&words.clone(), &add(&pos.clone(), &Py::Int(1)), &add(&pos.clone(), &Py::Int(2)), &Py::None);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_TIME_MARKERS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = after.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_TIME_FILLERS(), &geti(&after.clone(), &Py::Int(0)))) } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _object_after(args: &[Py]) -> Py {
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
    let mut cue: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cue") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cue"))) };
    let mut match_: Py = Py::None;
    let _ = py_str("True when any place this cue occurs in the line is followed by an object.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "finditer", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[cue.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut match_: Py = __item.clone(); __out.push(_takes_an_object(&[text.clone(), mcall(&match_.clone(), "end", &[])])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _state_cue(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut cue: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cue") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cue"))) };
    let mut head: Py = Py::None;
    let _ = py_str("The word in `SELF_ONLY_STATE` this cue is built on, or None: \"wiped out\" is \"wiped\".");
    if py_truthy(&Py::Bool(py_contains(&g_SELF_ONLY_STATE(), &cue.clone()))) {
    return cue.clone();
    }
    head = geti(&mcall(&cue.clone(), "split", &[]), &Py::Int(0));
    return if py_truthy(&Py::Bool(py_contains(&g_SELF_ONLY_STATE(), &head.clone()))) { head.clone() } else { Py::None };
    Py::None
}
pub fn _a_copula_before(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a copula or a feeling verb comes just before `start`: \"im so pooped\".");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&slice_or_raise(&text.clone(), &Py::None, &start.clone(), &Py::None), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:-|\"'")]), "lower", &[])); } py_list(__out) })() };
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_FEELING_FILLER(), &geti(&words.clone(), &neg(&Py::Int(1))))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("both"), py_str("all")]), &geti(&words.clone(), &neg(&Py::Int(1))))) } } } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_COPULA_BEFORE(), &geti(&words.clone(), &neg(&Py::Int(1))))) } };
    Py::None
}
pub fn _state_of_a_person(args: &[Py]) -> Py {
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
    let mut cue: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cue") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cue"))) };
    let mut match_: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a person is what the feeling word is said of: \"im low\", \"they were shattered\".\n\n    The word before the cue is the subject, skipping the joiners, so a thing is still a thing:\n    \"the battery is low\", \"the windscreen is shattered\", \"work has been awful lately\".\n    ");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[cue.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_SUPPLY_WORDS(), &cue.clone())); if !py_truthy(&__b) { __b } else { _about_a_supply(&[text.clone(), mcall(&match_.clone(), "end", &[])]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_VERB_ONLY_STATE(), &cue.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_copula_before(&[text.clone(), mcall(&match_.clone(), "start", &[])]))) } }) {
    continue;
    }
    if py_truthy(&_takes_an_object(&[text.clone(), mcall(&match_.clone(), "end", &[])])) {
    continue;
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&slice_or_raise(&text.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:-|\"'")]), "lower", &[])); } py_list(__out) })() };
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_STATE_JOINERS(), &geti(&words.clone(), &neg(&Py::Int(1))))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_FEELING_FILLER(), &geti(&words.clone(), &neg(&Py::Int(1))))) } } } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(true);
    }
    subject = geti(&words.clone(), &neg(&Py::Int(1)));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PERSON_SUBJECT(), &subject.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OTHER_PEOPLE(), &subject.clone())) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_MOOD_NOUNS(), &subject.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&slice_or_raise(&slice_or_raise(&words.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None), &neg(&Py::Int(3)), &Py::None, &Py::None), &py_str("my"))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&_speaker_before(&[text.clone(), mcall(&match_.clone(), "start", &[])])) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _said_of_the_speaker(args: &[Py]) -> Py {
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
    let mut cue: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cue") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cue"))) };
    let mut match_: Py = Py::None;
    let _ = py_str("True when this cue comes straight after a self cue: \"im empty\", \"i feel out of sorts\".\n\n    A word that is also an ordinary word for a thing needs it: \"the flat is cold\" and \"an empty\n    shelf\" name a thing's state, and \"a blank page\" a page's.\n    ");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[cue.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&_speaker_before(&[text.clone(), mcall(&match_.clone(), "start", &[])])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _names_an_episode(args: &[Py]) -> Py {
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
    let mut cue: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cue") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cue"))) };
    let mut match_: Py = Py::None;
    let _ = py_str("True for \"panic attack\": the cue runs into the noun for its own episode.");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[cue.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b\\s+(\\w+)")), text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_EPISODE_NOUNS(), &mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[])))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn a_broken_heart_shape(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("True when the line says a heart is broken, in any of the shapes people type it.");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    return call_builtin("bool", &[{ let __b = mcall(&g_HEART_IN_A_STATE(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_BROKEN_HEARTED(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_BROKE_A_HEART(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_BROKEN_HEART(), "search", &[low.clone()]) } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _somebody_elses_heart(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut person: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the broken heart the line names is somebody else's, and the line says whose.\n\n    \"she is heartbroken\" and \"my mum is heartbroken\" give another person as the subject and the\n    feeling after it, which is their state and not the speaker's. It is read from the words,\n    because there is no state word here for `_state_owner` to hang an owner on - \"heartbroken\"\n    is a feeling but not a state word - so without this the line is nobody's, and the speaker's\n    card answers somebody else's trouble.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&a_broken_heart_shape(&[low.clone()]))); if py_truthy(&__b) { __b } else { _a_broken_heart_is_theirs(&[low.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_POSSESSED_HEART(), "search", &[low.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_PERSON_IS_THE_SUBJECT(), "match", &[low.clone()])) {
    return Py::Bool(true);
    }
    words = mcall(&low.clone(), "split", &[]);
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("my"), py_str("our"), py_str("the")]), &geti(&words.clone(), &Py::Int(0)))) } }) {
    person = mcall(&geti(&words.clone(), &Py::Int(1)), "strip", &[py_str(".,!?'\"")]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_A_PERSON_ELSE(), &person.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_PERSON_ELSE(), &slice_or_raise(&person.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None))) } }) {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_list(vec![py_str("heart")]), py_list(vec![py_str("hearts")])]), &slice_or_raise(&words.clone(), &Py::Int(2), &Py::Int(3), &Py::None)))) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_broken_heart_is_theirs(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut owner: Py = Py::None;
    let mut starts: Py = Py::None;
    let mut window: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the heart the line breaks is the speaker's own.\n\n    \"my heart is broken\" and \"im broken hearted\" are the person's, and so is \"i have a broken\n    heart\". \"her heart is broken\" and \"my mum's heart is broken\"\n    are somebody else's, and \"how do i get over a broken heart\" is one in general - the sad\n    pool's advice is written for the person who is in it, so only the speaker's own heart\n    counts. The owner is read from the word in front of the heart, because that is where the\n    line puts it, and a name or a relation with an \"s\" on it (\"mums heart\", \"friends heart\")\n    is as much somebody else's as \"her\".\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&a_broken_heart_shape(&[low.clone()])))) {
    return Py::Bool(false);
    }
    words = mcall(&low.clone(), "split", &[]);
    starts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![stdlib_call("re", "search", &[py_str("\\bhearts?\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "search", &[py_str("\\bbroken\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e))])) { let mut match_: Py = __item.clone(); if py_truthy(&match_.clone()) {  __out.push(mcall(&match_.clone(), "start", &[])); } } py_list(__out) })() };
    at = if py_truthy(&starts.clone()) { call_builtin("len", &[mcall(&slice_or_raise(&low.clone(), &Py::None, &call_builtin("min", &[starts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None), "split", &[])]).unwrap_or_else(|e| exc::raise(e)) } else { Py::Int(0) };
    window = slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), sub(&at.clone(), &Py::Int(2))]).unwrap_or_else(|e| exc::raise(e)), &at.clone(), &Py::None);
    owner = if py_truthy(&window.clone()) { geti(&window.clone(), &neg(&Py::Int(1))) } else { py_str("") };
    if py_truthy(&mcall(&g_HAVE_A_BROKEN_HEART(), "search", &[low.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_BROKE_A_HEART(), "search", &[low.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_THEIR_OWN(), &owner.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&owner.clone(), "endswith", &[py_str("'s")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&owner.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("is"), py_str("was"), py_str("this"), py_str("its"), py_str("us")]), &owner.clone())) } } } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&window.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&py_tuple(vec![py_str("my"), py_str("our")]), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the")]), &owner.clone()))) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:her|his|their|your)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[stdlib_call("re", "match", &[py_str("^(?:i|im|i'm|ive|i've|we|we're|weve|we've|our)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _a_hedged_low(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut count: Py = Py::None;
    let mut doubted: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the whole line is the polite way of saying not great: \"could be better\".\n\n    The reading hands this the line with a bar where one clause ends and the next begins\n    (\"could be better | you\"), so the bar is punctuation like the rest of it.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str(".,!?;:-'\"|")])); } py_list(__out) })() };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    doubted = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&g_HEDGED_DOUBT());
    for __item in __iter1 {
    phrase = __item.clone();
    count = call_builtin("len", &[mcall(&phrase.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &count.clone()); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &neg(&count.clone()), &Py::None, &Py::None)]), &phrase.clone())) } }) {
    words = slice_or_raise(&words.clone(), &Py::None, &neg(&count.clone()), &Py::None);
    doubted = Py::Bool(true);
    break;
    }
    }
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_HEDGED_WRAPPER(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    let _ = mcall(&words.clone(), "pop", &[Py::Int(0)]);
    }
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_HEDGED_WRAPPER(), &geti(&words.clone(), &neg(&Py::Int(1))))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    answer = mcall(&py_str(" "), "join", &[words.clone()]);
    return { let __b = Py::Bool(py_contains(&g_HEDGED_LOW(), &answer.clone())); if py_truthy(&__b) { __b } else { { let __b = doubted.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_HEDGED_OK(), &answer.clone())) } } } };
    Py::None
}
pub fn _copula_just_before(args: &[Py]) -> Py {
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
    let mut pos: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pos") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pos"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when a copula or feeling verb sits directly before the word, degrees aside.");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&words.clone(), &Py::None, &pos.clone(), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_FEELING_FILLER(), &word.clone()))) {
    continue;
    }
    return Py::Bool(py_contains(&g_FEELING_COPULA(), &word.clone()));
    }
    return Py::Bool(false);
    Py::None
}
pub fn _state_as_feeling(args: &[Py]) -> Py {
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
    let mut pos: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a state word of the list above is the speaker's mood in this line.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str(".,!?;:-|\"'")])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pos = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g_STATE_AS_FEELING(), &word.clone()))) {
    continue;
    }
    if py_truthy(&_thing_turned_down(&[words.clone(), pos.clone()])) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_copula_just_before(&[words.clone(), pos.clone()])))) {
    continue;
    }
    if py_truthy(&_particle_is_a_place(&[words.clone(), pos.clone()])) {
    continue;
    }
    after = if py_truthy(&lt(&add(&pos.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&words.clone(), &add(&pos.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&g_FEELING_NOT_MOOD(), &after.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("self"), py_str("other")]), &_whose_state(&[words.clone(), pos.clone()])))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _works_on_the_speaker(args: &[Py]) -> Py {
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
    let mut acted_on: Py = Py::None;
    let mut ahead: Py = Py::None;
    let mut back: Py = Py::None;
    let mut feeling: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut named: Py = Py::None;
    let mut pos: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The feeling a thing does to the speaker, or None: \"work is stressing me out\".\n\n    Only the shape <thing> <verb> me/us, and only a verb whose stem is a feeling, so \"he\n    stressed the point\" is not it. The person acted on has to be \"me\" or \"us\": \"the kids are\n    tiring my mum out\" is her state, not the speaker's.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:-|\"'()")]), "lower", &[])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pos = __unpk2[0].clone();
    word = __unpk2[1].clone();
    feeling = mcall(&g_WORK_ON_ME(), "get", &[word.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&feeling.clone()))) {
    continue;
    }
    lead = Py::None;
    let __iter3 = py_iter_or_raise(&call_builtin("range", &[sub(&pos.clone(), &Py::Int(1)), call_builtin("max", &[sub(&pos.clone(), &Py::Int(4)), neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)), neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    back = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_ACT_ON_FILLER(), &geti(&words.clone(), &back.clone()))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ACT_ON_LEAD(), &geti(&words.clone(), &back.clone()))) } }) {
    continue;
    }
    lead = geti(&words.clone(), &back.clone());
    break;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("i"), py_str("i'm"), py_str("im"), py_str("me"), py_str("my"), py_str("myself"), py_str("we"), py_str("our")]), &lead.clone()))) {
    continue;
    }
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &call_builtin("max", &[sub(&pos.clone(), &Py::Int(4)), Py::Int(0)]).unwrap_or_else(|e| exc::raise(e)), &pos.clone(), &Py::None)) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_ACT_ON_FILLER(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_PARTS_OF_ME(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&{ let __b = lead.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&lead.clone(), "endswith", &[py_str("nt")]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("not"), py_str("never"), py_str("no")]), &lead.clone())) } } } }) {
    continue;
    }
    acted_on = Py::None;
    let __iter4 = py_iter_or_raise(&call_builtin("range", &[add(&pos.clone(), &Py::Int(1)), call_builtin("min", &[add(&pos.clone(), &Py::Int(4)), call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    ahead = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_ACT_ON_FILLER(), &geti(&words.clone(), &ahead.clone())))) {
    continue;
    }
    acted_on = geti(&words.clone(), &ahead.clone());
    break;
    }
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("me"), py_str("us")]), &acted_on.clone()))) {
    continue;
    }
    return feeling.clone();
    }
    return Py::None;
    Py::None
}
pub fn _skeleton(args: &[Py]) -> Py {
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
    let mut letter: Py = Py::None;
    let _ = py_str("A word with its vowels taken out: the letters that carry how it sounds.");
    return mcall(&py_str(""), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&word.clone(), "lower", &[])) { let mut letter: Py = __item.clone(); if py_truthy(&{ let __b = mcall(&letter.clone(), "isalpha", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("aeiouy"), &letter.clone())) } }) {  __out.push(letter.clone()); } } py_list(__out) })() }]);
    Py::None
}
pub fn _sound_cues(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut cue: Py = Py::None;
    let mut cues: Py = Py::None;
    let mut index: Py = Py::None;
    let mut known: Py = Py::None;
    let mut name: Py = Py::None;
    let _ = py_str("({skeleton: {emotion name: cues}}, every single feeling word) this file knows.");
    if py_truthy(&Py::Bool(py_is(&g__SOUND_CUES(), &Py::None))) {
    index = py_dict(vec![]);
    known = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&g_EMOTION_WORDS(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    cues = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&cues.clone());
    for __item in __iter3 {
    cue = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&cue.clone(), &py_str(" ")))) {
    continue;
    }
    let _ = mcall(&known.clone(), "add", &[cue.clone()]);
    if py_truthy(&lt(&call_builtin("len", &[cue.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    continue;
    }
    let _ = mcall(&mcall(&mcall(&index.clone(), "setdefault", &[_skeleton(&[cue.clone()]), py_dict(vec![])]), "setdefault", &[name.clone(), py_list(vec![])]), "append", &[cue.clone()]);
    }
    }
    s__SOUND_CUES(py_tuple(vec![index.clone(), known.clone()]));
    }
    return g__SOUND_CUES();
    Py::None
}
pub fn feeling_by_sound(args: &[Py]) -> Py {
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
    let mut cue: Py = Py::None;
    let mut cues: Py = Py::None;
    let mut found: Py = Py::None;
    let mut index: Py = Py::None;
    let mut known: Py = Py::None;
    let mut low: Py = Py::None;
    let mut matches: Py = Py::None;
    let mut name: Py = Py::None;
    let _ = py_str("The feeling a word is the sound of, when it is not the feeling's spelling, or None.\n\n    People type \"im board\" for bored, \"im tried\" for tired, \"im anxty\" for anxiety. Each is\n    another word or no word, so the misspelling fixer may not touch it - but in the \"im\" frame\n    the word says how the person is, and the vowels are what chat typing loses. Only a word\n    that keeps the feeling's consonants, in order and with the same first letter, is read this\n    way, and only when one feeling matches: \"board\" is bored, \"im cory\" and \"im karen\" stay\n    names because no feeling sounds like them.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    low = mcall(&mcall(&mcall(&word.clone(), "strip", &[]), "strip", &[py_str(".,!?;:'\"()")]), "lower", &[]);
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&low.clone(), "isalpha", &[]))) } }) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_sound_cues(&[]), 2, None);
    index = __unpk1[0].clone();
    known = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_contains(&known.clone(), &low.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&crate::gen::understanding::lexicon::g_COMMON_NAMES(), &low.clone()))) {
    return Py::None;
    }
    matches = { let __b = mcall(&index.clone(), "get", &[_skeleton(&[low.clone()])]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    found = call_builtin("sorted", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&matches.clone(), "items", &[])) { let __unpk8436 = py_unpack(&__item.clone(), 2, None); let mut name: Py = __unpk8436[0].clone(); let mut cues: Py = __unpk8436[1].clone(); if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&cues.clone()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_eq(&slice_or_raise(&cue.clone(), &Py::None, &Py::Int(1), &Py::None), &slice_or_raise(&low.clone(), &Py::None, &Py::Int(1), &Py::None)))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(name.clone()); } } py_set(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { geti(&found.clone(), &Py::Int(0)) } else { Py::None };
    Py::None
}
pub fn _a_feeling_can_end_here(args: &[Py]) -> Py {
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
    let mut end: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "end") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: end"))) };
    let mut rest: Py = Py::None;
    let _ = py_str("True when what follows the word lets it be the state rather than the thing.");
    rest = mcall(&slice_or_raise(&text.clone(), &end.clone(), &Py::None, &Py::None), "lstrip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_str(".,!?;:"), &geti(&rest.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(true);
    }
    return Py::Bool(py_contains(&g_SOUND_FOLLOWS(), &mcall(&geti(&mcall(&rest.clone(), "split", &[]), &Py::Int(0)), "strip", &[py_str(".,!?;:'\"")])));
    Py::None
}
pub fn _feeling_said_as_a_sound(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let _ = py_str("The feeling a word straight after \"im\" is the sound of, or None.");
    let __iter1 = py_iter_or_raise(&mcall(&g_FEELING_SOUND_FRAME(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&_takes_an_object(&[text.clone(), mcall(&match_.clone(), "end", &[])])) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_a_feeling_can_end_here(&[text.clone(), mcall(&match_.clone(), "end", &[])])))) {
    continue;
    }
    name = feeling_by_sound(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]);
    if py_truthy(&name.clone()) {
    return name.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_period_of_their_own(args: &[Py]) -> Py {
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
    let mut cue: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cue") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cue"))) };
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the state word describes a day or a week the speaker had.\n\n    \"i had a low day\", \"ive had a low few weeks\", \"today was a low day\": the state stands in\n    front of the period, so nothing in front of it is a person. The head of the clause has to be\n    the speaker or the day itself, or the period belongs to something else (\"the battery was low\n    all day\") and the state is that thing's.\n    ");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[cue.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    match_ = __item.clone();
    after = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&slice_or_raise(&text.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:-|\"'")]), "lower", &[])); } py_list(__out) })() };
    seen = Py::Int(0);
    while py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { { let __b = lt(&seen.clone(), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OWN_PERIOD_FILLER(), &geti(&after.clone(), &Py::Int(0)))) } } } }) {
    let _ = mcall(&after.clone(), "pop", &[Py::Int(0)]);
    let __aug2 = add(&seen.clone(), &Py::Int(1));
    seen = __aug2;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_OWN_PERIOD_NOUNS(), &geti(&after.clone(), &Py::Int(0)))) } }) {
    continue;
    }
    before = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&slice_or_raise(&text.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:-|\"'()")]), "lower", &[])); } py_list(__out) })() };
    let __iter3 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&before.clone(), &neg(&g_OWN_PERIOD_REACH()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_OWN_PERIOD_STOPS(), &word.clone()))) {
    break;
    }
    if py_truthy(&Py::Bool(py_contains(&g_OWN_PERIOD_HEADS(), &word.clone()))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _acts_on_the_speaker(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The feeling a line about something acting on the speaker carries, or None.");
    if py_truthy(&mcall(&g_LAUGH_AT_THE_END(), "search", &[text.clone()])) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_ACTS_ON_THE_SPEAKER());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    name = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&stdlib_call("re", "finditer", &[add(&add(&py_str("\\b(?:"), &pattern.clone()), &py_str(")")), text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    match_ = __item.clone();
    before = slice_or_raise(&mcall(&slice_or_raise(&text.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "split", &[]), &neg(&Py::Int(2)), &Py::None, &Py::None);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&before.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_A_BODY_SUBJECT(), &mcall(&word.clone(), "strip", &[py_str(".,!?;:'\"")])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    return name.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn emotion_of(args: &[Py]) -> Py {
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
    let mut acted: Py = Py::None;
    let mut cue: Py = Py::None;
    let mut cues: Py = Py::None;
    let mut idiom: Py = Py::None;
    let mut name: Py = Py::None;
    let mut sounded: Py = Py::None;
    let mut state: Py = Py::None;
    let mut worked_on: Py = Py::None;
    let _ = py_str("The feeling the line names about a person, or None: \"a mad day\" is a busy one.");
    text = mcall(&g_BETTER_OFF(), "sub", &[py_str(" "), text.clone()]);
    text = mcall(&g_NO_BETTER(), "sub", &[py_str("worse"), text.clone()]);
    text = mcall(&g_GETTING_WORSE(), "sub", &[py_func("lambda", move |args: &[Py]| { let acted = acted.clone(); let cue = cue.clone(); let cues = cues.clone(); let idiom = idiom.clone(); let name = name.clone(); let sounded = sounded.clone(); let state = state.clone(); let text = text.clone(); let worked_on = worked_on.clone(); let match_: Py = __kwp(args, 0, "match").unwrap_or(Py::None); add(&mcall(&match_.clone(), "group", &[Py::Int(1)]), &py_str(" worse")) }), text.clone()]);
    text = mcall(&g_IDIOM_STATES(), "sub", &[py_str(" "), text.clone()]);
    text = mcall(&g_ABILITY_AT(), "sub", &[py_str(" "), text.clone()]);
    let __iter1 = py_iter_or_raise(&mcall(&g_EMOTION_WORDS(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    cues = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&cues.clone());
    for __item in __iter3 {
    cue = __item.clone();
    state = _state_cue(&[cue.clone()]);
    if py_truthy(&{ let __b = state.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_state_of_a_person(&[text.clone(), state.clone()]))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_BODY_STATE_CUES(), &cue.clone()))) {
    if py_truthy(&{ let __b = _a_body_part_before(&[text.clone(), cue.clone()]); if py_truthy(&__b) { __b } else { _object_after(&[text.clone(), cue.clone()]) } }) {
    continue;
    }
    }
    idiom = mcall(&g_NOT_A_MOOD_THERE(), "get", &[cue.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&idiom.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { mcall(&idiom.clone(), "search", &[text.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_LABEL_ON_A_PERSON(), &cue.clone())); if !py_truthy(&__b) { __b } else { _said_of_the_speaker(&[text.clone(), cue.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = { let __b = Py::Bool(py_contains(&g_SELF_ONLY_CUES(), &cue.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_ALWAYS_SELF_ONLY(), &cue.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_THING_OR_PERSON_CUES(), &cue.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_CONFUSED_SELF_ONLY(), &cue.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_COMPARISON(), &cue.clone())) } } } } } } } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_said_of_the_speaker(&[text.clone(), cue.clone()]))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_THING_OR_PERSON_CUES(), &cue.clone())); if !py_truthy(&__b) { __b } else { _laughed_off(&[text.clone(), cue.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PERIOD_CUES(), &cue.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_PERIOD(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_LIMIT_ABOUT_PEOPLE(), "search", &[text.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_MONTH_PERIOD(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_MONEY_SUBJECT(), "search", &[text.clone()]) } } } } } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_SAID_AS_A_NEGATION(), &cue.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&denied(&[text.clone(), cue.clone()]))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_GOOD_EMOTION_NAMES(), &name.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_SAID_AS_A_NEGATION(), &cue.clone())); if !py_truthy(&__b) { __b } else { denied(&[text.clone(), cue.clone()]) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_GOOD_EMOTION_NAMES(), &name.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_GOOD_WORD_MISSING(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_GOOD_WORD_COMPARED(), "search", &[text.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_GOOD_WORD_BY_COMPARISON(), "search", &[text.clone()]))) } } } } } }) {
    return py_str("sad");
    }
    if py_truthy(&{ let __b = cue_about_person(&[text.clone(), cue.clone()]); if py_truthy(&__b) { __b } else { _names_an_episode(&[text.clone(), cue.clone()]) } }) {
    return name.clone();
    }
    }
    }
    acted = _acts_on_the_speaker(&[text.clone()]);
    if py_truthy(&acted.clone()) {
    return acted.clone();
    }
    if py_truthy(&a_broken_heart_shape(&[text.clone()])) {
    return py_str("sad");
    }
    sounded = _feeling_said_as_a_sound(&[text.clone()]);
    if py_truthy(&sounded.clone()) {
    return sounded.clone();
    }
    worked_on = _works_on_the_speaker(&[text.clone()]);
    if py_truthy(&worked_on.clone()) {
    return worked_on.clone();
    }
    if py_truthy(&_state_as_feeling(&[text.clone()])) {
    return py_str("sad");
    }
    let __iter4 = py_iter_or_raise(&mcall(&g_EMOTION_WORDS(), "items", &[]));
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    name = __unpk5[0].clone();
    cues = __unpk5[1].clone();
    if py_truthy(&Py::Bool(py_contains(&g_GOOD_EMOTION_NAMES(), &name.clone()))) {
    continue;
    }
    let __iter6 = py_iter_or_raise(&cues.clone());
    for __item in __iter6 {
    cue = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_THE_SPEAKERS_DAY(), &cue.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PERIOD_CUES(), &cue.clone())) } }) {
    continue;
    }
    if py_truthy(&denied(&[text.clone(), cue.clone()])) {
    continue;
    }
    if py_truthy(&_a_period_of_their_own(&[text.clone(), cue.clone()])) {
    return name.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _capability_question(args: &[Py]) -> Py {
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
    let mut compact: Py = Py::None;
    let mut i: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut start: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True for \"what can you do?\" and \"what can you help with?\"; False when the phrase is\n    followed by something the person wants done (\"with leftover rice\", \"me cook tonight\") or\n    points back at the thing in play (\"about that\", \"for it\").");
    compact = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_CAPABILITY_FILLER(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&g_CAPABILITY());
    for __item in __iter1 {
    phrase = __item.clone();
    wanted = mcall(&phrase.clone(), "split", &[]);
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[add(&sub(&call_builtin("len", &[compact.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    start = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&slice_or_raise(&compact.clone(), &start.clone(), &add(&start.clone(), &call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None), &wanted.clone()))) {
    continue;
    }
    tail = slice_or_raise(&compact.clone(), &add(&start.clone(), &call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None, &Py::None);
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tail.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_CAPABILITY_CARRIER(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&rest.clone()) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk439 = py_unpack(&__item.clone(), 2, None); let mut i: Py = __unpk439[0].clone(); let mut word: Py = __unpk439[1].clone(); __out.push({ let __b = Py::Bool(py_contains(&g_REFERRING(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = lt(&add(&i.clone(), &Py::Int(1)), &call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_POINTED_AT(), &geti(&tail.clone(), &add(&i.clone(), &Py::Int(1))))) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _asks_how_the_bot_sees_me(args: &[Py]) -> Py {
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
    let mut ask: Py = Py::None;
    let mut changed: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut size: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the whole line asks the bot what it makes of the person.\n\n    A tag of honesty is not a new subject: \"what do you think of me tbh\" is the same ask, and\n    the chat-speak repair turns that tag into the words \"to be honest\" before this reads the\n    line, so the softening phrases people end a feeling with are peeled off first. The same\n    repair can also turn the \"me\" before the tag into \"my\" (\"what do you think of me tbh\" reads\n    as \"what do you think of my to be honest\"), which is why the pronoun is read back: the ask\n    is the ask, and the word in front of a tag is not a noun for the repair to fix.\n    ");
    words = call_builtin("list", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    changed = Py::Bool(true);
    while py_truthy(&{ let __b = changed.clone(); if !py_truthy(&__b) { __b } else { words.clone() } }) {
    changed = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&g_MOOD_TAIL());
    for __item in __iter1 {
    phrase = __item.clone();
    size = call_builtin("len", &[mcall(&phrase.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = lt(&size.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&words.clone(), &neg(&size.clone()), &Py::None, &Py::None), &mcall(&phrase.clone(), "split", &[]))) } }) {
    let _ = py_delitem(&words.clone(), &Py::None).unwrap_or(());
    changed = Py::Bool(true);
    }
    }
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ME_LEAD(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    let _ = mcall(&words.clone(), "pop", &[Py::Int(0)]);
    changed = Py::Bool(true);
    }
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ME_TAIL(), &geti(&words.clone(), &neg(&Py::Int(1))))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    changed = Py::Bool(true);
    }
    }
    ask = if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&words.clone(), &neg(&Py::Int(1))), &py_str("my"))) } }) { add(&slice_or_raise(&words.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None), &py_list(vec![py_str("me")])) } else { words.clone() };
    return { let __b = Py::Bool(py_contains(&g_ME_ASKS(), &mcall(&py_str(" "), "join", &[words.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ME_ASKS(), &mcall(&py_str(" "), "join", &[ask.clone()]))) } };
    Py::None
}
pub fn _asks_who_this_is(args: &[Py]) -> Py {
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
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True for the whole-line ask about who the person is talking to.");
    words = call_builtin("tuple", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WHO_TAIL(), &geti(&words.clone(), &neg(&Py::Int(1))))) } }) {
    words = slice_or_raise(&words.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None);
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_tuple(vec![py_str("who"), py_str("is"), py_str("this")]), py_tuple(vec![py_str("who"), py_str("am"), py_str("i"), py_str("talking"), py_str("to")]), py_tuple(vec![py_str("who"), py_str("am"), py_str("i"), py_str("speaking"), py_str("to")]), py_tuple(vec![py_str("who"), py_str("am"), py_str("i"), py_str("speaking"), py_str("with")]), py_tuple(vec![py_str("who"), py_str("am"), py_str("i"), py_str("chatting"), py_str("to")]), py_tuple(vec![py_str("who"), py_str("am"), py_str("i"), py_str("chatting"), py_str("with")]), py_tuple(vec![py_str("who"), py_str("am"), py_str("i"), py_str("talking"), py_str("with")]), py_tuple(vec![py_str("who"), py_str("was"), py_str("i"), py_str("talking"), py_str("to")]), py_tuple(vec![py_str("who"), py_str("was"), py_str("i"), py_str("speaking"), py_str("to")]), py_tuple(vec![py_str("who"), py_str("was"), py_str("i"), py_str("speaking"), py_str("with")])]), &words.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&slice_or_raise(&words.clone(), &Py::None, &Py::Int(4), &Py::None), &py_tuple(vec![py_str("who"), py_str("have"), py_str("i"), py_str("been")]))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![Py::Int(6), Py::Int(7)]), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return { let __b = Py::Bool(py_contains(&g_WHO_TALKING(), &geti(&words.clone(), &Py::Int(4)))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("to"), py_str("with")]), &geti(&words.clone(), &Py::Int(5)))) } };
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_tuple(vec![py_str("who"), py_str("did"), py_str("i")]), py_tuple(vec![py_str("who"), py_str("was"), py_str("i")]), py_tuple(vec![py_str("who"), py_str("am"), py_str("i")])]), &slice_or_raise(&words.clone(), &Py::None, &Py::Int(3), &Py::None))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![Py::Int(5), Py::Int(6)]), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&geti(&words.clone(), &Py::Int(3)), &py_str("just"))) } }) {
    return Py::Bool(false);
    }
    at = if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)))) { Py::Int(1) } else { Py::Int(0) };
    return { let __b = Py::Bool(py_contains(&g_WHO_TALKING(), &geti(&words.clone(), &add(&Py::Int(3), &at.clone())))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("to"), py_str("with")]), &geti(&words.clone(), &add(&Py::Int(4), &at.clone())))) } };
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_tuple(vec![py_str("am"), py_str("i"), py_str("talking")]), py_tuple(vec![py_str("am"), py_str("i"), py_str("speaking")]), py_tuple(vec![py_str("am"), py_str("i"), py_str("chatting")])]), &slice_or_raise(&words.clone(), &Py::None, &Py::Int(3), &Py::None)))) {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("to"), py_str("with")]), &geti(&words.clone(), &Py::Int(3)))) } }) {
    return { let __b = Py::Bool(py_contains(&g_WHO_A_THING(), &geti(&words.clone(), &Py::Int(4)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an")]), &geti(&words.clone(), &Py::Int(4)))) } };
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("to"), py_str("with")]), &geti(&words.clone(), &Py::Int(3)))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an")]), &geti(&words.clone(), &Py::Int(4)))) } } } }) {
    return Py::Bool(py_contains(&g_WHO_A_THING(), &geti(&words.clone(), &Py::Int(5))));
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_tuple(vec![py_str("is"), py_str("this")]), py_tuple(vec![py_str("is"), py_str("it")])]), &slice_or_raise(&words.clone(), &Py::None, &Py::Int(2), &Py::None))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![Py::Int(3), Py::Int(4), Py::Int(5)]), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    rest = slice_or_raise(&words.clone(), &Py::Int(2), &Py::None, &Py::None);
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an")]), &geti(&rest.clone(), &Py::Int(0)))) } }) {
    rest = slice_or_raise(&rest.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&rest.clone(), &Py::Int(0)), &py_str("real"))) } }) {
    rest = slice_or_raise(&rest.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    return { let __b = call_builtin("bool", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_WHO_A_THING(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    }
    return Py::Bool(false);
    Py::None
}
pub fn _asks_where_the_bot_is(args: &[Py]) -> Py {
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
    let _ = py_str("True for the whole-line ask about where the bot is.");
    words = call_builtin("tuple", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("where"), py_str("whereabouts")]), &geti(&words.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(false);
    }
    rest = slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(py_eq(&slice_or_raise(&rest.clone(), &Py::None, &Py::Int(2), &Py::None), &py_tuple(vec![py_str("are"), py_str("you")])))) {
    rest = slice_or_raise(&rest.clone(), &Py::Int(2), &Py::None, &Py::None);
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&slice_or_raise(&rest.clone(), &Py::None, &Py::Int(2), &Py::None), &py_tuple(vec![py_str("do"), py_str("you")]))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&rest.clone(), &Py::Int(2), &Py::Int(3), &Py::None), &py_tuple(vec![py_str("live")]))) } }) {
    rest = slice_or_raise(&rest.clone(), &Py::Int(3), &Py::None, &Py::None);
    } else {
    if py_truthy(&Py::Bool(py_eq(&slice_or_raise(&rest.clone(), &Py::None, &Py::Int(2), &Py::None), &py_tuple(vec![py_str("you"), py_str("live")])))) {
    rest = slice_or_raise(&rest.clone(), &Py::Int(2), &Py::None, &Py::None);
    } else {
    return Py::Bool(false);
    }
    }
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_WHERE_BOT_TAIL(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_stop_ask(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the whole line asks whether the person should stop or go.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str(","), py_str(" ")]), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str(""), py_str(" ")]), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_STOP_ASK_FILLER(), &mcall(&geti(&words.clone(), &Py::Int(0)), "strip", &[py_str(".,!?")]))) } }) {
    let _ = mcall(&words.clone(), "pop", &[Py::Int(0)]);
    }
    return call_builtin("bool", &[mcall(&g_STOP_ASK(), "fullmatch", &[mcall(&mcall(&py_str(" "), "join", &[words.clone()]), "strip", &[py_str(" .!?")])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _about_bot_kind(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut norm: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { py_tuple(vec![]) };
    let mut bare: Py = Py::None;
    let mut flat: Py = Py::None;
    let _ = py_str("Which kind of about-the-bot question this is, or None.\n\n    \"memory\" asks what the bot remembers, \"help\" what it can do, and \"identity\" what it is;\n    \"warmth\" asks what it makes of the person, which is the one ask about the bot that is about\n    both of them. The reply modules pick the phrasing from this, so each kind is read on the\n    words that actually ask it and nothing else is pulled in.\n    ");
    bare = add(&add(&py_str(" "), &mcall(&text.clone(), "strip", &[])), &py_str(" "));
    flat = mcall(&mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "replace", &[py_str(" | "), py_str(" ")]), "replace", &[py_str("|"), py_str(" ")]);
    if py_truthy(&mcall(&g_WHILE_ASK(), "search", &[flat.clone()])) {
    return py_str("while");
    }
    if py_truthy(&{ let __b = mcall(&g_KEEPING_ASK(), "search", &[flat.clone()]); if py_truthy(&__b) { __b } else { a_stop_ask(&[flat.clone()]) } }) {
    return py_str("keeping");
    }
    if py_truthy(&_asks_where_the_bot_is(&[norm.clone()])) {
    return py_str("where");
    }
    if py_truthy(&_asks_who_this_is(&[norm.clone()])) {
    return py_str("identity");
    }
    if py_truthy(&{ let __b = mcall(&g_ANSWERING_A_QUOTED_ASK(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_self_ask(&[norm.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&_asks_how_the_bot_sees_me(&[norm.clone()])) {
    return py_str("warmth");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&words.clone(), &py_str("remember"))); if !py_truthy(&__b) { __b } else { bitand(&py_set(vec![py_str("you"), py_str("your")]), &words.clone()) } }) {
    return py_str("memory");
    }
    if py_truthy(&_capability_question(&[call_builtin("list", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))])) {
    return py_str("help");
    }
    if py_truthy(&_asks_if_the_bot_is_a_product(&[norm.clone()])) {
    return py_str("model");
    }
    if py_truthy(&_recalls_their_own_line(&[norm.clone()])) {
    return Py::None;
    }
    if py_truthy(&_asks_what_is_new_with_the_bot(&[norm.clone()])) {
    return py_str("doings");
    }
    if py_truthy(&_asks_what_the_bot_has_been_doing(&[norm.clone()])) {
    return py_str("doings");
    }
    if py_truthy(&_asks_whether_the_bot_is_about(&[norm.clone()])) {
    return py_str("doings");
    }
    if py_truthy(&_asks_how_it_spends_its_time(&[norm.clone()])) {
    return py_str("doings");
    }
    if py_truthy(&{ let __b = _asks_what_the_bot_is_doing(&[norm.clone()]); if !py_truthy(&__b) { __b } else { { let __b = bitand(&py_set(vec![py_str("you"), py_str("u"), py_str("ya")]), &words.clone()); if !py_truthy(&__b) { __b } else { bitand(&words.clone(), &g_ASKS_IF_ANYTHING()) } } } }) {
    return py_str("doings");
    }
    if py_truthy(&_asks_about_its_experience(&[norm.clone()])) {
    return py_str("doings");
    }
    if py_truthy(&_self_ask(&[norm.clone()])) {
    return if py_truthy(&bitand(&py_set(vec![py_str("remember"), py_str("recall"), py_str("forget")]), &words.clone())) { py_str("memory") } else { py_str("identity") };
    }
    if py_truthy(&{ let __b = _how_long_itself(&[norm.clone()]); if py_truthy(&__b) { __b } else { _when_itself(&[norm.clone()]) } }) {
    return py_str("memory");
    }
    if py_truthy(&_counts_its_own_doings(&[norm.clone()])) {
    return py_str("memory");
    }
    if py_truthy(&{ let __b = mcall(&g_A_YOUR_KIN(), "search", &[mcall(&py_str(" "), "join", &[norm.clone()])]); if py_truthy(&__b) { __b } else { mcall(&g_A_YOUR_OWN_THING(), "search", &[mcall(&py_str(" "), "join", &[norm.clone()])]) } }) {
    return py_str("life");
    }
    if py_truthy(&{ let __b = mcall(&g_A_HOW_MANY_OF_THE_BOTS_OWN(), "search", &[mcall(&py_str(" "), "join", &[norm.clone()])]); if !py_truthy(&__b) { __b } else { bitand(&call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_BOT_LIFE()) } }) {
    return py_str("life");
    }
    if py_truthy(&mcall(&g_A_WHAT_ITS_LIKE(), "search", &[mcall(&py_str(" "), "join", &[norm.clone()])])) {
    return py_str("identity");
    }
    if py_truthy(&_asks_about_its_own_life(&[norm.clone()])) {
    return py_str("life");
    }
    if py_truthy(&{ let __b = _has_any(&[bare.clone(), g_ABOUT_BOT()]); if py_truthy(&__b) { __b } else { mcall(&mcall(&text.clone(), "strip", &[]), "endswith", &[g_ABOUT_BOT_END()]) } }) {
    return py_str("identity");
    }
    if py_truthy(&{ let __b = bitand(&py_set(vec![py_str("you"), py_str("your")]), &words.clone()); if !py_truthy(&__b) { __b } else { { let __b = bitand(&words.clone(), &g_ROBOT_WORDS()); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &py_str("are"))) } } } }) {
    return py_str("identity");
    }
    return Py::None;
    Py::None
}
pub fn _without_mood_tail(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut changed: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut size: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The clause as it reads without the words people soften a feeling with on the end.");
    words = call_builtin("list", &[clause.clone()]).unwrap_or_else(|e| exc::raise(e));
    changed = Py::Bool(true);
    while py_truthy(&{ let __b = changed.clone(); if !py_truthy(&__b) { __b } else { words.clone() } }) {
    changed = Py::Bool(false);
    if py_truthy(&Py::Bool(py_contains(&g_MOOD_TAIL_WORDS(), &geti(&words.clone(), &neg(&Py::Int(1)))))) {
    let _ = mcall(&words.clone(), "pop", &[]);
    changed = Py::Bool(true);
    continue;
    }
    let __iter1 = py_iter_or_raise(&g_MOOD_TAIL());
    for __item in __iter1 {
    phrase = __item.clone();
    size = call_builtin("len", &[mcall(&phrase.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = ge(&size.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&slice_or_raise(&words.clone(), &neg(&size.clone()), &Py::None, &Py::None), &mcall(&phrase.clone(), "split", &[]))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_MOOD_TAIL_TAKEN(), &geti(&words.clone(), &sub(&neg(&size.clone()), &Py::Int(1)))))) {
    continue;
    }
    let _ = py_delitem(&words.clone(), &Py::None).unwrap_or(());
    changed = Py::Bool(true);
    break;
    }
    }
    return words.clone();
    Py::None
}
pub fn _last_subject_pos(args: &[Py]) -> Py {
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
    let mut pos: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Where the last word of a clause that could be the thing its state is about sits.");
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), neg(&Py::Int(1)), neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    pos = __item.clone();
    word = geti(&words.clone(), &pos.clone());
    if py_truthy(&Py::Bool(py_contains(&g_BEFORE_STATE(), &word.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "endswith", &[py_str("ly")]); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("ing")]) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&mcall(&word.clone(), "replace", &[py_str("."), py_str("")]), "isdigit", &[]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NUMBER_WORDS(), &word.clone())) } }) {
    continue;
    }
    return pos.clone();
    }
    return Py::None;
    Py::None
}
pub fn _last_subject(args: &[Py]) -> Py {
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
    let mut pos: Py = Py::None;
    let _ = py_str("The last word of a clause that could be the thing its state is about.");
    pos = _last_subject_pos(&[words.clone()]);
    return if py_truthy(&Py::Bool(py_is(&pos.clone(), &Py::None))) { Py::None } else { geti(&words.clone(), &pos.clone()) };
    Py::None
}
pub fn _particle_is_a_place(args: &[Py]) -> Py {
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
    let mut pos: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pos") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pos"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when the particle at `pos` is a place or a movement, not how somebody is.\n\n    \"im going down the pub\", \"im off down the road\", \"walking up the hill\", \"down the stairs\": a\n    word of going in front of it, or an article and a noun that is not a length of time behind\n    it. \"im down the pub\" is the same line with the going left out.\n    ");
    if py_truthy(&{ let __b = pos.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_MOTION_WORDS(), &geti(&words.clone(), &sub(&pos.clone(), &Py::Int(1))))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = lt(&sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &pos.clone()), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_DETERMINERS(), &geti(&words.clone(), &add(&pos.clone(), &Py::Int(1))))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &add(&pos.clone(), &Py::Int(2)), &add(&pos.clone(), &Py::Int(5)), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_TIME_AFTER_A_PARTICLE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _a_direction(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut pos: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the particle at the character offset `at` is a place, not a feeling.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str(".,!?;:-|\"'")])); } py_list(__out) })() };
    pos = call_builtin("len", &[mcall(&mcall(&slice_or_raise(&text.clone(), &Py::None, &at.clone(), &Py::None), "lower", &[]), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = lt(&pos.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { _particle_is_a_place(&[words.clone(), pos.clone()]) } };
    Py::None
}
pub fn _copula_before(args: &[Py]) -> Py {
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
    let mut pos: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pos") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pos"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when the word before the state is a copula or a feeling verb, not a thing.");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&words.clone(), &Py::None, &pos.clone(), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_COPULA(), &word.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&g_BEFORE_STATE(), &word.clone()))) {
    continue;
    }
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _noun_reading(args: &[Py]) -> Py {
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
    let mut pos: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pos") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pos"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when the word here is a plain noun: \"a new flat\", \"the flat\".");
    if py_truthy(&{ let __b = pos.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DEGREE(), &geti(&words.clone(), &sub(&pos.clone(), &Py::Int(1))))) } }) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), sub(&pos.clone(), &Py::Int(2))]).unwrap_or_else(|e| exc::raise(e)), &pos.clone(), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_DETERMINERS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _thing_turned_down(args: &[Py]) -> Py {
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
    let mut pos: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pos") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pos"))) };
    let _ = py_str("\"turn the heat down\", \"shut it down\": a verb, a thing, then a particle.");
    if py_truthy(&{ let __b = lt(&pos.clone(), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_DETERMINERS(), &geti(&words.clone(), &sub(&pos.clone(), &Py::Int(2))))) } }) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(py_eq(&pos.clone(), &Py::Int(2))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_MANIPULATE(), &geti(&words.clone(), &sub(&pos.clone(), &Py::Int(3))))) } };
    Py::None
}
pub fn _is_person(args: &[Py]) -> Py {
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
    let _ = py_str("True when the word can be the one who has a feeling: \"i\", \"my mum\", \"my son\".");
    return { let __b = Py::Bool(py_contains(&g_PERSON_SUBJECT(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PERSON_NOUNS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_MOOD_NOUNS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_OTHER_PEOPLE(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_set(vec![py_str("that"), py_str("this"), py_str("it")]), &word.clone())) } } } } } } } };
    Py::None
}
pub fn _names_a_thing(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when the word at `at` is a thing that was named: \"the soup\", \"my phone battery\".");
    if py_truthy(&Py::Bool(py_contains(&g_EVENT_NOUNS(), &geti(&words.clone(), &at.clone())))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), sub(&at.clone(), &Py::Int(3))]).unwrap_or_else(|e| exc::raise(e)), &at.clone(), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_DETERMINERS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _names_somebody_else(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let _ = py_str("True when the words up to `at` name a person rather than a thing: \"my 5 year old\".");
    return { let __b = Py::Bool(py_eq(&geti(&words.clone(), &at.clone()), &py_str("old"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), sub(&at.clone(), &Py::Int(3))]).unwrap_or_else(|e| exc::raise(e)), &at.clone(), &Py::None), &py_str("year"))) } };
    Py::None
}
pub fn _whose_state(args: &[Py]) -> Py {
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
    let mut pos: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pos") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pos"))) };
    let mut head: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "head") { v.clone() }
    else { Py::None };
    let mut earlier: Py = Py::None;
    let mut following: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut subject_at: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("\"self\", \"other\" or \"thing\": whose state the word at `pos` is, as far as this clause says.\n\n    `head` is where the state word starts when it is more than one word (\"fed up\"), so the\n    subject is looked for before the whole state and the rest is read after it.\n    ");
    following = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &add(&pos.clone(), &Py::Int(1)), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); __out.push(word.clone()); } py_list(__out) })() };
    subject_at = _last_subject_pos(&[slice_or_raise(&words.clone(), &Py::None, &if py_truthy(&Py::Bool(py_is(&head.clone(), &Py::None))) { pos.clone() } else { head.clone() }, &Py::None)]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&subject_at.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_RELATIVE_SUBJECTS(), &geti(&words.clone(), &subject_at.clone()))) } }) {
    earlier = _last_subject_pos(&[slice_or_raise(&words.clone(), &Py::None, &subject_at.clone(), &Py::None)]);
    if py_truthy(&Py::Bool(!py_is(&earlier.clone(), &Py::None))) {
    subject_at = earlier.clone();
    }
    }
    subject = if py_truthy(&Py::Bool(py_is(&subject_at.clone(), &Py::None))) { Py::None } else { geti(&words.clone(), &subject_at.clone()) };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&subject.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { _copula_before(&[words.clone(), pos.clone()]) } }) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_OTHER_PEOPLE(), &subject.clone())); if py_truthy(&__b) { __b } else { _names_somebody_else(&[words.clone(), subject_at.clone()]) } }) {
    return py_str("other");
    }
    if py_truthy(&_is_person(&[subject.clone()])) {
    return py_str("self");
    }
    if py_truthy(&_names_a_thing(&[words.clone(), subject_at.clone()])) {
    return py_str("thing");
    }
    return py_str("self");
    }
    if py_truthy(&{ let __b = following.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PERSON_PREPOSITIONS(), &geti(&following.clone(), &Py::Int(0)))) } }) {
    if py_truthy(&Py::Bool(py_contains(&g_OTHER_PEOPLE(), &subject.clone()))) {
    return py_str("other");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&subject.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_is_person(&[subject.clone()]))) } }) {
    return py_str("thing");
    }
    return py_str("self");
    }
    let __iter1 = py_iter_or_raise(&following.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_CONJUNCTIONS(), &word.clone()))) {
    break;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_BETWEEN_STATE_AND_NOUN(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_TIME_FILLERS(), &word.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("ly")]); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_DETERMINERS(), &word.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("ing")]); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    return py_str("self");
    }
    if py_truthy(&Py::Bool(py_contains(&g_OTHER_PEOPLE(), &word.clone()))) {
    return py_str("other");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PERSON_NOUNS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_MOOD_NOUNS(), &word.clone())) } }) {
    return py_str("self");
    }
    return py_str("thing");
    }
    if py_truthy(&Py::Bool(py_is(&subject.clone(), &Py::None))) {
    return py_str("self");
    }
    if py_truthy(&Py::Bool(py_contains(&g_OTHER_PEOPLE(), &subject.clone()))) {
    return py_str("other");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PERSON_SUBJECT(), &subject.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PERSON_NOUNS(), &subject.clone())) } }) {
    return py_str("self");
    }
    if py_truthy(&Py::Bool(py_contains(&g_MOOD_NOUNS(), &subject.clone()))) {
    return py_str("self");
    }
    if py_truthy(&Py::Bool(py_contains(&py_set(vec![py_str("that"), py_str("this"), py_str("it")]), &subject.clone()))) {
    return py_str("self");
    }
    if py_truthy(&_names_a_thing(&[words.clone(), subject_at.clone()])) {
    return py_str("thing");
    }
    return py_str("self");
    Py::None
}
pub fn _echoed_state(args: &[Py]) -> Py {
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
    let mut carried: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("\"self\", \"other\" or None: whose state a tail after a state word carries over, if any.\n\n    `words` is what follows the state word (\"and so am i\", \"me too\", \"and the kids too\"). A tail\n    that names no person carries nothing: \"so is the weather\" is not somebody's state.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } }) {
    return Py::None;
    }
    carried = { let __b = { let __b = Py::Bool(py_eq(&geti(&words.clone(), &Py::Int(0)), &py_str("so"))); if !py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BE_FORMS(), &geti(&words.clone(), &Py::Int(1)))) } } } }; if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&slice_or_raise(&words.clone(), &neg(&Py::Int(1)), &Py::None, &Py::None), &py_list(vec![py_str("too")]))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&slice_or_raise(&words.clone(), &neg(&Py::Int(2)), &Py::None, &Py::None), &py_list(vec![py_str("as"), py_str("well")]))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&slice_or_raise(&words.clone(), &neg(&Py::Int(1)), &Py::None, &Py::None), &py_list(vec![py_str("also")]))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&slice_or_raise(&words.clone(), &Py::None, &Py::Int(2), &Py::None), &py_list(vec![py_str("same"), py_str("here")]))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PERSON_SUBJECT(), &geti(&words.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BE_FORMS(), &geti(&words.clone(), &Py::Int(1)))) } } } } } } } } } } } } } };
    if py_truthy(&Py::Bool(!py_truthy(&carried.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_list(vec![py_str("same"), py_str("here")]), py_list(vec![py_str("me"), py_str("too")]), py_list(vec![py_str("same"), py_str("same")])]), &slice_or_raise(&words.clone(), &Py::None, &Py::Int(2), &Py::None)))) {
    return py_str("self");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_PERSON_SUBJECT(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("self");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_OTHER_PEOPLE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("other");
    }
    return Py::None;
    Py::None
}
pub fn _carried_over(args: &[Py]) -> Py {
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
    let mut state_at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "state_at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: state_at"))) };
    let mut tail: Py = Py::None;
    let _ = py_str("The owner an echo after the state word hands the same state to, or None.\n\n    The echo sits in the same clause as the state word more often than not (\"my plant is sad and\n    so am i\" is one clause), so it is read from the words after the state rather than from a\n    clause of its own.\n    ");
    tail = if py_truthy(&Py::Bool(!py_is(&state_at.clone(), &Py::None))) { slice_or_raise(&words.clone(), &add(&state_at.clone(), &Py::Int(1)), &Py::None, &Py::None) } else { words.clone() };
    while py_truthy(&{ let __b = tail.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CONJUNCTIONS(), &geti(&tail.clone(), &Py::Int(0)))) } }) {
    tail = slice_or_raise(&tail.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    return _echoed_state(&[tail.clone()]);
    Py::None
}
pub fn _state_owner(args: &[Py]) -> Py {
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
    let mut carried: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut echo: Py = Py::None;
    let mut head: Py = Py::None;
    let mut owner: Py = Py::None;
    let mut pos: Py = Py::None;
    let mut state_at: Py = Py::None;
    let mut whose: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("\"self\", \"other\", \"thing\" or None: whose state the line names, if it names one.");
    owner = Py::None;
    carried = Py::None;
    if py_truthy(&_works_on_the_speaker(&[mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&clauses.clone()) { let mut clause: Py = __item.clone(); __out.push(if py_truthy(&call_builtin("isinstance", &[clause.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) { mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&clause.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]) } else { call_builtin("str", &[clause.clone()]).unwrap_or_else(|e| exc::raise(e)) }); } py_list(__out) })() }])])) {
    return py_str("self");
    }
    let __iter1 = py_iter_or_raise(&clauses.clone());
    for __item in __iter1 {
    clause = __item.clone();
    words = _without_mood_tail(&[clause.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&owner.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_STATE_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    echo = _echoed_state(&[words.clone()]);
    if py_truthy(&Py::Bool(py_eq(&echo.clone(), &py_str("self")))) {
    return py_str("self");
    }
    if py_truthy(&echo.clone()) {
    owner = echo.clone();
    }
    }
    state_at = Py::None;
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    pos = __unpk3[0].clone();
    word = __unpk3[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g_STATE_WORDS(), &word.clone()))) {
    continue;
    }
    state_at = pos.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOUN_STATES(), &word.clone())); if !py_truthy(&__b) { __b } else { _noun_reading(&[words.clone(), pos.clone()]) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_PARTICLE_STATES(), &word.clone()))) {
    if py_truthy(&_thing_turned_down(&[words.clone(), pos.clone()])) {
    owner = py_str("thing");
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_copula_before(&[words.clone(), pos.clone()])))) {
    continue;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_COPULA_STATES(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&pos.clone(), &Py::Int(0))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_copula_before(&[words.clone(), pos.clone()]))) } } } }) {
    continue;
    }
    head = Py::None;
    if py_truthy(&Py::Bool(py_eq(&word.clone(), &py_str("fed")))) {
    if py_truthy(&Py::Bool(!py_contains(&slice_or_raise(&words.clone(), &add(&pos.clone(), &Py::Int(1)), &add(&pos.clone(), &Py::Int(2)), &Py::None), &py_str("up")))) {
    continue;
    }
    let __unpk4 = py_unpack(&py_tuple(vec![pos.clone(), add(&pos.clone(), &Py::Int(1))]), 2, None);
    head = __unpk4[0].clone();
    pos = __unpk4[1].clone();
    }
    whose = _whose_state(&[words.clone(), pos.clone(), head.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&whose.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&bitor(&g_PERSON_SUBJECT(), &g_PERSON_NOUNS()), &carried.clone())) } }) {
    whose = py_str("self");
    }
    if py_truthy(&Py::Bool(py_eq(&whose.clone(), &py_str("self")))) {
    return py_str("self");
    }
    if py_truthy(&whose.clone()) {
    owner = whose.clone();
    }
    }
    if py_truthy(&Py::Bool(!py_is(&owner.clone(), &Py::None))) {
    echo = _carried_over(&[words.clone(), state_at.clone()]);
    if py_truthy(&Py::Bool(py_eq(&echo.clone(), &py_str("self")))) {
    return py_str("self");
    }
    if py_truthy(&echo.clone()) {
    owner = echo.clone();
    }
    }
    carried = { let __b = _last_subject(&[words.clone()]); if py_truthy(&__b) { __b } else { carried.clone() } };
    }
    return owner.clone();
    Py::None
}
pub fn _about_a_supply(args: &[Py]) -> Py {
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
    let mut end: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "end") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: end"))) };
    let mut rest: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the state word ending at `end` is followed by \"on/in/of\" and a thing.\n\n    \"im low on milk\", \"we are low on petrol\", \"im low in iron\": the state is a shortage of the\n    thing, so it is not the person's mood. The person who has run out of milk is not having a\n    low day, and neither is a car that is low on petrol.\n    ");
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&slice_or_raise(&text.clone(), &end.clone(), &Py::None, &Py::None), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:-|\"'")]), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_SUPPLY_PREPOSITIONS(), &geti(&rest.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(false);
    }
    thing = geti(&rest.clone(), &Py::Int(1));
    return { let __b = Py::Bool(!py_contains(&g_OTHER_PEOPLE(), &thing.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_PERSON_NOUNS(), &thing.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_PERSON_SUBJECT(), &thing.clone())) } } } };
    Py::None
}
pub fn shortage_of(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut out: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing the line says has run out, or None when nothing has.\n\n    `_about_a_supply` already keeps a mood word off \"im low on milk\", but leaving the line with\n    no mood in it is not enough: the composer's own feeling cue (\"im low\") matched anyway and\n    the person who was out of milk got the condolence for a low day. Naming the thing here means\n    every module reads a shortage of milk instead of a mood, once, for every turn.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&text.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:-|\"'")]), "lower", &[])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_SUPPLY_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { ge(&add(&at.clone(), &Py::Int(2)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_contains(&g_SUPPLY_PREPOSITIONS(), &geti(&words.clone(), &add(&at.clone(), &Py::Int(1)))))) {
    continue;
    }
    thing = geti(&words.clone(), &add(&at.clone(), &Py::Int(2)));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_DETERMINERS(), &thing.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_OWN_RESERVES(), &thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OTHER_PEOPLE(), &thing.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PERSON_NOUNS(), &thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PERSON_SUBJECT(), &thing.clone())) } }) {
    continue;
    }
    return thing.clone();
    }
    out = stdlib_call("re", "search", &[py_str("\\b(?:run|ran|running) out of ([a-z]+)"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = out.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_OWN_RESERVES(), &mcall(&out.clone(), "group", &[Py::Int(1)]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_DETERMINERS(), &mcall(&out.clone(), "group", &[Py::Int(1)]))) } } } }) {
    return mcall(&out.clone(), "group", &[Py::Int(1)]);
    }
    return Py::None;
    Py::None
}
pub fn _how_long_itself(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line asks how long the bot itself has been going.\n\n    \"how long have you been going\", \"how long have you been around\", \"how long have you been\n    talking to people\": the answer is the bot's own and there is nothing in the world to measure\n    or look up, so the line is about the bot. A word of the world after the ask (\"how long have\n    you been cooking that stew\") leaves the line to the topic it names.\n    ");
    said = mcall(&py_str(" "), "join", &[norm.clone()]);
    match_ = mcall(&g_ITSELF_LONG(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&sub(&call_builtin("set", &[mcall(&slice_or_raise(&said.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_ITSELF_LIFE())));
    Py::None
}
pub fn _when_itself(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line asks when the bot itself started.\n\n    \"when did you start\", \"when were you built\", \"how long ago did you start\": the answer is the\n    bot's own (nothing came before this chat), so the line is about the bot and the knowledge\n    base has nothing to look up.\n    ");
    said = mcall(&py_str(" "), "join", &[norm.clone()]);
    match_ = mcall(&g_ITSELF_WHEN(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&sub(&call_builtin("set", &[mcall(&slice_or_raise(&said.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_ITSELF_WHEN_TAIL())));
    Py::None
}
pub fn _counts_its_own_doings(args: &[Py]) -> Py {
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
    let mut i: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line counts something of the bot's own life rather than working a sum out.\n\n    \"how many people have you talked to\" is a question about the bot, and read as a number it\n    reached the maths entry and answered a stranger with a porridge recipe. The line has to\n    open on \"how many/much\", count a thing the bot's doings happen to (people, conversations,\n    questions), say \"you\", and name one of those doings.\n    ");
    words = call_builtin("list", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    at = Py::Int(0);
    while py_truthy(&{ let __b = lt(&at.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_COUNT_OPENERS(), &geti(&words.clone(), &at.clone()))) } }) {
    let __aug1 = add(&at.clone(), &Py::Int(1));
    at = __aug1;
    }
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_list(vec![py_str("how"), py_str("many")]), py_list(vec![py_str("how"), py_str("much")])]), &slice_or_raise(&words.clone(), &at.clone(), &add(&at.clone(), &Py::Int(2)), &Py::None)))) {
    return Py::Bool(false);
    }
    rest = slice_or_raise(&words.clone(), &add(&at.clone(), &Py::Int(2)), &Py::None, &Py::None);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bitand(&call_builtin("set", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_BOT_COUNTED()))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&py_set(vec![py_str("you"), py_str("u"), py_str("ya"), py_str("your"), py_str("ur")]), &call_builtin("set", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e))))) } }) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk663 = py_unpack(&__item.clone(), 2, None); let mut i: Py = __unpk663[0].clone(); let mut w: Py = __unpk663[1].clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&g_BOT_DOINGS(), &w.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_BOT_HAVE(), &w.clone())); if !py_truthy(&__b) { __b } else { _a_count_of_chats(&[rest.clone()]) } } } }) {  __out.push(_the_doer_is_the_bot(&[rest.clone(), i.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_count_of_chats(args: &[Py]) -> Py {
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
    let _ = py_str("True when what the line counts is the chat itself (\"how many chats did you have today\").");
    return call_builtin("bool", &[bitand(&call_builtin("set", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_BOT_CHATS())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_doer_is_the_bot(args: &[Py]) -> Py {
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
    let mut doing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "doing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: doing"))) };
    let mut at: Py = Py::None;
    let _ = py_str("Whether the doing at `doing` is the bot's own, or nobody-in-particular's.\n\n    \"how many people have you talked to\" is the bot talking, and \"how many questions do people\n    ask you\" is the bot being asked: both count the bot's day. \"how many questions does the\n    doctor ask you\" is the doctor asking, and the only person it is about is the one typing, so\n    the memory answer would be a reply to a question nobody asked. Walking back over the\n    auxiliaries and the adverbs lands on the doer, and the doer has to be the bot or a word that\n    names nobody in particular.\n    ");
    at = sub(&doing.clone(), &Py::Int(1));
    while py_truthy(&{ let __b = ge(&at.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_COUNT_FILLERS(), &geti(&rest.clone(), &at.clone()))); if py_truthy(&__b) { __b } else { mcall(&geti(&rest.clone(), &at.clone()), "endswith", &[py_str("ly")]) } } } }) {
    let __aug1 = sub(&at.clone(), &Py::Int(1));
    at = __aug1;
    }
    return { let __b = ge(&at.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BOT_DOERS(), &geti(&rest.clone(), &at.clone()))) } };
    Py::None
}
pub fn _asks_about_its_experience(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True for \"have you ever ...\" put to the bot about a life it has not had.\n\n    \"have you ever had a day like that\", \"have you ever been in love\", \"were you ever ill\": the\n    thing asked about is the bot's own experience, and a program has none, so the honest answer\n    is that nothing like that has happened to it. All three went to the search hedge, which asked\n    the person to go and look it up as though they had asked about the world.\n\n    What keeps this from taking turns it should not answer is the vocabulary, the same way the\n    other asks about the bot's life are held: every word after the ask has to be a word of a life\n    (been, had, felt, ill, love, a day) or a word that says nothing on its own. A line that names\n    a thing of the world is a question about that thing and keeps its own answer - \"have you ever\n    eaten a jacket potato\" is the potato recipe, \"have you ever seen a double yolk egg\" is the egg\n    card, \"have you ever heard of truffle oil\" is a knowledge ask. A problem of theirs in the same\n    breath is theirs too (\"have you ever seen my boiler do this\").\n    ");
    words = call_builtin("list", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    said = mcall(&py_str(" "), "join", &[words.clone()]);
    match_ = mcall(&g_EXPERIENCE_ASK(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    rest = mcall(&slice_or_raise(&said.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&bitand(&call_builtin("set", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_THEIR_PROBLEM())) {
    return Py::Bool(false);
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_EXPERIENCE_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_BOT_LIFE(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BOT_LIFE_FILLER(), &word.clone())) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asks_about_its_own_life(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line asks about the bot's own life and names nothing of the world.\n\n    \"do you have kids\", \"have you got a pet\", \"what's your job\", \"do you get paid\": the subject\n    is the bot and the thing asked about is one a program does not have, so there is nothing in\n    the world to look up and a search hedge is no answer. Every word has to be a thing of that\n    kind, so a line that names anything else is left alone.\n    ");
    words = call_builtin("list", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&py_set(vec![py_str("you"), py_str("u"), py_str("ya"), py_str("your"), py_str("ur"), py_str("yours"), py_str("yourself")]), &call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))))) } }) {
    return Py::Bool(false);
    }
    match_ = mcall(&g_BOT_LIFE_ASK(), "match", &[mcall(&py_str(" "), "join", &[words.clone()])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    rest = mcall(&slice_or_raise(&mcall(&py_str(" "), "join", &[words.clone()]), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { sub(&sub(&call_builtin("set", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_BOT_LIFE()), &g_BOT_LIFE_FILLER()) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[bitand(&call_builtin("set", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_BOT_LIFE())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_bots_own_not(args: &[Py]) -> Py {
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
    let mut i: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "i") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: i"))) };
    let _ = py_str("True when the \"not\" at this position is the bot's own negative, not a clause's.\n\n    Three places it stands at: right after the bot (\"youre not chatgpt\", \"are you not real\",\n    \"you not being real is fine\"), right before it because a contraction carries the not in\n    front of the subject (\"cant you sleep\", \"didnt you understand me\"), and after a subject\n    that came first (\"you cant sleep\" is \"you can not sleep\"). The last one is only read that\n    way when nothing opens a clause of the line's own in front of it, because \"what do you do\n    when you cant sleep\" is a question about the night and the bot in it is the person.\n    ");
    if py_truthy(&{ let __b = lt(&add(&i.clone(), &Py::Int(1)), &call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_BEFORE_THE_BOT(), &geti(&norm.clone(), &add(&i.clone(), &Py::Int(1))))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = ge(&i.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_AFTER_THE_BOT(), &geti(&norm.clone(), &sub(&i.clone(), &Py::Int(1))))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = ge(&i.clone(), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NOT_BEFORE_THE_BOT(), &geti(&norm.clone(), &sub(&i.clone(), &Py::Int(2))))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NOT_AUX(), &geti(&norm.clone(), &sub(&i.clone(), &Py::Int(1))))); if !py_truthy(&__b) { __b } else { { let __b = lt(&i.clone(), &Py::Int(3)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_CLAUSE_OPENERS(), &geti(&norm.clone(), &sub(&i.clone(), &Py::Int(3))))) } } } } } } } }) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _recalls_their_own_line(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks for something the person themselves said.");
    return call_builtin("bool", &[mcall(&g__I_SAID_IT(), "match", &[mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() }])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _self_ask(args: &[Py]) -> Py {
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
    let mut i: Py = Py::None;
    let mut left: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the whole line is a question about the bot and names nothing else.\n\n    \"do you have hobbies\", \"who made you\", \"what should i call you\" qualify. \"how do you spell\n    necessary\" and \"what can you do with leftover rice\" do not, because a word outside the\n    bot's own vocabulary is in the line, so the topic keeps them.\n    ");
    words = call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&words.clone(), &py_set(vec![py_str("you"), py_str("your"), py_str("yourself"), py_str("yours")])))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&words.clone(), &g_SELF_ASK())))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = _a_person_they_like(&[norm.clone()]); if py_truthy(&__b) { __b } else { _a_thing_they_own(&[norm.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = norm.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_JOIN_OPENERS(), &geti(&norm.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { bitand(&words.clone(), &g_MAKER_WORDS()) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[sub(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e))) { let mut i: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_POSSESSIVE_WORDS(), &geti(&norm.clone(), &i.clone()))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_PERSON_AFTER_A_POSSESSIVE(), &geti(&norm.clone(), &add(&i.clone(), &Py::Int(1))))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[sub(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e))) { let mut i: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_LEARNING_WORDS(), &geti(&norm.clone(), &i.clone()))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&norm.clone(), &add(&i.clone(), &Py::Int(1))), &py_str("from"))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = bitand(&call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_A_HAVING()); if !py_truthy(&__b) { __b } else { bitand(&call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_CHILD_NOUNS()) } }) {
    return Py::Bool(false);
    }
    left = sub(&sub(&sub(&words.clone(), &g_SELF_WORDS()), &g_SELF_FILLER()), &py_set(vec![py_str("you"), py_str("your"), py_str("yourself"), py_str("yours")]));
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &py_str("not")))) {
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[Py::Int(1), call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut i: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_eq(&geti(&norm.clone(), &i.clone()), &py_str("not"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_bots_own_not(&[norm.clone(), i.clone()]))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    left = sub(&left.clone(), &py_set(vec![py_str("not")]));
    }
    return Py::Bool(!py_truthy(&left.clone()));
    Py::None
}
pub fn _about_bot(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut norm: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { py_tuple(vec![]) };
    let _ = py_str("True when the line asks about the bot itself, not about something it was asked to do.");
    return Py::Bool(!py_is(&_about_bot_kind(&[text.clone(), words.clone(), norm.clone()]), &Py::None));
    Py::None
}
pub fn _asks_if_the_bot_is_a_product(args: &[Py]) -> Py {
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
    let mut tail: Py = Py::None;
    let _ = py_str("True for a yes/no ask that names an assistant product: \"are you claude\".\n\n    \"are you claude\" asks whether there is a model under this, which is the \"model\" kind and not\n    the \"identity\" one, so the answer that says there is no model keeps the turn. The same words\n    as a statement (\"so youre claude\") say what the bot is, not what it is made of.\n    ");
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_PRODUCT_ASK_WORDS()); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("are"), py_str("is")]), &geti(&norm.clone(), &Py::Int(0)))) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&bitand(&call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_ROBOT_WORDS())) {
    return Py::Bool(false);
    }
    tail = call_builtin("list", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    while py_truthy(&{ let __b = tail.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PRODUCT_ASK_TAIL(), &geti(&tail.clone(), &neg(&Py::Int(1))))) } }) {
    let _ = mcall(&tail.clone(), "pop", &[]);
    }
    return { let __b = gt(&call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ASSISTANT_NAMES(), &geti(&tail.clone(), &neg(&Py::Int(1))))) } };
    Py::None
}
pub fn _starts_with(args: &[Py]) -> Py {
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
    let mut phrases: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrases") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrases"))) };
    let mut bare: Py = Py::None;
    let mut p: Py = Py::None;
    bare = mcall(&py_str(" "), "join", &[words.clone()]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrases.clone()) { let mut p: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_eq(&bare.clone(), &p.clone())); if py_truthy(&__b) { __b } else { mcall(&bare.clone(), "startswith", &[add(&p.clone(), &py_str(" "))]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn without_filler(args: &[Py]) -> Py {
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
    let _ = py_str("This line with one interjection taken off the front, or the line itself.");
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BEFORE_COURTESY(), &geti(&norm.clone(), &Py::Int(0)))) } }) {
    return slice_or_raise(&norm.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    return norm.clone();
    Py::None
}
pub fn _a_thing_named_after_the_courtesy(args: &[Py]) -> Py {
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
    let _ = py_str("True when the courtesy word is the first half of a thing's name: \"thank you card\".");
    rest = _after_thanks(&[norm.clone()]);
    return { let __b = call_builtin("bool", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_COURTESY_THING(), &geti(&rest.clone(), &Py::Int(0)))) } };
    Py::None
}
pub fn _after_thanks(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words left when a leading \"thanks\" is taken off, or None if it is not there.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THANKS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    phrase = __item.clone();
    words = mcall(&phrase.clone(), "split", &[]);
    if py_truthy(&Py::Bool(py_eq(&slice_or_raise(&norm.clone(), &Py::None, &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None), &words.clone()))) {
    rest = slice_or_raise(&norm.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None);
    if py_truthy(&{ let __b = slice_or_raise(&rest.clone(), &Py::None, &Py::Int(1), &Py::None); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_COURTESY_THING(), &geti(&rest.clone(), &Py::Int(0)))) } }) {
    return Py::None;
    }
    return rest.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn the_line_after_a_courtesy(args: &[Py]) -> Py {
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
    let _ = py_str("The words a line goes on with after a leading \"thanks\", or None if it starts with one.\n\n    A thank-you is often a wrapper round a line that still needs an answer (\"thanks, my boss\n    is a nightmare\"), and the caller needs to see the line inside it.\n    ");
    return _after_thanks(&[without_filler(&[norm.clone()])]);
    Py::None
}
pub fn _a_fall(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut index: Py = Py::None;
    let mut one: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Did the rest of the line say somebody fell, rather than found or forgot something?");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_FALL(), &word.clone())); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "startswith", &[py_str("collaps")]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_eq(&word.clone(), &py_str("fell")))) {
    after = if py_truthy(&lt(&add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&rest.clone(), &add(&index.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("asleep"), py_str("for"), py_str("into"), py_str("in"), py_str("behind"), py_str("out")]), &after.clone()))) {
    return Py::Bool(true);
    }
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("tripped"), py_str("slipped"), py_str("slipping"), py_str("stumbled"), py_str("stumbling")]), &word.clone()))) {
    after = slice_or_raise(&rest.clone(), &add(&index.clone(), &Py::Int(1)), &add(&index.clone(), &Py::Int(7)), &Py::None);
    if py_truthy(&{ let __b = slice_or_raise(&after.clone(), &Py::None, &Py::Int(1), &Py::None); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SLIPPED_FROM(), &geti(&after.clone(), &Py::Int(0)))) } }) {
    continue;
    }
    if py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&after.clone()) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_FOUND(), &one.clone()))) {  __out.push(one.clone()); } } py_list(__out) })() }) {
    continue;
    }
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _after_greeting(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words left when a leading greeting is taken off, or None if there is not one.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_GREETINGS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    phrase = __item.clone();
    words = mcall(&phrase.clone(), "split", &[]);
    if py_truthy(&Py::Bool(py_eq(&slice_or_raise(&norm.clone(), &Py::None, &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None), &words.clone()))) {
    return slice_or_raise(&norm.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None);
    }
    }
    return Py::None;
    Py::None
}
pub fn after_greeting(args: &[Py]) -> Py {
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
    let _ = py_str("The words a line says after a leading hello, or None when it opens with none.\n\n    For the readers that answer what a line says rather than how it opens: \"hi, I'm\n    vegetarian\" says the diet and the hello only opens it, and a module that can answer the\n    diet needs the rest of the words.\n    ");
    return _after_greeting(&[call_builtin("list", &[{ let __b = norm.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _after_farewell(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words left when a leading farewell is taken off, or None if there is not one.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_FAREWELLS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    phrase = __item.clone();
    words = mcall(&phrase.clone(), "split", &[]);
    if py_truthy(&Py::Bool(py_eq(&slice_or_raise(&norm.clone(), &Py::None, &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None), &words.clone()))) {
    return slice_or_raise(&norm.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None);
    }
    }
    return Py::None;
    Py::None
}
pub fn farewell_only(args: &[Py]) -> Py {
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
    let mut left: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line is a goodbye, not a goodbye wrapped round something to answer.\n\n    A farewell word is an ordinary word in its own right - \"night\" is when the baby wakes,\n    \"later\" is when something happens, \"care\" is what a nurse gives - so the goodbye has to be\n    the whole line. \"night night\", \"good night all\" and \"see you tomorrow\" are goodbyes; \"night\n    feeds are killing me\", \"night shift this week\", \"good night routine for a baby\" and \"take\n    care of your mum\" are lines about something, and the answer belongs to what they say.\n    ");
    rest = _after_farewell(&[norm.clone()]);
    if py_truthy(&Py::Bool(py_is(&rest.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    if py_truthy(&bitand(&call_builtin("set", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_NEEDS_ANSWER())) {
    return Py::Bool(false);
    }
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__WISH_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[left.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&left.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_FAREWELL_TAIL(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_POLITE(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__THANKS_WORDS(), &geti(&rest.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(true);
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_FAREWELL_TAIL(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_POLITE(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__SAID_ON_ITS_OWN(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_TERMS_OF_ADDRESS(), &word.clone())) } } } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn greeting_only(args: &[Py]) -> Py {
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
    let mut lead: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line is a hello, not a hello wrapped round something to answer.\n\n    \"hi\", \"morning all\" and \"hey, quick question\" are greetings, and so is \"hi, I'm Sam\",\n    which gives a name and nothing else. \"hi, my boiler is leaking\" is a report: the rest of\n    the line decides the kind, because a hello-shaped answer to a leaking boiler is useless.\n    ");
    rest = _after_greeting(&[norm.clone()]);
    if py_truthy(&Py::Bool(py_is(&rest.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_GREETING_TAIL(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_POLITE(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = le(&call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&bitand(&call_builtin("set", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_NEEDS_ANSWER()))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_says_what_they_are(&[rest.clone()]))) } } } }) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&g_INTRODUCES());
    for __item in __iter1 {
    lead = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("tuple", &[slice_or_raise(&rest.clone(), &Py::None, &call_builtin("len", &[lead.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &lead.clone()))) {
    continue;
    }
    tail = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&rest.clone(), &call_builtin("len", &[lead.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_NAMING_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&if py_truthy(&le(&Py::Int(1), &{ let __cmp20 = call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e)); __cmp20.clone() })) { le(&__cmp20.clone(), &Py::Int(2)) } else { Py::Bool(false) }))) {
    return Py::Bool(false);
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tail.clone()) { let mut word: Py = __item.clone(); __out.push(_a_name(&[word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    return Py::Bool(false);
    Py::None
}
pub fn _says_what_they_are(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the words after a hello say what the person is, has or does.\n\n    A hello with a name after it is a greeting (\"hi, I'm Sam\"), and the same hello with a fact\n    after it is a person telling you the fact (\"hi, I'm vegetarian\", \"hi, im diabetic\"): the\n    words are the whole of the line in the second, and the hello is only how it opens. Which\n    one it is, is the fact store's reading (`facts.about_themselves`) - the only reading that\n    can tell a name from a fact about the person - and it is the same question the store asks\n    before it says the fact back (`bot/memory/ack.py`). Imported here because the fact store\n    reads this module.\n    ");
    let mut about_themselves: Py = py_func("about_themselves", |args: &[Py]| crate::gen::memory::facts::about_themselves(args));
    return call_builtin("bool", &[py_call(&about_themselves, &[mcall(&py_str(" "), "join", &[rest.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_name(args: &[Py]) -> Py {
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
    let _ = py_str("Whether a word said after \"I'm\" is a name: \"hi, I'm Sam\" against \"hi, I'm a nurse\".");
    let mut reads_as_a_name: Py = py_func("reads_as_a_name", |args: &[Py]| crate::gen::memory::facts::reads_as_a_name(args));
    return call_builtin("bool", &[py_call(&reads_as_a_name, &[word.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _says_the_help_was_good(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the whole line says the last answer helped, without a thanks word.");
    words = call_builtin("list", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    while py_truthy(&{ let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BEFORE_HELP_WAS_GOOD(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    words = slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    while py_truthy(&gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    said = mcall(&py_str(" "), "join", &[words.clone()]);
    tail = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_AFTER_HELP_WAS_GOOD_SAID_OUT()) { let mut phrase: Py = __item.clone(); if py_truthy(&mcall(&said.clone(), "endswith", &[add(&py_str(" "), &phrase.clone())])) {  __out.push(mcall(&phrase.clone(), "split", &[])); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(py_is(&tail.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_AFTER_HELP_WAS_GOOD(), &geti(&words.clone(), &neg(&Py::Int(1))))) } }) {
    tail = py_list(vec![geti(&words.clone(), &neg(&Py::Int(1)))]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&tail.clone(), &Py::None)); if py_truthy(&__b) { __b } else { lt(&sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(2)) } }) {
    break;
    }
    words = slice_or_raise(&words.clone(), &Py::None, &neg(&call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    }
    return Py::Bool(py_contains(&g_HELP_WAS_GOOD(), &mcall(&py_str(" "), "join", &[words.clone()])));
    Py::None
}
pub fn _thanks_only(args: &[Py]) -> Py {
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
    let mut clauses: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clauses") { v.clone() }
    else { Py::None };
    let mut rest: Py = Py::None;
    let _ = py_str("True when \"thanks\" is the whole line, not a wrapper round a problem.\n\n    \"thanks, that worked\" and \"thanks for staying with me\" are thanks; \"thanks, my boiler\n    keeps losing pressure\" reports a fault, and the answer belongs to the boiler, so the\n    rest of the line decides the kind.\n    ");
    rest = _after_thanks(&[norm.clone()]);
    if py_truthy(&Py::Bool(py_is(&rest.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    if py_truthy(&bitand(&call_builtin("set", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_NEEDS_ANSWER())) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_STILL_WRONG(), "search", &[mcall(&py_str(" "), "join", &[rest.clone()])])) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_clause_of_its_own(&[rest.clone(), clauses.clone()])) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&_a_fall(&[rest.clone()])));
    Py::None
}
pub fn _a_clause_of_its_own(args: &[Py]) -> Py {
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
    let mut clauses: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clauses") { v.clone() }
    else { Py::None };
    let mut clause: Py = Py::None;
    let _ = py_str("True when the rest of a line that opens with a thank-you says something of its own.\n\n    A thank-you at the front of a line is a wrapper round what follows it, and what follows it\n    is often the point: \"cheers, im skint till friday\" is news about the week, and read as a\n    thank-you it was answered with the answer to the turn before, all over again. The\n    thank-you does hold what comes after it when that is short (\"thanks, it worked\"), opens\n    with what the thanks is about (\"thanks, that explains the boiler\"), is agreement (\"thanks,\n    that worked a treat\"), or says things are better (\"thanks, i feel much better now\"). A\n    clause of four words or more that does none of those is the line, and the line needs an\n    answer.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_THANKS_ABOUT(), &geti(&rest.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&_starts_with(&[rest.clone(), g_THANKS()])) {
    return Py::Bool(false);
    }
    if py_truthy(&lt(&call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_step_done(&[rest.clone()])) {
    return _the_step_went_wrong(&[rest.clone()]);
    }
    if py_truthy(&{ let __b = _agrees(&[rest.clone()]); if py_truthy(&__b) { __b } else { bitand(&call_builtin("set", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_BETTER()) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&_praises_the_thing(&[rest.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&bitand(&call_builtin("set", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_SAYS_ABOUT_YOU())) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_fall_that_is_not_one(&[rest.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&rest.clone(), &py_str("temperature")))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_SIGNING_OFF(), "search", &[mcall(&py_str(" "), "join", &[rest.clone()])])) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = clauses.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter1 {
    clause = __item.clone();
    if py_truthy(&{ let __b = ge(&call_builtin("len", &[clause.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_THANKS_ABOUT(), &geti(&clause.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_step_done(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let _ = py_str("True when the rest of the line reports doing a step of the advice, and nothing went wrong.\n\n    \"thanks, i cleared the cache\" and \"cheers, i restarted the router\" are the person saying they\n    did what the bot suggested. The step is named as a thing being acted on (\"the\", \"my\", \"it\"),\n    so a line about the person's own day keeps its own answer: \"thanks, i booked a table for\n    friday\" is still news, and \"thanks, i tried the new place on saturday\" is a thing they did\n    rather than a report back. A step that went wrong is the news, not the thank-you.\n    ");
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("i"), py_str("we")]), &geti(&rest.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_STEP_VERBS(), &geti(&rest.clone(), &Py::Int(1)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&rest.clone(), &Py::Int(1)), &py_str("gave"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&slice_or_raise(&rest.clone(), &Py::Int(2), &Py::Int(4), &Py::None), &py_str("go"))) } })) } }) {
    return Py::Bool(false);
    }
    after = slice_or_raise(&rest.clone(), &Py::Int(2), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("tried"), py_str("swapped")]), &geti(&rest.clone(), &Py::Int(1))))) {
    return Py::Bool(py_eq(&slice_or_raise(&after.clone(), &Py::None, &Py::Int(1), &Py::None), &py_list(vec![py_str("it")])));
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_STEP_ANY_OBJECT(), &geti(&rest.clone(), &Py::Int(1)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_list(vec![py_str("it")]), py_list(vec![py_str("the")])]), &slice_or_raise(&after.clone(), &Py::None, &Py::Int(1), &Py::None))) } }) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(py_contains(&after.clone(), &py_str("the"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&after.clone(), &py_str("my"))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&after.clone(), &Py::None, &Py::Int(1), &Py::None), &py_list(vec![py_str("it")]))) } } } };
    Py::None
}
pub fn _the_step_went_wrong(args: &[Py]) -> Py {
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
    let _ = py_str("True when a step of the advice is reported as having failed.");
    return call_builtin("bool", &[bitand(&call_builtin("set", &[slice_or_raise(&rest.clone(), &Py::Int(2), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_WRONG())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_fall_that_is_not_one(args: &[Py]) -> Py {
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
    let _ = py_str("True when the rest names a fall that is not one, so it stays part of the thank-you.\n\n    \"thanks, i fell asleep at nine\" and \"thanks, i fell for it\" say something went well; the\n    words are the ones an accident uses, and an accident is what makes the line news, so the\n    rest of the line has to say what kind of fall it was. _a_fall already tells those two from\n    \"thanks, i slipped in the garden\".\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&call_builtin("set", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_FELL_WORDS())))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&_a_fall(&[rest.clone()])));
    Py::None
}
pub fn _praises_the_thing(args: &[Py]) -> Py {
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
    let _ = py_str("True when what follows the thank-you is a compliment about a thing, not news of its own.\n\n    \"thanks, the oven temperature was perfect\" and \"thanks, the cake was delicious\" praise the\n    thing the bot helped with and need no answer, so the thank-you still holds the line. Praise\n    said of the speaker (\"thanks, im good now\") is a thank-you too. A thing praised and then\n    complained about in the same breath is the complaint: \"the meal was good but the service was\n    slow\".\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = bitand(&call_builtin("set", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &bitor(&g_PRAISING(), &g_PRAISING_VERB())); if py_truthy(&__b) { __b } else { mcall(&g_PRAISE_SAID(), "search", &[mcall(&py_str(" "), "join", &[rest.clone()])]) } }))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&bitand(&call_builtin("set", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_TURNED_ROUND())));
    Py::None
}
pub fn _agrees(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether every word before the thanks is agreement, so the thanks is the whole answer.\n\n    \"not\" is the hard part: \"not bad thanks\" agrees, and \"still not working thanks\" does not,\n    because the person has just told you their thing is still broken. So what follows \"not\" is\n    the word that decides.\n    ");
    if py_truthy(&Py::Bool(!py_contains(&rest.clone(), &py_str("not")))) {
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_AGREE_TAIL(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    after = slice_or_raise(&rest.clone(), &add(&mcall(&rest.clone(), "index", &[py_str("not")]), &Py::Int(1)), &Py::None, &Py::None);
    while py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_FILLER(), &geti(&after.clone(), &Py::Int(0)))) } }) {
    after = slice_or_raise(&after.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_AGREE_AFTER_NOT(), &geti(&after.clone(), &Py::Int(0)))) } }))) {
    return Py::Bool(false);
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&after.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_AGREE_TAIL(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_AGREE_AFTER_NOT(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _ends_with_thanks(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut named: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said_to: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a short line ends in thanks: \"pretty good thanks\", \"brilliant thanks\".\n\n    People answer the bot and thank it in one breath, and the tail is the whole point of the\n    line: \"loving it thanks\" is thanks, not news.\n    ");
    said_to = call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    while py_truthy(&{ let __b = said_to.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ADDRESSED(), &geti(&norm.clone(), &sub(&said_to.clone(), &Py::Int(1))))) } }) {
    let __aug1 = sub(&said_to.clone(), &Py::Int(1));
    said_to = __aug1;
    }
    line = slice_or_raise(&norm.clone(), &Py::None, &said_to.clone(), &Py::None);
    named = lt(&said_to.clone(), &call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)));
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THANKS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    phrase = __item.clone();
    words = mcall(&phrase.clone(), "split", &[]);
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[line.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&line.clone(), &neg(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None, &Py::None), &words.clone())) } }) {
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&line.clone(), &Py::None, &neg(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None)) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_THANKS_NOISE(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&gt(&call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6))) {
    continue;
    }
    if py_truthy(&{ let __b = le(&call_builtin("set", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &bitor(&g_AFFIRM(), &g_ACK())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&named.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = slice_or_raise(&rest.clone(), &Py::None, &Py::Int(1), &Py::None); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_set(vec![py_str("no"), py_str("nope"), py_str("nah"), py_str("never"), py_str("naw")]), &geti(&rest.clone(), &Py::Int(0)))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_agrees(&[rest.clone()])))) {
    continue;
    }
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _talk_opening(args: &[Py]) -> Py {
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
    let mut changed: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the whole line asks for room to talk, with nothing named yet in it.");
    words = mcall(&text.clone(), "split", &[]);
    changed = Py::Bool(true);
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { changed.clone() } }) {
    changed = Py::Bool(false);
    if py_truthy(&Py::Bool(py_contains(&g_TALK_HEAD(), &geti(&words.clone(), &Py::Int(0))))) {
    let __unpk1 = py_unpack(&py_tuple(vec![slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None), Py::Bool(true)]), 2, None);
    words = __unpk1[0].clone();
    changed = __unpk1[1].clone();
    }
    let __iter2 = py_iter_or_raise(&g_TALK_TAIL());
    for __item in __iter2 {
    tail = __item.clone();
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &add(&call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("tuple", &[slice_or_raise(&words.clone(), &neg(&call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &tail.clone())) } }) {
    let __unpk3 = py_unpack(&py_tuple(vec![slice_or_raise(&words.clone(), &Py::None, &neg(&call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None), Py::Bool(true)]), 2, None);
    words = __unpk3[0].clone();
    changed = __unpk3[1].clone();
    break;
    }
    }
    }
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_TALK_OPENING(), "fullmatch", &[mcall(&py_str(" "), "join", &[words.clone()])])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn talk_frame(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the whole line asks for room to talk and names nothing in it.\n\n    `opening_line` uses this, and so does the reading: the frame is the whole line, so a word\n    inside it (\"run\", \"brain\", \"share\") is not a subject, and an answer that matched one of them\n    (\"can i run something by you\" found the fitness entry) is not an answer to this line.\n    ");
    text = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() }]);
    return { let __b = call_builtin("bool", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _talk_opening(&[text.clone()]) } };
    Py::None
}
pub fn opening_line(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line only says a question is coming, or asks for advice in general.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    text = mcall(&py_str(" "), "join", &[words.clone()]);
    if py_truthy(&_talk_opening(&[text.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_OPENING_MAX())) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&py_set(vec![py_str("have"), py_str("do"), py_str("does"), py_str("did")]), &geti(&words.clone(), &Py::Int(0))))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_OPENING_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_OPENING_FRAME(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _short_reply(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let _ = py_str("True when the whole line is one of the set replies people give after a question.");
    let __iter1 = py_iter_or_raise(&g_SHORT_REPLIES());
    for __item in __iter1 {
    phrase = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&text.clone(), &phrase.clone())); if py_truthy(&__b) { __b } else { mcall(&text.clone(), "startswith", &[add(&phrase.clone(), &py_str(" "))]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _cant_for_cannot(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut joined: Py = Py::None;
    let _ = py_str("The line as one string, with a split \"can not\" written back as \"cant\".\n\n    The reading takes the apostrophe out (\"cant\" arrives as `can not`), and the phrases that\n    carry an agreement are typed with the apostrophe left out, so the two have to be joined.\n    ");
    joined = py_list(vec![]);
    index = Py::Int(0);
    while py_truthy(&lt(&index.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&words.clone(), &index.clone()), &py_str("can"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&words.clone(), &add(&index.clone(), &Py::Int(1)), &add(&index.clone(), &Py::Int(2)), &Py::None), &py_list(vec![py_str("not")]))) } }) {
    let _ = mcall(&joined.clone(), "append", &[py_str("cant")]);
    let __aug1 = add(&index.clone(), &Py::Int(2));
    index = __aug1;
    continue;
    }
    let _ = mcall(&joined.clone(), "append", &[geti(&words.clone(), &index.clone())]);
    let __aug2 = add(&index.clone(), &Py::Int(1));
    index = __aug2;
    }
    return mcall(&py_str(" "), "join", &[joined.clone()]);
    Py::None
}
pub fn _agrees_to_try(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut hedged: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line is the person agreeing to do what was just suggested.\n\n    \"will give it a go\", \"gonna try that\", \"ok give it a go\": the subject is dropped and the\n    whole line is the commitment, so the only thing it can answer is the bot's last turn.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&word.clone(), &py_str("|"))) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(9)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_QUESTION_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_COMMIT_STOP(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    bare = mcall(&py_str(" "), "join", &[words.clone()]);
    hedged = call_builtin("list", &[words.clone()]).unwrap_or_else(|e| exc::raise(e));
    while py_truthy(&hedged.clone()) {
    if py_truthy(&Py::Bool(py_contains(&g_HEDGE_LEAD_ONE(), &geti(&hedged.clone(), &Py::Int(0))))) {
    hedged = slice_or_raise(&hedged.clone(), &Py::Int(1), &Py::None, &Py::None);
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_HEDGE_LEAD_TWO(), &mcall(&py_str(" "), "join", &[slice_or_raise(&hedged.clone(), &Py::None, &Py::Int(2), &Py::None)])))) {
    hedged = slice_or_raise(&hedged.clone(), &Py::Int(2), &Py::None, &Py::None);
    } else {
    break;
    }
    }
    }
    said = _cant_for_cannot(&[hedged.clone()]);
    if py_truthy(&{ let __b = slice_or_raise(&mcall(&said.clone(), "split", &[]), &Py::None, &Py::Int(1), &Py::None); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NO_HARM_SUBJECT(), &geti(&mcall(&said.clone(), "split", &[]), &Py::Int(0)))) } }) {
    said = mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&said.clone(), "split", &[]), &Py::Int(1), &Py::None, &Py::None)]);
    }
    if py_truthy(&{ let __b = if py_truthy(&lt(&Py::Int(0), &{ let __cmp10 = call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)); __cmp10.clone() })) { le(&__cmp10.clone(), &Py::Int(7)) } else { Py::Bool(false) }; if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_COMMIT_NO_HARM()) { let mut phrase: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_eq(&said.clone(), &phrase.clone())); if py_truthy(&__b) { __b } else { mcall(&said.clone(), "startswith", &[add(&phrase.clone(), &py_str(" "))]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&g_COMMIT_ALONE(), &bare.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_COMMIT_ALONE()) { let mut phrase: Py = __item.clone(); __out.push({ let __b = mcall(&bare.clone(), "startswith", &[add(&phrase.clone(), &py_str(" "))]); if !py_truthy(&__b) { __b } else { le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_COMMIT_ALONE_TAIL()) { let mut phrase: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_eq(&bare.clone(), &phrase.clone())); if py_truthy(&__b) { __b } else { mcall(&bare.clone(), "startswith", &[add(&phrase.clone(), &py_str(" "))]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    lead = crate::gen::understanding::normalize::strip_leaders(&[words.clone()]);
    if py_truthy(&{ let __b = slice_or_raise(&lead.clone(), &Py::None, &Py::Int(1), &Py::None); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_AFFIRM(), &geti(&lead.clone(), &Py::Int(0)))) } }) {
    lead = slice_or_raise(&lead.clone(), &Py::Int(1), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(py_eq(&slice_or_raise(&lead.clone(), &Py::None, &Py::Int(1), &Py::None), &py_list(vec![py_str("one")])))) {
    lead = slice_or_raise(&lead.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    }
    if py_truthy(&{ let __b = slice_or_raise(&lead.clone(), &Py::None, &Py::Int(1), &Py::None); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_COMMIT_SUBJECT(), &geti(&lead.clone(), &Py::Int(0)))) } }) {
    lead = slice_or_raise(&lead.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&{ let __b = slice_or_raise(&lead.clone(), &Py::None, &Py::Int(1), &Py::None); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_HEDGED_COMMIT(), &geti(&lead.clone(), &Py::Int(0)))) } }) {
    rest = mcall(&py_str(" "), "join", &[slice_or_raise(&lead.clone(), &Py::Int(1), &Py::None, &Py::None)]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_HEDGED_ALONE()) { let mut phrase: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_eq(&rest.clone(), &phrase.clone())); if py_truthy(&__b) { __b } else { mcall(&rest.clone(), "startswith", &[add(&phrase.clone(), &py_str(" "))]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&{ let __b = slice_or_raise(&lead.clone(), &Py::None, &Py::Int(1), &Py::None); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_COMMIT_SUBJECT(), &geti(&lead.clone(), &Py::Int(0)))) } }) {
    lead = slice_or_raise(&lead.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&lead.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_COMMIT_OPEN(), &geti(&lead.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(false);
    }
    lead = slice_or_raise(&lead.clone(), &Py::Int(1), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(py_eq(&slice_or_raise(&lead.clone(), &Py::None, &Py::Int(1), &Py::None), &py_list(vec![py_str("to")])))) {
    lead = slice_or_raise(&lead.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    return { let __b = call_builtin("bool", &[lead.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_COMMIT_TRY(), &geti(&lead.clone(), &Py::Int(0)))) } };
    Py::None
}
pub fn _asks_the_room(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut stripped: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "stripped") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: stripped"))) };
    let _ = py_str("True when an open collective subject asks the room for something.\n\n    \"anyone/anybody know a plumber\" asks. \"someone got hurt\", \"everyone got the memo\",\n    \"some1 found my keys\" are reports: only a question word after the verb (\"some1 know\n    where to get tyres\") turns a someone/everyone line into a question.\n    ");
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[stripped.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_ASK_THE_ROOM(), &geti(&stripped.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_contains(&g_ASK_THE_ROOM_VERB(), &geti(&stripped.clone(), &Py::Int(1))))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g_ASK_THE_ROOM_ANY(), &geti(&stripped.clone(), &Py::Int(0))))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_contains(&g_ASK_FOR_A_NAME(), &geti(&stripped.clone(), &Py::Int(1))))) {
    return Py::Bool(false);
    }
    return { let __b = gt(&call_builtin("len", &[stripped.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ASK_THE_ROOM_WANTS(), &geti(&stripped.clone(), &Py::Int(2)))) } };
    Py::None
}
pub fn _asks_at_the_end(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut stripped: Py = Py::None;
    let _ = py_str("True when the last clause asks for the answer without a question word.");
    stripped = { let __b = crate::gen::understanding::normalize::strip_leaders(&[clause.clone()]); if py_truthy(&__b) { __b } else { clause.clone() } };
    if py_truthy(&Py::Bool(!py_truthy(&stripped.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&_opens_question(&[stripped.clone()])) {
    return Py::Bool(true);
    }
    return { let __b = call_builtin("bool", &[bitand(&call_builtin("set", &[stripped.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_TAIL_ASKS())]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { le(&call_builtin("set", &[stripped.clone()]).unwrap_or_else(|e| exc::raise(e)), &bitor(&g_TAIL_ASKS(), &g_TAIL_ASK_FILLER())) } };
    Py::None
}
pub fn _says_nothing_but_the_floor(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut words: Py = Py::None;
    let _ = py_str("Whether the clause is only the words that hold a conversation together.\n\n    \"thanks, what should i cook tonight\" says nothing about a situation, and neither does\n    \"ok thanks. also, should i worry about the damp patch\" - the courtesy and the \"also\" are the\n    turn, not the situation, and the answer must not be held to them.\n    ");
    words = call_builtin("set", &[{ let __b = crate::gen::understanding::normalize::strip_leaders(&[clause.clone()]); if py_truthy(&__b) { __b } else { clause.clone() } }]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { le(&words.clone(), &bitor(&g_SETUP_NOTHING(), &g_SETUP_CONNECTIVES())) } };
    Py::None
}
pub fn the_setup(args: &[Py]) -> Py {
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
    let mut ask_at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut index: Py = Py::None;
    let mut setup: Py = Py::None;
    let _ = py_str("The situation a line describes before the question it ends on, else None.\n\n    \"friends are coming for sunday lunch and one of them is vegan, what can i cook that everyone\n    will eat\" says the situation and then asks; the ask is what the line is about (it is\n    `focus`) and the setup is what the answer has to hold to. Read whole, the situation's words\n    are the ones a reply is found by, and the answer goes to the situation instead of the\n    question: \"my son has a football match on saturday and its freezing, what should he wear\" is\n    answered with the temperature a baby's room should be, because the only thing the line named\n    that a card knows about is the freezing.\n\n    Two questions in one line have no setup (\"what is your name and how old are you\"): the\n    second is another ask, not the condition for the first.\n    ");
    clauses = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&line.clone(), "get", &[py_str("clauses")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut clause: Py = __item.clone(); if py_truthy(&clause.clone()) {  __out.push(clause.clone()); } } py_list(__out) })() };
    if py_truthy(&lt(&call_builtin("len", &[clauses.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::None;
    }
    ask_at = Py::None;
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[sub(&call_builtin("len", &[clauses.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), Py::Int(0), neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&_asks_at_the_end(&[geti(&clauses.clone(), &index.clone())])) {
    ask_at = index.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(py_is(&ask_at.clone(), &Py::None))) {
    return Py::None;
    }
    before = slice_or_raise(&clauses.clone(), &Py::None, &ask_at.clone(), &Py::None);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&before.clone()) { let mut clause: Py = __item.clone(); __out.push(_opens_question(&[clause.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&before.clone()) { let mut clause: Py = __item.clone(); __out.push(_says_nothing_but_the_floor(&[clause.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    setup = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&before.clone()) { let mut clause: Py = __item.clone(); __out.push(mcall(&py_str(" "), "join", &[{ let __b = crate::gen::understanding::normalize::strip_leaders(&[clause.clone()]); if py_truthy(&__b) { __b } else { clause.clone() } }])); } py_list(__out) })() }]);
    setup = mcall(&mcall(&g_A_FRAME_IN_FRONT(), "sub", &[py_str(""), setup.clone()]), "strip", &[py_str(" ,-")]);
    if py_truthy(&Py::Bool(!py_truthy(&setup.clone()))) {
    return Py::None;
    }
    if py_truthy(&gt(&call_builtin("len", &[mcall(&setup.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_SETUP_MOST_WORDS())) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_FRAME_NOT_A_SETUP(), "match", &[add(&py_str(" "), &setup.clone())])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&sub(&call_builtin("set", &[mcall(&setup.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_SETUP_CONNECTIVES())))) {
    return Py::None;
    }
    return setup.clone();
    Py::None
}
pub fn _opens_question(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut stripped: Py = Py::None;
    let _ = py_str("True when a clause opens like a question (\"is 8gb enough\"), not like an order.");
    stripped = { let __b = crate::gen::understanding::normalize::strip_leaders(&[clause.clone()]); if py_truthy(&__b) { __b } else { clause.clone() } };
    if py_truthy(&_asks_the_room(&[stripped.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&stripped.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_QUESTION_STARTS(), &geti(&stripped.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[stripped.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NEGATIONS(), &geti(&stripped.clone(), &Py::Int(1)))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_SUBJECT_NEGATIONS(), &geti(&stripped.clone(), &Py::Int(1)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_PRONOUNS(), &geti(&stripped.clone(), &Py::Int(2)))) } } } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&stripped.clone(), &Py::Int(0)), &py_str("do"))); if !py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[stripped.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_QUESTION_WORDS(), &geti(&stripped.clone(), &Py::Int(1)))) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_YESNO_STARTS(), &geti(&stripped.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[stripped.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { _verb_form(&[geti(&stripped.clone(), &Py::Int(1)), geti(&stripped.clone(), &Py::Int(0))]) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[stripped.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { crate::gen::understanding::normalize::opens_without_subject(&[stripped.clone(), Py::Int(0)]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _hands_a_question_to_the_speaker(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut handed_to: Py = Py::None;
    let mut modal: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line itself asks for a question to answer: \"quiz me\", \"give me a question\".\n\n    The verb has to open the line, or stand behind nothing but a modal or a courtesy, so the\n    person is the one doing the asking. \"she quizzed me\" and \"he asked me a question\" are a\n    question that already happened to somebody, and reading them as a request to be quizzed put\n    a general-knowledge question in front of a person who had said nothing of the kind.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&word.clone(), &py_str("|"))) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[slice_or_raise(&words.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g_HAND_OVER_VERBS(), &word.clone()))) {
    continue;
    }
    handed_to = { let __b = Py::Bool(py_contains(&g_HAND_OVER_TO(), &geti(&words.clone(), &add(&at.clone(), &Py::Int(1))))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("my"), py_str("our")]), &geti(&words.clone(), &add(&at.clone(), &Py::Int(1))))); if !py_truthy(&__b) { __b } else { { let __b = lt(&add(&at.clone(), &Py::Int(2)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_HAND_OVER_MINE(), &geti(&words.clone(), &add(&at.clone(), &Py::Int(2))))) } } } } } };
    if py_truthy(&Py::Bool(!py_truthy(&handed_to.clone()))) {
    continue;
    }
    before = slice_or_raise(&words.clone(), &Py::None, &at.clone(), &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&before.clone()))) {
    return Py::Bool(true);
    }
    modal = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&before.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_HAND_OVER_MODALS(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&before.clone()) { let mut w: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_HAND_OVER_LEADS(), &w.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_HAND_OVER_MODALS(), &w.clone())); if py_truthy(&__b) { __b } else { { let __b = modal.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_HAND_OVER_SUBJECTS(), &w.clone())) } } } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&{ let __b = modal.clone(); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&before.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_HAND_OVER_SUBJECTS(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _asks_a_question_coming(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut after: Py = Py::None;
    let mut asked_here: Py = Py::None;
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut named: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line says a question is coming without asking it yet.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&if py_truthy(&lt(&Py::Int(0), &{ let __cmp10 = call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)); __cmp10.clone() })) { le(&__cmp10.clone(), &Py::Int(12)) } else { Py::Bool(false) })); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { bitand(&call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_ALREADY_ASKED()) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&_hands_a_question_to_the_speaker(&[norm.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&py_set(vec![py_str("do"), py_str("does"), py_str("did"), py_str("any")]), &geti(&norm.clone(), &Py::Int(0))))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_set(vec![py_str("have"), py_str("has")]), &geti(&norm.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_set(vec![py_str("you"), py_str("u"), py_str("anyone"), py_str("anybody"), py_str("any"), py_str("they")]), &geti(&norm.clone(), &Py::Int(1)))) } } } }) {
    return Py::Bool(false);
    }
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[slice_or_raise(&norm.clone(), &Py::None, &Py::Int(5), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    at = __unpk3[0].clone();
    word = __unpk3[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &py_str("asked"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&at.clone(), &Py::Int(1))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_set(vec![py_str("i"), py_str("we")]), &geti(&norm.clone(), &Py::Int(0)))) } } } }) {
    continue;
    }
    if py_truthy(&bitand(&call_builtin("set", &[slice_or_raise(&norm.clone(), &Py::None, &at.clone(), &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &g_PAST_ASK())) {
    continue;
    }
    after = if py_truthy(&lt(&add(&at.clone(), &Py::Int(1)), &call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&norm.clone(), &add(&at.clone(), &Py::Int(1))) } else { py_str("") };
    asked_here = { let __b = Py::Bool(py_contains(&g_ASKING_VERBS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&bitor(&g_ASK_TARGET(), &g_ASK_ABOUT()), &after.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&at.clone(), &Py::Int(0))); if py_truthy(&__b) { __b } else { { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&norm.clone(), &Py::None, &at.clone(), &Py::None)) { let mut before: Py = __item.clone(); __out.push(Py::Bool(py_contains(&bitor(&g_ASK_MODALS(), &g_ASK_AUX()), &before.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ASK_TARGET(), &after.clone())) } } } } } } } };
    named = { let __b = Py::Bool(py_contains(&g_QUESTION_NOUNS(), &word.clone())); if py_truthy(&__b) { __b } else { asked_here.clone() } };
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&at.clone(), &Py::Int(0))); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&norm.clone(), &Py::None, &at.clone(), &Py::None)) { let mut before: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_ASK_FRAMES(), &before.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _asks_for_tips(args: &[Py]) -> Py {
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
    let mut norm: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { py_tuple(vec![]) };
    let mut about: Py = Py::None;
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a tip word here asks for advice, not a tip being left or given.\n\n    \"any tips for a roast\", \"tips for a job interview\", \"got any hints about sourdough\" and\n    \"any tip on painting a door\" ask; \"a 10% tip\", \"he tips well\", \"tip the delivery driver\"\n    and \"the tip of my knife broke\" are about a thing or an action.\n\n    A bare \"any tips\" with nothing in the line it is tips about is not read as an ask: the\n    subject it follows in the conversation (a bad night's sleep) is what an answer needs, and\n    turning the line into a request only swaps the ack for \"I would rather not guess\".\n    ");
    about = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_TIPS_FRAME(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&mcall(&g_TIPS(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    let __unpk2 = py_unpack(&py_tuple(vec![slice_or_raise(&text.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), slice_or_raise(&text.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)]), 2, None);
    before = __unpk2[0].clone();
    after = __unpk2[1].clone();
    if py_truthy(&mcall(&g_TIPS_AMOUNT(), "search", &[before.clone()])) {
    continue;
    }
    if py_truthy(&mcall(&g_TIPS_WANTED(), "search", &[before.clone()])) {
    if py_truthy(&about.clone()) {
    return Py::Bool(true);
    }
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("tips"), py_str("hints"), py_str("pointers")]), &mcall(&match_.clone(), "group", &[Py::Int(1)])))) {
    if py_truthy(&mcall(&g_TIPS_SUBJECT(), "search", &[before.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g_TIPS_ABOUT(), "match", &[after.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&after.clone(), "strip", &[py_str(" ?.!,")]))) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _asks_the_bot(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks the bot something in the second person.");
    if py_truthy(&lt(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(py_contains(&g_YOU_WORDS(), &geti(&norm.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_ASK_VERBS(), &geti(&norm.clone(), &Py::Int(1)))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ASK_FOLLOW(), &geti(&norm.clone(), &Py::Int(2)))) } } } };
    Py::None
}
pub fn after_any_link(args: &[Py]) -> Py {
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
    let _ = py_str("The words after the link in \"any news about X\", or () when the line is another shape.\n\n    The tuple is what the line is about: \"any word on the job\" is about the job, and read whole\n    the topic was the word it was asked with. A line that only says the kind of answer wanted\n    (\"any luck?\") has nothing after the link, so it keeps the conversation's subject, and so does\n    one that puts the whole ask after it (\"any idea how to stop him\"): there the ask is the line\n    and the thing it is about came from the turns before.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&norm.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("any"), py_str("anything")]), &geti(&norm.clone(), &Py::Int(0)))) } }) {
    return py_tuple(vec![]);
    }
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_ANY_ANSWER_WANTED(), &geti(&norm.clone(), &Py::Int(1)))) } }) {
    return py_tuple(vec![]);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(2), call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    at = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_ANY_ANSWER_LINKS(), &geti(&norm.clone(), &at.clone())))) {
    return call_builtin("tuple", &[slice_or_raise(&norm.clone(), &add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return py_tuple(vec![]);
    Py::None
}
pub fn _asks_after_any(args: &[Py]) -> Py {
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
    let mut about: Py = Py::None;
    let _ = py_str("True for \"any word on the job\" and its cousins, which ask without a question mark.\n\n    It has to name the thing after the link. \"any luck\" and \"any ideas\" on their own are\n    already read as well as they can be, and a bare frame says nothing new about the line.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&norm.clone())); if py_truthy(&__b) { __b } else { { let __b = lt(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ANY_ALREADY_ASKS(), &geti(&norm.clone(), &Py::Int(1)))) } } } }) {
    return Py::Bool(false);
    }
    about = after_any_link(&[norm.clone()]);
    return { let __b = call_builtin("bool", &[about.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_a_person(&[about.clone()]))) } };
    Py::None
}
pub fn _names_a_person(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let _ = py_str("True when the tail of an \"any word about X\" names a person rather than a thing.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e))) { let mut at: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_OWNS_A_PERSON(), &geti(&words.clone(), &at.clone()))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_PERSON_NAMED(), &geti(&words.clone(), &add(&at.clone(), &Py::Int(1))))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _ask_words(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("The words of a line that may be a question turned back, fillers and \"bout\" removed.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&g_BOUT(), "get", &[word.clone(), word.clone()])); } py_list(__out) })() };
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_FILLER_BEFORE(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    let _ = mcall(&words.clone(), "pop", &[Py::Int(0)]);
    }
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_FILLER_AFTER(), &geti(&words.clone(), &neg(&Py::Int(1))))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    return words.clone();
    Py::None
}
pub fn _asks_it_back(args: &[Py]) -> Py {
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
    let _ = py_str("True when the whole line is the question turned back on the bot.");
    return Py::Bool(py_contains(&g_ASKS_IT_BACK(), &mcall(&py_str(" "), "join", &[_ask_words(&[norm.clone()])])));
    Py::None
}
pub fn asks_how_i_am(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the whole line asks the bot how it is, however the greeting is spelled.\n\n    This is the greeting as a shape, so it does not depend on which spelling of it a phrase\n    list happens to hold: \"how you been\", \"hows everything\" and \"what u saying\" are the same\n    question as \"how are you\", and the caller that answers one answers them all. A state word\n    followed by \"with\", \"about\", \"to\" and the rest asks about something else (\"you ok with\n    that\", \"you alright to drive\"), so the short shape stands down for it.\n    ");
    while py_truthy(&{ let __b = gt(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&norm.clone(), &neg(&Py::Int(2))), &py_str("with"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_GREETING_ASK_SECOND_PERSON(), &geti(&norm.clone(), &neg(&Py::Int(1))))) } } } }) {
    norm = slice_or_raise(&norm.clone(), &Py::None, &neg(&Py::Int(2)), &Py::None);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&norm.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)) } }) {
    return Py::Bool(false);
    }
    words = call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&words.clone(), &g_GREETING_ASK_SUBJECT())))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&geti(&norm.clone(), &Py::Int(0)), &py_str("what")))) {
    return le(&words.clone(), &g_GREETING_WHAT_SAYING());
    }
    if py_truthy(&Py::Bool(!py_eq(&geti(&norm.clone(), &Py::Int(0)), &py_str("how")))) {
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&bitand(&words.clone(), &g_GREETING_ASK_STATE()))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&words.clone(), &g_GREETING_ASK_SECOND_PERSON()))) } } } }) {
    return Py::Bool(false);
    }
    at = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk5392 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk5392[0].clone(); let mut word: Py = __unpk5392[1].clone(); if py_truthy(&Py::Bool(py_contains(&g_GREETING_ASK_STATE(), &word.clone()))) {  __out.push(index.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&norm.clone(), &Py::None, &at.clone(), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_GREETING_ASK_SECOND_PERSON(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&{ let __b = gt(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &add(&at.clone(), &Py::Int(1))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_GREETING_ASK_ABOUT_SOMETHING(), &geti(&norm.clone(), &add(&at.clone(), &Py::Int(1))))) } }));
    }
    return le(&words.clone(), &g_GREETING_ASK_WORDS());
    Py::None
}
pub fn _tag_tells_a_plan(args: &[Py]) -> Py {
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
    let mut body: Py = Py::None;
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the body of the line before the tag is a plan or an event with a time.");
    body = slice_or_raise(&norm.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&body.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_TAG_PLAN(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[slice_or_raise(&body.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)]).unwrap_or_else(|e| exc::raise(e))) { let __unpk3798 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk3798[0].clone(); let mut word: Py = __unpk3798[1].clone(); __out.push({ let __b = Py::Bool(py_eq(&word.clone(), &py_str("at"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_TAG_TIME_NUMBER(), &geti(&body.clone(), &add(&index.clone(), &Py::Int(1))))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _tag_checks_a_fact(args: &[Py]) -> Py {
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
    let mut body: Py = Py::None;
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the body before the tag says something the bot could confirm.");
    body = slice_or_raise(&norm.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[slice_or_raise(&body.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_TAG_CHECKABLE(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&body.clone(), &add(&index.clone(), &Py::Int(1))), &py_str("be"))) } }) {
    return Py::Bool(false);
    }
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&body.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_TAG_CHECKABLE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asks_for_confirmation(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line ends by asking the other person to agree with it.\n\n    The tag is not a question word, so the line used to read as news and got \"Noted: ...\"\n    or \"I'm listening\" instead of an answer. Only the end of the line is read, and \"right\"\n    needs a word in front that can end a statement: \"im board right\" is a tag, while\n    \"im doing everything right\", \"ur right\" and \"i cant do anything right\" are not.\n    ");
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_TAG_ENDINGS(), &geti(&norm.clone(), &neg(&Py::Int(1))))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = _opens_question(&[norm.clone()]); if py_truthy(&__b) { __b } else { { let __b = _asks_the_bot(&[norm.clone()]); if py_truthy(&__b) { __b } else { { let __b = _tag_tells_a_plan(&[norm.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_tag_checks_a_fact(&[norm.clone()]))) } } } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&geti(&norm.clone(), &neg(&Py::Int(1))), &py_str("right")))) {
    if py_truthy(&mcall(&geti(&norm.clone(), &neg(&Py::Int(2))), "endswith", &[py_tuple(vec![py_str("ing"), py_str("ed")])])) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_contains(&g_TAG_RIGHT_NOT(), &geti(&norm.clone(), &neg(&Py::Int(2)))));
    }
    return ge(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4));
    Py::None
}
pub fn _asks_about_the_bot_day(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the whole line asks the bot about its own day.");
    words = _ask_words(&[norm.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    line = mcall(&py_str(" "), "join", &[words.clone()]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_BOT_DAY_SHAPES()) { let mut shape: Py = __item.clone(); __out.push(mcall(&shape.clone(), "fullmatch", &[line.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asks_how_it_spends_its_time(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("True for \"how do you spend your time\" and its neighbours, in the words they are typed.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&norm.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)) } }) {
    return Py::Bool(false);
    }
    words = call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&words.clone(), &py_set(vec![py_str("you"), py_str("u"), py_str("ya"), py_str("your"), py_str("ur")]))))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_SPENDS_ITS_TIME(), "search", &[mcall(&py_str(" "), "join", &[norm.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asks_whether_the_bot_is_about(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("True for \"you busy tonight\" and its neighbours: the bot, a time, and nothing else.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&norm.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } }) {
    return Py::Bool(false);
    }
    words = call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&words.clone(), &py_set(vec![py_str("you"), py_str("u"), py_str("ya"), py_str("ur"), py_str("your")]))))) {
    return Py::Bool(false);
    }
    if py_truthy(&bitand(&words.clone(), &g_FIRST_PERSON())) {
    return Py::Bool(false);
    }
    if py_truthy(&bitand(&words.clone(), &g_MANNERS_WORDS())) {
    return Py::Bool(false);
    }
    return { let __b = call_builtin("bool", &[bitand(&words.clone(), &g_ABOUT_AT_A_TIME())]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[bitand(&words.clone(), &g_FREE_OR_BUSY())]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn the_manners_question(args: &[Py]) -> Py {
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
    let mut lead: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the whole line asks how the bot is filling its time, however it is spelled.\n\n    \"what are you up to\", \"whatcha doing today\", \"watcha up to mate\". The same words put any\n    other way are a different ask: \"u up\" is whether anyone is there, and \"what are you doing\n    with that\" is about the thing in the person's hand, so the line has to open on `what` and\n    be made of nothing but the ask itself.\n    ");
    lead = Py::Int(0);
    while py_truthy(&{ let __b = lt(&lead.clone(), &call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::gen::understanding::normalize::g_UP_TO_LEADS(), &geti(&norm.clone(), &lead.clone()))) } }) {
    let __aug1 = add(&lead.clone(), &Py::Int(1));
    lead = __aug1;
    }
    norm = slice_or_raise(&norm.clone(), &lead.clone(), &Py::None, &Py::None);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&norm.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_MANNERS_OPENERS(), &geti(&norm.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_SAYS_WHAT_ARE_YOU(), &geti(&norm.clone(), &Py::Int(0)))) } } } }) {
    return Py::Bool(false);
    }
    words = call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bitand(&py_set(vec![py_str("you"), py_str("u"), py_str("ya")]), &words.clone()))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_SAYS_WHAT_ARE_YOU(), &geti(&norm.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&words.clone(), &py_str("been"))); if !py_truthy(&__b) { __b } else { bitand(&py_set(vec![py_str("do"), py_str("doing")]), &words.clone()) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&words.clone(), &py_str("busy"))); if !py_truthy(&__b) { __b } else { bitand(&py_set(vec![py_str("keeps"), py_str("keeping")]), &words.clone()) } }) {
    return le(&words.clone(), &g_KEEPS_BUSY_SHAPE());
    }
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&py_set(vec![py_str("up"), py_str("doing"), py_str("doin"), py_str("do"), py_str("at"), py_str("been")]), &words.clone())))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = bitand(&words.clone(), &g_WHEN_IN_THE_ASK()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&py_set(vec![py_str("up"), py_str("doing"), py_str("doin"), py_str("at"), py_str("been")]), &words.clone()))) } }) {
    return Py::Bool(false);
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_DOING_FILLER(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WHEN_IN_THE_ASK(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asks_what_is_new_with_the_bot(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("True for \"what's new with you\" and the same ask in the words people type it in.");
    words = call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(9)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&words.clone(), &py_set(vec![py_str("you"), py_str("u"), py_str("ya"), py_str("your"), py_str("yours")]))))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&words.clone(), &g_NEW_FRAME())))) {
    return Py::Bool(false);
    }
    if py_truthy(&bitand(&words.clone(), &g_NEW_ASK_AHEAD())) {
    return Py::Bool(false);
    }
    return le(&words.clone(), &g_NEW_ASK_WORDS());
    Py::None
}
pub fn _asks_what_the_bot_is_doing(args: &[Py]) -> Py {
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
    let _ = py_str("True for a line whose whole content is asking the bot how it is filling its time.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&norm.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(10)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = bitand(&py_set(vec![py_str("doing"), py_str("do")]), &call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("to"), py_str("much"), py_str("on")]), &geti(&norm.clone(), &neg(&Py::Int(1))))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&norm.clone(), &py_str("up"))) } } } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = le(&py_set(vec![py_str("going"), py_str("on")]), &call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { bitand(&call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_GOING_ON_WHEN()) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_DOING_FILLER(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return _asks_what_the_bot_has_been_doing(&[norm.clone()]);
    Py::None
}
pub fn _asks_what_the_bot_has_been_doing(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("True for the same ask with the time named, as long as that time has already gone.\n\n    \"what did you do yesterday\", \"what were you up to at the weekend\", \"where were you last\n    night\": the bot has no yesterday and no Saturday, so the honest answer is that it has\n    nothing of its own to report. Read as ordinary questions these went to the search hedge, to\n    the weekend-plan advice, or to \"we're still at the beginning\" as if the person had said what\n    they did. A day still to come is not this ask (`STILL_COMING`), and neither is a line that\n    puts the past auxiliary to work on a thing: \"what did you do with the leftover paint\".\n    ");
    words = call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&words.clone(), &py_set(vec![py_str("you"), py_str("u"), py_str("ya"), py_str("your"), py_str("yours")]))))) {
    return Py::Bool(false);
    }
    if py_truthy(&bitand(&words.clone(), &g_STILL_COMING())) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&words.clone(), &g_BEEN_SAID())))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bitand(&words.clone(), &py_set(vec![py_str("did"), py_str("done"), py_str("been"), py_str("were"), py_str("was"), py_str("had")])))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&words.clone(), &g_BEEN_WHEN()))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&words.clone(), &py_str("where"))); if !py_truthy(&__b) { __b } else { { let __b = bitand(&words.clone(), &g_SPENT_TIME()); if !py_truthy(&__b) { __b } else { bitand(&words.clone(), &py_set(vec![py_str("have"), py_str("has"), py_str("been"), py_str("were"), py_str("was")])) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&words.clone(), &py_set(vec![py_str("do"), py_str("does"), py_str("did"), py_str("done"), py_str("doing")]))))) {
    return Py::Bool(false);
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_DOING_FILLER(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_BEEN_WHEN(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BEEN_SAID(), &word.clone())) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _wants_a_recommendation(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the words ask the bot to pick something out, rather than describe the speaker.");
    if py_truthy(&{ let __b = bitand(&call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_COMPARATIVE()); if !py_truthy(&__b) { __b } else { mcall(&g_COMPARISON(), "search", &[add(&add(&py_str(" "), &text.clone()), &py_str(" "))]) } }) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g_COMPARATIVE(), &word.clone()))) {
    continue;
    }
    before = if py_truthy(&index.clone()) { geti(&norm.clone(), &sub(&index.clone(), &Py::Int(1))) } else { py_str("") };
    after = if py_truthy(&lt(&add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&norm.clone(), &add(&index.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_A_PERSON_AFTER(), &after.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("man"), py_str("woman")]), &after.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&norm.clone(), &add(&index.clone(), &Py::Int(2)), &add(&index.clone(), &Py::Int(3)), &Py::None), &py_list(vec![py_str("for")]))) } })) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_DESCRIBES(), &before.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_AFTER_COMPARATIVE(), &after.clone())) } }) {
    return Py::Bool(true);
    }
    }
    return call_builtin("bool", &[bitand(&call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &sub(&g_RECOMMEND(), &g_COMPARATIVE()))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_thing_a_thank_you_is_for(args: &[Py]) -> Py {
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
    let mut end: Py = Py::None;
    let mut run: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The (start, end) of the words a thank you's \"for\" takes, or None when it is not one.\n\n    \"thanks for the advice\", \"thanks a lot for the pointers\", \"thank you for the ideas\": the\n    words before the \"for\" are the courtesy itself and words that only measure it.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&norm.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_starts_with(&[norm.clone(), g_THANKS()]))) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[slice_or_raise(&norm.clone(), &Py::None, &Py::Int(5), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_eq(&word.clone(), &py_str("for")))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&norm.clone(), &Py::None, &at.clone(), &Py::None)) { let mut w: Py = __item.clone(); __out.push({ let __b = Py::Bool(!py_contains(&g_COURTESY_WORD(), &w.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_GRATITUDE_OBJECT(), &w.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    end = add(&at.clone(), &Py::Int(1));
    while py_truthy(&{ let __b = lt(&end.clone(), &call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&bitor(&bitor(&g_GRATITUDE_OBJECT(), &g_RECOMMEND()), &g_TIP_WORDS()), &geti(&norm.clone(), &end.clone()))) } }) {
    if py_truthy(&{ let __b = gt(&end.clone(), &add(&at.clone(), &Py::Int(1))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_GRATITUDE_ASK_AGAIN(), &geti(&norm.clone(), &end.clone()))) } }) {
    break;
    }
    let __aug3 = add(&end.clone(), &Py::Int(1));
    end = __aug3;
    }
    if py_truthy(&gt(&end.clone(), &add(&at.clone(), &Py::Int(1)))) {
    while py_truthy(&{ let __b = lt(&add(&end.clone(), &Py::Int(1)), &call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_GRATITUDE_JOINERS(), &geti(&norm.clone(), &end.clone()))) } }) {
    run = add(&end.clone(), &Py::Int(1));
    while py_truthy(&{ let __b = lt(&run.clone(), &call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&bitor(&bitor(&g_GRATITUDE_OBJECT(), &g_RECOMMEND()), &g_TIP_WORDS()), &geti(&norm.clone(), &run.clone()))) } }) {
    let __aug4 = add(&run.clone(), &Py::Int(1));
    run = __aug4;
    }
    if py_truthy(&Py::Bool(py_eq(&run.clone(), &add(&end.clone(), &Py::Int(1))))) {
    break;
    }
    if py_truthy(&{ let __b = lt(&run.clone(), &call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&norm.clone(), &run.clone()), &py_str("for"))) } }) {
    break;
    }
    end = run.clone();
    }
    return py_tuple(vec![at.clone(), end.clone()]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _asks(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut left: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("What the line asks for, as a set of short labels.");
    thing = _the_thing_a_thank_you_is_for(&[norm.clone()]);
    if py_truthy(&thing.clone()) {
    left = add(&slice_or_raise(&norm.clone(), &Py::None, &geti(&thing.clone(), &Py::Int(0)), &Py::None), &slice_or_raise(&norm.clone(), &geti(&thing.clone(), &Py::Int(1)), &Py::None, &Py::None));
    return _ask_labels(&[left.clone(), add(&add(&py_str(" "), &mcall(&py_str(" "), "join", &[left.clone()])), &py_str(" "))]);
    }
    return _ask_labels(&[norm.clone(), text.clone()]);
    Py::None
}
pub fn _a_bare_definition(args: &[Py]) -> Py {
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
    let mut head: Py = Py::None;
    let _ = py_str("True for a bare \"what is X\" that asks what the thing is, and False for the rest.\n\n    The head word decides most of it (see NOT_A_DEFINITION_HEAD), and a possessive head - \"what\n    is peru's capital\" - asks about somebody's thing rather than the meaning of a word.\n    ");
    found = mcall(&g_BARE_DEFINITION(), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    head = mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    if py_truthy(&{ let __b = mcall(&head.clone(), "endswith", &[py_tuple(vec![py_str("'s"), py_str("’s")])]); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\d"), head.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&stdlib_call("re", "match", &[py_str("\\s*s\\b"), slice_or_raise(&text.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_contains(&g_NOT_A_DEFINITION_HEAD(), &head.clone()));
    Py::None
}
pub fn _ask_labels(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut asks: Py = Py::None;
    let mut digits: Py = Py::None;
    let mut label: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The asks the line itself makes, gratitude already taken off.");
    asks = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    words = call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = _asks_the_room(&[norm.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ASK_FOR_A_NAME(), &geti(&norm.clone(), &Py::Int(1)))) } }) {
    let _ = mcall(&asks.clone(), "add", &[py_str("recommend")]);
    }
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &py_str("how")))) {
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &py_str("long")))) {
    let _ = mcall(&asks.clone(), "add", &[py_str("how_long")]);
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&words.clone(), &py_str("much"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &py_str("many"))) } }) {
    let _ = mcall(&asks.clone(), "add", &[py_str("how_much")]);
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&words.clone(), &py_str("often"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &py_str("frequently"))) } }) {
    let _ = mcall(&asks.clone(), "add", &[py_str("how_often")]);
    } else {
    let _ = mcall(&asks.clone(), "add", &[py_str("how")]);
    }
    }
    }
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("what"), py_str("why"), py_str("when"), py_str("where"), py_str("who"), py_str("which")]));
    for __item in __iter1 {
    label = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &label.clone()))) {
    let _ = mcall(&asks.clone(), "add", &[label.clone()]);
    }
    }
    if py_truthy(&{ let __b = _wants_a_recommendation(&[norm.clone(), text.clone()]); if py_truthy(&__b) { __b } else { { let __b = _has_any(&[text.clone(), g_RECOMMEND_PHRASES()]); if py_truthy(&__b) { __b } else { _asks_for_tips(&[text.clone(), norm.clone()]) } } } }) {
    let _ = mcall(&asks.clone(), "add", &[py_str("recommend")]);
    }
    if py_truthy(&{ let __b = _has_any(&[text.clone(), g_STEPS()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_STEPS_WORD(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { _asks_for_help(&[text.clone()]) } } } }) {
    let _ = mcall(&asks.clone(), "add", &[py_str("steps")]);
    }
    if py_truthy(&_has_any(&[text.clone(), g_OPINION()])) {
    let _ = mcall(&asks.clone(), "add", &[py_str("opinion")]);
    }
    if py_truthy(&{ let __b = _has_any(&[text.clone(), g_COMPARE()]); if py_truthy(&__b) { __b } else { bitand(&words.clone(), &py_set(vec![py_str("vs"), py_str("versus")])) } }) {
    let _ = mcall(&asks.clone(), "add", &[py_str("compare")]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_ASKS_ABOUT_A_PERSON(), "search", &[text.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = _has_any(&[text.clone(), g_DEFINITION()]); if py_truthy(&__b) { __b } else { _a_bare_definition(&[text.clone()]) } } } }) {
    let _ = mcall(&asks.clone(), "add", &[py_str("definition")]);
    }
    digits = call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\d"), mcall(&g_A_DATE_FIGURE(), "sub", &[py_str(" "), text.clone()])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = bitand(&words.clone(), &g_MATH_WORDS()); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("%"))); if py_truthy(&__b) { __b } else { { let __b = digits.clone(); if !py_truthy(&__b) { __b } else { bitand(&py_set(vec![py_str("of"), py_str("each"), py_str("per"), py_str("split")]), &words.clone()) } } } } } }) {
    let _ = mcall(&asks.clone(), "add", &[py_str("maths")]);
    }
    if py_truthy(&{ let __b = _has_any(&[text.clone(), py_tuple(vec![py_str(" or ")])]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&two_ends_of_one_scale(&[text.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_turns_the_pair_down(&[text.clone()]))) } } } }) {
    let _ = mcall(&asks.clone(), "add", &[py_str("choice")]);
    }
    if py_truthy(&bitand(&words.clone(), &py_set(vec![py_str("list"), py_str("ideas"), py_str("options"), py_str("examples"), py_str("suggest")]))) {
    let _ = mcall(&asks.clone(), "add", &[py_str("list")]);
    }
    if py_truthy(&bitand(&words.clone(), &py_set(vec![py_str("write"), py_str("word"), py_str("rewrite"), py_str("draft")]))) {
    let _ = mcall(&asks.clone(), "add", &[py_str("write")]);
    }
    if py_truthy(&_asks_a_question_coming(&[norm.clone(), text.clone()])) {
    let _ = mcall(&asks.clone(), "add", &[py_str("question_coming")]);
    }
    return call_builtin("sorted", &[asks.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _third_person(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut stem: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "stem") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: stem"))) };
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&stem.clone(), "endswith", &[py_str("y")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("aeiouy"), &geti(&stem.clone(), &neg(&Py::Int(2))))) } } } }) {
    return add(&slice_or_raise(&stem.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None), &py_str("ies"));
    }
    if py_truthy(&mcall(&stem.clone(), "endswith", &[py_tuple(vec![py_str("s"), py_str("sh"), py_str("ch"), py_str("x"), py_str("z"), py_str("o")])])) {
    return add(&stem.clone(), &py_str("es"));
    }
    return add(&stem.clone(), &py_str("s"));
    Py::None
}
pub fn _verb_form(args: &[Py]) -> Py {
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
    let mut auxiliary: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "auxiliary") { v.clone() }
    else { py_str("") };
    let _ = py_str("True when this word is a verb here rather than a thing the question is about.");
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("ing")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_PAST_AUX(), &auxiliary.clone())) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(py_contains(&g_VERB_FORMS(), &word.clone()));
    Py::None
}
pub fn noun_phrase_request(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True for a verbless line that names a thing and a place: a request for options there.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&line.clone(), &py_str("norm"))) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_NOUN_REQUEST_MAX()); if py_truthy(&__b) { __b } else { geti(&geti(&line.clone(), &py_str("signals")), &py_str("question_mark")) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NOUN_REQUEST_SELF(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk8532 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk8532[0].clone(); let mut word: Py = __unpk8532[1].clone(); __out.push({ let __b = Py::Bool(py_contains(&g_VERBISH(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&slice_or_raise(&words.clone(), &sub(&index.clone(), &Py::Int(1)), &index.clone(), &Py::None), &py_list(vec![py_str("to")]))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return { let __b = ge(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_PLACE_WORD(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn taking_up(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut match_: Py = Py::None;
    let mut text: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The activity the person is into, else None: `{\"text\", \"words\", \"shape\"}`.\n\n    `text` is the activity as typed (\"im really into baking at the moment\" -> \"baking\"), `words`\n    the same split up for matching, and `shape` is \"started\" for somebody beginning it (\"ive got\n    into yoga\", \"im getting into gardening\") and \"doing\" for somebody already at it (\"im into\n    running\").\n    ");
    text = call_builtin("str", &[{ let __b = mcall(&{ let __b = read.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&line.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    text = mcall(&stdlib_call("re", "sub", &[py_str("['’]"), py_str(""), text.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&modulo(&py_str(" %s "), &mcall(&text.clone(), "lower", &[])), &py_str(" not "))) } }) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "match", &[py_str("\\W*(?:am|is|are|was|were)\\s+(?:i|we|you|they|he|she)\\b"), text.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    match_ = mcall(&g_INTEREST_HEAD(), "match", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    thing = mcall(&mcall(&g_INTEREST_TAIL(), "sub", &[py_str(""), mcall(&mcall(&match_.clone(), "group", &[py_str("thing")]), "strip", &[py_str(" .,!?")])]), "strip", &[]);
    words = mcall(&thing.clone(), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_INTEREST_NOT(), &geti(&words.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_INTEREST_NOT(), &geti(&words.clone(), &Py::Int(1)))) } } } }) {
    return Py::None;
    }
    thing = mcall(&thing.clone(), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    return py_dict(vec![(py_str("text"), thing.clone()), (py_str("words"), words.clone()), (py_str("shape"), if py_truthy(&mcall(&g_STARTED_OUT(), "search", &[text.clone()])) { py_str("started") } else { py_str("doing") }), (py_str("back"), call_builtin("bool", &[mcall(&g_BACK_INTO(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e)))]);
    Py::None
}
pub fn _bare_subject(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut first: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a clause only names the thing: \"my cat\", \"the boiler\", \"milo\".\n\n    A clause with a verb in it says something about the thing, so a question after it is the\n    person's own question (\"my back hurts, can i take ibuprofen\").\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&clause.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&clause.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_VERBISH(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_tuple(vec![py_str("ed"), py_str("ing")])]) } } } }) {
    return Py::Bool(false);
    }
    }
    first = geti(&clause.clone(), &Py::Int(0));
    if py_truthy(&Py::Bool(py_contains(&g_DETERMINERS(), &first.clone()))) {
    return Py::Bool(true);
    }
    return { let __b = Py::Bool(py_eq(&call_builtin("len", &[clause.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_A_SUBJECT(), &first.clone())) } };
    Py::None
}
pub fn _continues_the_sentence(args: &[Py]) -> Py {
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
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut clause: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when this clause only finishes the sentence the clause before it started.");
    if py_truthy(&{ let __b = lt(&index.clone(), &Py::Int(1)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_bare_subject(&[geti(&clauses.clone(), &sub(&index.clone(), &Py::Int(1)))]))) } }) {
    return Py::Bool(false);
    }
    clause = geti(&clauses.clone(), &index.clone());
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&clause.clone(), &Py::Int(1), &Py::Int(3), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_SUBJECT_OF_ITS_OWN(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g_WH_CONTINUES(), &geti(&clause.clone(), &Py::Int(0))))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_contains(&g_YESNO_STARTS(), &geti(&clause.clone(), &Py::Int(0))))) {
    return Py::Bool(false);
    }
    return { let __b = ge(&index.clone(), &Py::Int(2)); if !py_truthy(&__b) { __b } else { _bare_subject(&[geti(&clauses.clone(), &sub(&index.clone(), &Py::Int(2)))]) } };
    Py::None
}
pub fn denied(args: &[Py]) -> Py {
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
    let mut cue: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cue") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cue"))) };
    let mut match_: Py = Py::None;
    let _ = py_str("True when this feeling word is the thing the line rules out, not the state it is in.");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[cue.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&mcall(&g_DENY_BEFORE(), "search", &[slice_or_raise(&text.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None)])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_state_of_the_speaker(args: &[Py]) -> Py {
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
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut back: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the word at `index` is said about how the speaker is: \"im ok\", \"im not ok\",\n    \"we are doing ok\". The words in between are the verb, a negator and the amount.");
    verb = Py::Bool(false);
    back = sub(&index.clone(), &Py::Int(1));
    while py_truthy(&{ let __b = ge(&back.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { le(&sub(&index.clone(), &back.clone()), &Py::Int(4)) } }) {
    word = geti(&norm.clone(), &back.clone());
    if py_truthy(&Py::Bool(py_contains(&g_SPEAKER(), &word.clone()))) {
    return verb.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&g_STATING_VERB(), &word.clone()))) {
    verb = Py::Bool(true);
    } else {
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_NEGATORS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NEGATION_SKIP(), &word.clone())) } }) {
    return Py::Bool(false);
    }
    }
    let __aug1 = sub(&back.clone(), &Py::Int(1));
    back = __aug1;
    }
    return Py::Bool(false);
    Py::None
}
pub fn _asks_for_it_rather_than_got_it(args: &[Py]) -> Py {
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
    let mut arrived: Py = Py::None;
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the asking or sending word comes before the letter that arrived.\n\n    \"i asked for a letter from the bank\" is the speaker asking for one, so advice on how to\n    answer it is the wrong direction. But a letter that arrived can ask for something itself -\n    \"i got a letter from the council asking me to call\" is still a letter to them, and the word\n    that names the asking sits after the word that says it arrived. So the veto only reaches as\n    far as the received word, the same way `_a_letter_someone_else_got` places the possessive.\n    ");
    arrived = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk4701 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk4701[0].clone(); let mut word: Py = __unpk4701[1].clone(); if py_truthy(&Py::Bool(py_contains(&g_LETTER_RECEIVED(), &word.clone()))) {  __out.push(index.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_is(&arrived.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&norm.clone(), &Py::None, &add(&arrived.clone(), &Py::Int(1)), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_LETTER_ASKED_FOR(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_letter_someone_else_got(args: &[Py]) -> Py {
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
    let mut verb: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "verb") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: verb"))) };
    let mut after: Py = Py::None;
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether a possessive in front of the letter names a person other than the speaker.\n\n    \"my neighbour got a letter from the council\" and \"my wife got a letter from the school\" are\n    letters to them, not to the person writing, so there is nothing for them to answer. The\n    possessive has to sit before the letter word to count, and \"i got my letter from the\n    council\" names the letter itself, so that one keeps its summons.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[slice_or_raise(&norm.clone(), &Py::None, &verb.clone(), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&g_LETTER_SOMEONE_ELSES(), &word.clone()))) {
    after = if py_truthy(&lt(&add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&norm.clone(), &add(&index.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_LETTER_ITSELF(), &after.clone())) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn summons_of(args: &[Py]) -> Py {
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
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut at: Py = Py::None;
    let mut gap: Py = Py::None;
    let mut index: Py = Py::None;
    let mut joined: Py = Py::None;
    let mut letter_first: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut rang: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("{\"who\", \"shape\"} when the line is someone wanting a word with the speaker, else None.\n\n    `who` is the office or person who can call them in, `shape` is what they asked for:\n    \"see\" (wants a word, a meeting), \"written\" (a letter) or \"rang\" (a phone call). A question\n    is left alone: \"what do i do if the council write to me\" is asking for advice, not news.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&norm.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&kind.clone(), &py_str("question"))) } }) {
    return Py::None;
    }
    words = call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    who = Py::None;
    at = Py::None;
    let __iter1 = py_iter_or_raise(&g_SUMMONS_WHO_PHRASES());
    for __item in __iter1 {
    phrase = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&mcall(&py_str(" "), "join", &[norm.clone()]), &phrase.clone()))) {
    who = phrase.clone();
    at = mcall(&norm.clone(), "index", &[geti(&mcall(&phrase.clone(), "split", &[]), &Py::Int(0))]);
    break;
    }
    }
    if py_truthy(&Py::Bool(py_is(&who.clone(), &Py::None))) {
    who = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_SUMMONS_WHO(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    at = if py_truthy(&who.clone()) { mcall(&norm.clone(), "index", &[who.clone()]) } else { Py::None };
    }
    if py_truthy(&Py::Bool(py_is(&who.clone(), &Py::None))) {
    return Py::None;
    }
    verb = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk8969 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk8969[0].clone(); let mut word: Py = __unpk8969[1].clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&g_SUMMONS_VERB(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("has"), py_str("have")]), &word.clone())) } }) {  __out.push(index.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_is(&verb.clone(), &Py::None))) {
    verb = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk314 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk314[0].clone(); let mut word: Py = __unpk314[1].clone(); if py_truthy(&Py::Bool(py_contains(&g_SUMMONS_VERB(), &word.clone()))) {  __out.push(index.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    }
    joined = add(&add(&py_str(" "), &mcall(&py_str(" "), "join", &[norm.clone()])), &py_str(" "));
    letter_first = { let __b = Py::Bool(!py_is(&verb.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = gt(&at.clone(), &verb.clone()); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SUMMONS_WRITTEN()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&joined.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_LETTER_RECEIVED(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_asks_for_it_rather_than_got_it(&[norm.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_letter_someone_else_got(&[norm.clone(), verb.clone()]))) } } } } } } } } } };
    if py_truthy(&{ let __b = Py::Bool(py_is(&verb.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = gt(&at.clone(), &verb.clone()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&letter_first.clone())) } } } }) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&norm.clone(), &call_builtin("max", &[Py::Int(0), sub(&verb.clone(), &Py::Int(3))]).unwrap_or_else(|e| exc::raise(e)), &verb.clone(), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_SUMMONS_NOT(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = letter_first.clone(); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SUMMONS_WRITTEN()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&joined.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_dict(vec![(py_str("who"), who.clone()), (py_str("shape"), py_str("written"))]);
    }
    rang = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SUMMONS_RANG()) { let mut phrase: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&joined.clone(), &phrase.clone()))) {  __out.push(phrase.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&rang.clone()) {
    gap = slice_or_raise(&mcall(&geti(&mcall(&joined.clone(), "split", &[rang.clone(), Py::Int(1)]), &Py::Int(1)), "split", &[]), &Py::None, &Py::Int(3), &Py::None);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bitand(&call_builtin("set", &[gap.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_SUMMONS_CALL_DONE()))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&call_builtin("set", &[gap.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_SUMMONS_SAID()))) } }) {
    return py_dict(vec![(py_str("who"), who.clone()), (py_str("shape"), py_str("rang"))]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&words.clone(), &call_builtin("set", &[g_SUMMONS_WANTS()]).unwrap_or_else(|e| exc::raise(e)))))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SUMMONS_SEE()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&joined.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_dict(vec![(py_str("who"), who.clone()), (py_str("shape"), py_str("see"))]);
    }
    return Py::None;
    Py::None
}
pub fn _said_to_somebody(args: &[Py]) -> Py {
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
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let _ = py_str("True when this word closes a short line as the person it is said to.\n\n    \"thanks love\", \"morning pet\", \"cheers duck\" and \"night night love\" end with a name for the\n    person, and the line above it is one of the ones people say to somebody: a thanks, a\n    greeting, a goodbye, a yes or a no. Anything else keeps its feeling word (\"i love it\",\n    \"theres no love between them\", \"i love\").\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&index.clone(), &sub(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&if py_truthy(&le(&Py::Int(2), &{ let __cmp10 = call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)); __cmp10.clone() })) { le(&__cmp10.clone(), &Py::Int(3)) } else { Py::Bool(false) })) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(py_contains(&g_ADDRESSED_LINES(), &geti(&norm.clone(), &Py::Int(0))));
    Py::None
}
pub fn _a_body_not_a_mood(args: &[Py]) -> Py {
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
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut forward: Py = Py::None;
    let _ = py_str("True when the feeling word at `index` is naming what is wrong with the stomach.");
    forward = if py_truthy(&lt(&add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&norm.clone(), &add(&index.clone(), &Py::Int(1))) } else { Py::None };
    return Py::Bool(py_contains(&g_STOMACH_WORDS(), &forward.clone()));
    Py::None
}
pub fn _asks_what_was_meant(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let _ = py_str("True when the whole line asks what the bot's own last answer meant.\n\n    \"what do you mean\", \"what does that mean\", \"how do you mean it\" and the half a dozen ways\n    people type the same question (\"wot u mean\", \"wdym\", \"whatcha mean\", \"whaddya mean by\n    that\") name nothing of their own: they point at the line just given. The knowledge base read\n    them as a follow-up on the subject of the last answer and took the word \"mean\" for the thing\n    named, so after \"my tomato plants have blight\" the person got \"The same goes for the wot u\n    mean: cut off and bin the affected leaves\" - their own slip said back at them as the thing\n    being asked about, and the composer's own reply (\"Just this: Brown-black patches ...\") was\n    the right one all along.\n\n    The spellings are read in one place already, by the reply that answers this question\n    (`bot/memory/meant.py::MEAN`), so this borrows that rule rather than keeping a second list\n    that falls behind it. A thing named after \"mean\" is not this reading: \"what does the boiler\n    pressure mean\", \"what does it mean to be a good friend\" and \"what does mean reversion mean\"\n    are questions about a thing, and their card keeps the turn.\n    ");
    asked = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return Py::Bool(false);
    }
    let mut meant: Py = crate::gen::memory::meant::module_obj();
    return call_builtin("bool", &[mcall(&gattr(&meant.clone(), "MEAN"), "match", &[asked.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asks_back(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("True when the line ends by handing the question back to the bot.\n\n    \"not bad, and you?\", \"im fine thanks, yourself\", \"good thanks, u?\" answer how the person is\n    and ask the bot the same thing in one breath. The reading called the whole line news about\n    them, so the reply said neither how the bot was nor took the question: \"im fine thanks,\n    yourself\" was answered \"Thanks for telling me - I'm here if you want to dig into any of\n    that\", which answers neither half.\n\n    Only the end of the line counts, and the \"you\" has to be a question rather than the object\n    of the line's own verb: \"i love you\", \"is it ok with you\", \"i miss talking to you\",\n    \"thanks, youre alright you\" and \"i saw your mum and you\" are none of them questions back.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_BACK_AT(), &geti(&words.clone(), &neg(&Py::Int(1))))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let __b = Py::Bool(py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BACK_JOIN(), &geti(&words.clone(), &Py::Int(0)))) } }; if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_BACK_ASKING(), &geti(&words.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BACK_JOIN(), &geti(&words.clone(), &Py::Int(1)))) } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BACK_NOT_A_SUBJECT(), &geti(&words.clone(), &neg(&Py::Int(3))))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_BACK_AT(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_set(vec![py_str("youre"), py_str("your")]), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&words.clone(), &neg(&Py::Int(2))), &py_str("about"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::None, &neg(&Py::Int(2)), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_BACK_ASKING(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g_BACK_JOIN(), &geti(&words.clone(), &neg(&Py::Int(2)))))) {
    return { let __b = Py::Bool(!py_eq(&_polarity(&[norm.clone()]), &Py::Int(0))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&emotion_of(&[mcall(&py_str(" "), "join", &[words.clone()])]), &Py::None)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::None, &neg(&Py::Int(2)), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_BACK_AFTER(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } };
    }
    return Py::Bool(py_contains(&g_BACK_AFTER(), &geti(&words.clone(), &neg(&Py::Int(2)))));
    Py::None
}
pub fn says_how_they_are(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line says how the speaker is with a word that is also another thing.\n\n    \"im grand\", \"im sound thanks\", \"im chill\": the reading has to see a feeling there, or the\n    line reads as news and the bot answers a bare \"im chill\" with \"Say more if it helps\".\n    \"the sound of the boiler\" and \"the content of the email\" are the other thing, so the word\n    only counts when it is said about the person.\n    ");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk1315 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk1315[0].clone(); let mut word: Py = __unpk1315[1].clone(); __out.push({ let __b = Py::Bool(py_contains(&g_SAYS_HOW_THEY_ARE(), &word.clone())); if !py_truthy(&__b) { __b } else { _a_state_of_the_speaker(&[norm.clone(), index.clone()]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_bad_verdict(args: &[Py]) -> Py {
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
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut back: Py = Py::None;
    let mut subject: Py = Py::None;
    let _ = py_str("True when the word at `index` is a verdict on a thing named before it: \"my job is a joke\".");
    if py_truthy(&{ let __b = ge(&index.clone(), &call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_VERDICT_NOUNS(), &geti(&norm.clone(), &index.clone()))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_VERDICT_ADJECTIVES(), &geti(&norm.clone(), &index.clone()))) } } } }) {
    return Py::Bool(false);
    }
    back = sub(&index.clone(), &Py::Int(1));
    while py_truthy(&{ let __b = ge(&back.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_VERDICT_FILLER(), &geti(&norm.clone(), &back.clone()))) } }) {
    let __aug1 = sub(&back.clone(), &Py::Int(1));
    back = __aug1;
    }
    if py_truthy(&{ let __b = lt(&back.clone(), &Py::Int(0)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_COPULAS(), &geti(&norm.clone(), &back.clone()))) } }) {
    return Py::Bool(false);
    }
    subject = sub(&back.clone(), &Py::Int(1));
    if py_truthy(&{ let __b = lt(&subject.clone(), &Py::Int(0)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PRONOUN_SUBJECT(), &geti(&norm.clone(), &subject.clone()))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_ASKING_SUBJECT(), &geti(&norm.clone(), &subject.clone()))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_COPULAS(), &geti(&norm.clone(), &subject.clone()))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING(), &geti(&norm.clone(), &subject.clone()))) } } } } } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn verdicts_a_thing(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut norm: Py = Py::None;
    let mut part: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line calls something a joke, a nightmare, a shambles.\n\n    Read by the module that hands out jokes, and by the composer: \"my landlord is a joke\" has the\n    word in it and is a person with a problem, and a punchline under it is the bot not reading the\n    line at all. The sentence is tidied the way the reading tidies it, so \"the trains are a total\n    joke\" counts.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "replace", &[py_str("'"), py_str("")]), "split", &[])]);
    norm = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&text.clone(), "split", &[])) { let mut part: Py = __item.clone(); __out.push(stdlib_call("re", "sub", &[py_str("[^a-z0-9]+"), py_str(""), part.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut index: Py = __item.clone(); __out.push(a_bad_verdict(&[norm.clone(), index.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _polarity(args: &[Py]) -> Py {
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
    let mut feeling: Py = Py::None;
    let mut index: Py = Py::None;
    let mut negated: Py = Py::None;
    let mut score: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("-1, 0 or 1 for the feeling the line carries, with a negation turning it the other way.\n\n    \"im not fine\", \"not feeling great\" and \"im not ok\" carry a bad feeling even though every\n    feeling word in them is a good one, and \"not bad\" carries a good one. A line with words\n    both ways (\"tired but happy\") stays 0.\n\n    A negator turns the next feeling word only, so \"i cant stop crying\" and \"i cant help\n    feeling sad\" stay bad: there the negated verb is not about the feeling. \"ok\" counts only\n    as somebody's state (\"im ok\"), because on its own it is an acknowledgement or a question\n    about permission.\n    ''worry'' is left out on purpose: \"should i worry about my knee\" is a question about\n    whether to worry, not a line with a bad feeling in it.\n    ");
    score = Py::Int(0);
    negated = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&g_NEGATORS(), &word.clone()))) {
    negated = Py::Bool(true);
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_TERMS_OF_ADDRESS(), &word.clone())); if !py_truthy(&__b) { __b } else { _said_to_somebody(&[norm.clone(), index.clone()]) } }) {
    negated = Py::Bool(false);
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_OK_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SAYS_HOW_THEY_ARE(), &word.clone())) } }) {
    feeling = if py_truthy(&_a_state_of_the_speaker(&[norm.clone(), index.clone()])) { Py::Int(1) } else { Py::None };
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_POSITIVE(), &word.clone()))) {
    feeling = Py::Int(1);
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_NEGATIVE(), &word.clone()))) {
    feeling = if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_BODY_STATE_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_body_not_a_mood(&[norm.clone(), index.clone()]))) } }) { neg(&Py::Int(1)) } else { Py::None };
    } else {
    if py_truthy(&a_bad_verdict(&[norm.clone(), index.clone()])) {
    feeling = neg(&Py::Int(1));
    } else {
    feeling = Py::None;
    }
    }
    }
    }
    if py_truthy(&Py::Bool(py_is(&feeling.clone(), &Py::None))) {
    if py_truthy(&Py::Bool(!py_contains(&g_NEGATION_SKIP(), &word.clone()))) {
    negated = Py::Bool(false);
    }
    continue;
    }
    let __aug3 = add(&score.clone(), &if py_truthy(&negated.clone()) { neg(&feeling.clone()) } else { feeling.clone() });
    score = __aug3;
    negated = Py::Bool(false);
    }
    return sub(&gt(&score.clone(), &Py::Int(0)), &lt(&score.clone(), &Py::Int(0)));
    Py::None
}
pub fn _thanks_still_asks(args: &[Py]) -> Py {
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
    let mut back: Py = Py::None;
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a thanks is followed by an ask that has no question word of its own.\n\n    A person who thanks the bot and then asks for advice on something often writes no question\n    word at all: \"thanks, any tips for a cold\". The advice word is the ask there, while in\n    \"thanks for the advice\" the same word is only what they are thanking the bot for.\n    ");
    words = call_builtin("list", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g_THANKS_ASK_WORDS(), &word.clone()))) {
    continue;
    }
    back = if py_truthy(&index.clone()) { geti(&words.clone(), &sub(&index.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&g_GIVEN_ADVICE(), &back.clone()))) {
    continue;
    }
    return Py::Bool(true);
    }
    let __iter3 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    index = __unpk4[0].clone();
    word = __unpk4[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g_THANKS_ASK_NOUNS(), &word.clone()))) {
    continue;
    }
    back = if py_truthy(&index.clone()) { geti(&words.clone(), &sub(&index.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&g_GIVEN_ADVICE(), &back.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = lt(&add(&index.clone(), &Py::Int(2)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_THANKS_ASK_PREPS(), &geti(&words.clone(), &add(&index.clone(), &Py::Int(1))))); if !py_truthy(&__b) { __b } else { mcall(&geti(&words.clone(), &add(&index.clone(), &Py::Int(2))), "isalpha", &[]) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _only_the_ask(args: &[Py]) -> Py {
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
    let mut asks: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asks") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: asks"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line is nothing but a frame asking after something the last turn named.\n\n    \"the best time to visit\" is not a statement about the best time: it is the question \"what\n    is the best time to visit?\" with the subject left out, which is how people follow up. The\n    line names nobody (\"i\"/\"we\" makes it a statement about what they want), says nothing about\n    a subject (\"is\"/\"was\"), and is a short frame - a thing word, a quality or an ask, and \"to\n    <verb>\". A frame that picks one thing out is asking for a recommendation even when the ask\n    words missed it: \"a good place to eat\" asks for one as plainly as \"the best way to eat\".\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_POLITE(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &bitor(&g_SPEAKER(), &g_SAYS_WHAT_IT_IS()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_contains(&g_OPENS_A_FRAME(), &geti(&words.clone(), &Py::Int(0))))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_PICKING()))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_TO_A_VERB(), "search", &[call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))]))) } }) {
    return Py::Bool(false);
    }
    return { let __b = call_builtin("bool", &[asks.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_PICKING_QUALITY())]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn classify(args: &[Py]) -> Py {
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
    let mut ask: Py = Py::None;
    let mut asks: Py = Py::None;
    let mut at: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut behind: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut courtesy: Py = Py::None;
    let mut denied_bad: Py = Py::None;
    let mut emotion: Py = Py::None;
    let mut index: Py = Py::None;
    let mut intent: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut limit: Py = Py::None;
    let mut norm: Py = Py::None;
    let mut opens_like_question: Py = Py::None;
    let mut polarity: Py = Py::None;
    let mut ruled_out: Py = Py::None;
    let mut short: Py = Py::None;
    let mut shortage: Py = Py::None;
    let mut signals: Py = Py::None;
    let mut state_owner: Py = Py::None;
    let mut suggestion: Py = Py::None;
    let mut tag: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turned_back: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    text = geti(&line.clone(), &py_str("norm_text"));
    norm = geti(&line.clone(), &py_str("norm"));
    words = call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    signals = geti(&line.clone(), &py_str("signals"));
    clauses = geti(&line.clone(), &py_str("clauses"));
    opens_like_question = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[clauses.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk9661 = py_unpack(&__item.clone(), 2, None); let mut at: Py = __unpk9661[0].clone(); let mut clause: Py = __unpk9661[1].clone(); __out.push({ let __b = _opens_question(&[clause.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_continues_the_sentence(&[clauses.clone(), at.clone()]))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    asks = _asks(&[norm.clone(), add(&add(&py_str(" "), &text.clone()), &py_str(" "))]);
    turned_back = { let __b = _asks_it_back(&[norm.clone()]); if py_truthy(&__b) { __b } else { _asks_about_the_bot_day(&[norm.clone()]) } };
    tag = _asks_for_confirmation(&[norm.clone()]);
    kind = py_str("statement");
    courtesy = without_filler(&[norm.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&norm.clone()))) {
    kind = py_str("empty");
    } else {
    if py_truthy(&{ let __b = geti(&signals.clone(), &py_str("question_mark")); if py_truthy(&__b) { __b } else { { let __b = _opens_question(&[norm.clone()]); if py_truthy(&__b) { __b } else { { let __b = opens_like_question.clone(); if py_truthy(&__b) { __b } else { { let __b = _asks_the_bot(&[norm.clone()]); if py_truthy(&__b) { __b } else { { let __b = _asks_for_help(&[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = tag.clone(); if py_truthy(&__b) { __b } else { { let __b = _asks_after_any(&[norm.clone()]); if py_truthy(&__b) { __b } else { { let __b = asks.clone(); if !py_truthy(&__b) { __b } else { { let __b = _starts_with(&[norm.clone(), g_THANKS()]); if !py_truthy(&__b) { __b } else { { let __b = geti(&signals.clone(), &py_str("question_mark")); if py_truthy(&__b) { __b } else { { let __b = bitand(&call_builtin("set", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_QUESTION_WORDS()); if py_truthy(&__b) { __b } else { _thanks_still_asks(&[norm.clone()]) } } } } } } } } } } } } } } } } } } } } } }) {
    kind = py_str("question");
    } else {
    if py_truthy(&_only_the_ask(&[norm.clone(), asks.clone(), text.clone()])) {
    kind = py_str("question");
    asks = call_builtin("sorted", &[bitor(&call_builtin("set", &[asks.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("recommend")]))]).unwrap_or_else(|e| exc::raise(e));
    } else {
    if py_truthy(&{ let __b = _starts_with(&[norm.clone(), g_GREETINGS()]); if !py_truthy(&__b) { __b } else { greeting_only(&[norm.clone()]) } }) {
    kind = py_str("greeting");
    } else {
    if py_truthy(&{ let __b = { let __b = _starts_with(&[courtesy.clone(), g_THANKS()]); if !py_truthy(&__b) { __b } else { _thanks_only(&[courtesy.clone(), clauses.clone()]) } }; if py_truthy(&__b) { __b } else { _says_the_help_was_good(&[norm.clone()]) } }) {
    kind = py_str("thanks");
    } else {
    if py_truthy(&_ends_with_thanks(&[courtesy.clone()])) {
    kind = py_str("thanks");
    }
    }
    }
    }
    }
    }
    if py_truthy(&turned_back.clone()) {
    asks = py_list(vec![]);
    }
    if py_truthy(&Py::Bool(!py_eq(&kind.clone(), &py_str("question")))) {
    behind = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&bitor(&crate::gen::understanding::normalize::background_clause(&[norm.clone()]), &crate::gen::understanding::normalize::background_positions(&[norm.clone()]))) { let mut index: Py = __item.clone(); __out.push(geti(&norm.clone(), &index.clone())); } py_set(__out) })() };
    asks = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asks.clone()) { let mut ask: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&behind.clone(), &ask.clone()))) {  __out.push(ask.clone()); } } py_list(__out) })() };
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("question"))); if !py_truthy(&__b) { __b } else { { let __b = { let __b = Py::Bool(!py_truthy(&turned_back.clone())); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&line.clone(), &py_str("clauses"))) { let mut clause: Py = __item.clone(); __out.push({ let __b = slice_or_raise(&crate::gen::understanding::normalize::strip_leaders(&[clause.clone()]), &Py::None, &Py::Int(1), &Py::None); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_YESNO_STARTS(), &geti(&crate::gen::understanding::normalize::strip_leaders(&[clause.clone()]), &Py::Int(0)))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }; if py_truthy(&__b) { __b } else { tag.clone() } } } }) {
    asks = call_builtin("sorted", &[bitor(&call_builtin("set", &[asks.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("yesno")]))]).unwrap_or_else(|e| exc::raise(e));
    }
    short = le(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3));
    bare = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_POLITE(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    denied_bad = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NEGATIVE(), &word.clone())); if !py_truthy(&__b) { __b } else { denied(&[text.clone(), word.clone()]) } }) {  __out.push(word.clone()); } } py_set(__out) })() };
    intent = py_str("statement");
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("empty")))) {
    intent = py_str("empty");
    } else {
    if py_truthy(&{ let __b = geti(&signals.clone(), &py_str("laughter")); if !py_truthy(&__b) { __b } else { le(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    intent = py_str("laugh");
    } else {
    if py_truthy(&{ let __b = _starts_with(&[crate::gen::understanding::normalize::strip_leaders(&[norm.clone()]), g_FAREWELLS()]); if !py_truthy(&__b) { __b } else { farewell_only(&[crate::gen::understanding::normalize::strip_leaders(&[norm.clone()])]) } }) {
    intent = py_str("farewell");
    } else {
    if py_truthy(&_about_bot(&[text.clone(), words.clone(), norm.clone()])) {
    intent = py_str("about_bot");
    } else {
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("greeting")))) {
    intent = py_str("greeting");
    } else {
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("thanks")))) {
    intent = py_str("thanks");
    } else {
    if py_truthy(&{ let __b = _has_any(&[add(&add(&py_str(" "), &text.clone()), &py_str(" ")), g_CORRECTION()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&words.clone(), &py_str("meant"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &py_str("mean"))) } } } }) {
    intent = py_str("correction");
    } else {
    if py_truthy(&{ let __b = _asks_it_back(&[norm.clone()]); if py_truthy(&__b) { __b } else { { let __b = _asks_about_the_bot_day(&[norm.clone()]); if py_truthy(&__b) { __b } else { { let __b = _asks_what_the_bot_is_doing(&[norm.clone()]); if py_truthy(&__b) { __b } else { { let __b = the_manners_question(&[norm.clone()]); if py_truthy(&__b) { __b } else { asks_how_i_am(&[norm.clone()]) } } } } } } } }) {
    intent = py_str("smalltalk");
    } else {
    if py_truthy(&agreement_of(&[text.clone()])) {
    intent = py_str("ack");
    } else {
    if py_truthy(&{ let __b = _short_reply(&[text.clone()]); if !py_truthy(&__b) { __b } else { le(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } }) {
    intent = py_str("ack");
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("statement"))); if !py_truthy(&__b) { __b } else { _agrees_to_try(&[norm.clone()]) } }) {
    intent = py_str("ack");
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_set(vec![py_str("no"), py_str("nah"), py_str("nope"), py_str("never")]), &geti(&norm.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { { let __b = le(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&slice_or_raise(&norm.clone(), &Py::Int(1), &Py::Int(2), &Py::None), &py_list(vec![py_str("one")]))) } } } }) {
    intent = py_str("deny");
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_ACK(), &geti(&norm.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { le(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    intent = py_str("ack");
    } else {
    if py_truthy(&{ let __b = le(&call_builtin("len", &[bare.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { le(&call_builtin("set", &[bare.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_AFFIRM()) } }) {
    intent = py_str("affirm");
    } else {
    if py_truthy(&{ let __b = le(&call_builtin("len", &[bare.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { le(&call_builtin("set", &[bare.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_DENY()) } }) {
    intent = py_str("deny");
    } else {
    if py_truthy(&{ let __b = le(&call_builtin("len", &[bare.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = le(&call_builtin("set", &[bare.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_ACK()); if py_truthy(&__b) { __b } else { le(&call_builtin("set", &[bare.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_LAUGH()) } } } }) {
    intent = py_str("ack");
    } else {
    if py_truthy(&small_talk_only(&[norm.clone()])) {
    intent = py_str("smalltalk");
    } else {
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("question")))) {
    intent = py_str("question");
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&asks.clone(), &py_str("recommend"))); if py_truthy(&__b) { __b } else { { let __b = _asks_for_tips(&[text.clone(), norm.clone()]); if py_truthy(&__b) { __b } else { { let __b = _has_any(&[text.clone(), g_STEPS()]); if py_truthy(&__b) { __b } else { bitand(&words.clone(), &py_set(vec![py_str("help"), py_str("need"), py_str("want")])) } } } } } }) {
    intent = py_str("request");
    } else {
    if py_truthy(&{ let __b = slice_or_raise(&norm.clone(), &Py::None, &Py::Int(1), &Py::None); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_set(vec![py_str("tell"), py_str("explain"), py_str("show"), py_str("give"), py_str("write"), py_str("make")]), &geti(&norm.clone(), &Py::Int(0)))) } }) {
    intent = py_str("request");
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&words.clone(), &py_str("i"))); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&denied_bad.clone(), &w.clone()))) {  __out.push({ let __b = Py::Bool(py_contains(&g_NEGATIVE(), &w.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_POSITIVE(), &w.clone())) } }); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { says_how_they_are(&[norm.clone()]) } } } }) {
    intent = py_str("feeling");
    } else {
    if py_truthy(&{ let __b = { let __b = Py::Bool(py_contains(&words.clone(), &py_str("i"))); if py_truthy(&__b) { __b } else { _acts_on_the_speaker(&[text.clone()]) } }; if !py_truthy(&__b) { __b } else { emotion_of(&[text.clone()]) } }) {
    intent = py_str("feeling");
    } else {
    if py_truthy(&_has_any(&[add(&add(&py_str(" "), &text.clone()), &py_str(" ")), g_NEWS()])) {
    intent = py_str("news");
    } else {
    if py_truthy(&{ let __b = bitand(&py_set(vec![py_str("i"), py_str("me"), py_str("my"), py_str("we"), py_str("our")]), &words.clone()); if !py_truthy(&__b) { __b } else { { let __b = bitand(&words.clone(), &g_EVENT_WORDS()); if py_truthy(&__b) { __b } else { le(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)) } } } }) {
    intent = py_str("news");
    } else {
    if py_truthy(&{ let __b = _has_any(&[text.clone(), g_OPINION()]); if py_truthy(&__b) { __b } else { { let __b = bitand(&words.clone(), &py_set(vec![py_str("think")])); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&kind.clone(), &py_str("question"))) } } } }) {
    intent = py_str("opinion");
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    state_owner = _state_owner(&[geti(&line.clone(), &py_str("clauses"))]);
    limit = _limit_of(&[text.clone()]);
    suggestion = _a_suggestion_from_someone(&[norm.clone(), text.clone()]);
    ruled_out = _ruled_out_of(&[text.clone()]);
    emotion = Py::None;
    if py_truthy(&{ let __b = Py::Bool(py_contains(&words.clone(), &py_str("i"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&words.clone(), &py_str("im"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&words.clone(), &py_str("my"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&kind.clone(), &py_str("question"))) } } } } } }) {
    emotion = emotion_of(&[text.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![Py::None, py_str("thing")]), &state_owner.clone())); if !py_truthy(&__b) { __b } else { _somebody_elses_heart(&[text.clone()]) } }) {
    state_owner = py_str("other");
    }
    if py_truthy(&{ let __b = emotion.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![Py::None, py_str("thing")]), &state_owner.clone())); if !py_truthy(&__b) { __b } else { _a_broken_heart_is_theirs(&[text.clone()]) } } } }) {
    state_owner = py_str("self");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&emotion.clone())); if !py_truthy(&__b) { __b } else { _a_hedged_low(&[{ let __b = mcall(&line.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { text.clone() } }]) } }) {
    emotion = py_str("sad");
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![Py::None, py_str("thing")]), &state_owner.clone()))) {
    state_owner = py_str("self");
    }
    }
    polarity = _polarity(&[norm.clone()]);
    shortage = shortage_of(&[text.clone()]);
    if py_truthy(&{ let __b = shortage.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&emotion.clone(), &Py::None)) } }) {
    state_owner = Py::None;
    }
    return py_dict(vec![(py_str("kind"), kind.clone()), (py_str("intent"), intent.clone()), (py_str("asks"), asks.clone()), (py_str("tag"), tag.clone()), (py_str("emotion"), emotion.clone()), (py_str("state_owner"), state_owner.clone()), (py_str("shortage"), shortage.clone()), (py_str("polarity"), polarity.clone()), (py_str("short"), short.clone()), (py_str("first_person"), call_builtin("bool", &[bitand(&words.clone(), &py_set(vec![py_str("i"), py_str("me"), py_str("my"), py_str("we"), py_str("our"), py_str("us"), py_str("myself")]))]).unwrap_or_else(|e| exc::raise(e))), (py_str("asks_opinion"), Py::Bool(py_contains(&asks.clone(), &py_str("opinion")))), (py_str("asks_recommendation"), Py::Bool(py_contains(&asks.clone(), &py_str("recommend")))), (py_str("asks_steps"), Py::Bool(py_contains(&asks.clone(), &py_str("steps")))), (py_str("help_ask"), _asks_for_help(&[text.clone()])), (py_str("asks_a_verdict"), _asks_a_verdict(&[text.clone()])), (py_str("about_bot"), Py::Bool(py_eq(&intent.clone(), &py_str("about_bot")))), (py_str("about_bot_kind"), if py_truthy(&Py::Bool(py_eq(&intent.clone(), &py_str("about_bot")))) { _about_bot_kind(&[text.clone(), words.clone(), norm.clone()]) } else { Py::None }), (py_str("counts_the_bots_own_day"), _counts_its_own_doings(&[norm.clone()])), (py_str("replies_to_bot"), Py::Bool(py_contains(&py_tuple(vec![py_str("affirm"), py_str("deny"), py_str("ack"), py_str("laugh")]), &intent.clone()))), (py_str("agrees_to_try"), _agrees_to_try(&[norm.clone()])), (py_str("wants_a_quiz"), _hands_a_question_to_the_speaker(&[norm.clone()])), (py_str("agreement"), agreement_of(&[text.clone()])), (py_str("summons"), summons_of(&[norm.clone(), kind.clone()])), (py_str("asks_back"), _asks_back(&[norm.clone()])), (py_str("asks_what_was_meant"), _asks_what_was_meant(&[text.clone()])), (py_str("excludes"), if py_truthy(&limit.clone()) { py_list(vec![limit.clone()]) } else { py_list(vec![]) }), (py_str("ruled_out"), ruled_out.clone()), (py_str("constraint"), Py::Bool(!py_is(&limit.clone(), &Py::None))), (py_str("suggestion"), suggestion.clone())]);
    Py::None
}
pub fn shift_worked(args: &[Py]) -> Py {
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
    let _ = py_str("The shift pattern the line names (\"late\", \"early\", \"night\"), or None.\n\n    \"im on lates this week\" and \"im on earlies\" are how half the working country says a week of\n    two-till-ten or six-till-two. They are the hours of the week, not a late period: the word\n    \"lates\" was read as \"late\" and reached the cycle card.\n    ");
    text = if py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { text.clone() } else { py_str("") };
    if py_truthy(&mcall(&g_SHIFT_NIGHT(), "search", &[text.clone()])) {
    return py_str("night");
    }
    if py_truthy(&mcall(&g_SHIFT_LATE(), "search", &[text.clone()])) {
    return py_str("late");
    }
    if py_truthy(&mcall(&g_SHIFT_EARLY(), "search", &[text.clone()])) {
    return py_str("early");
    }
    return Py::None;
    Py::None
}
pub fn _a_suggestion_from_someone(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut before: Py = Py::None;
    let mut found: Py = Py::None;
    let mut named: Py = Py::None;
    let mut report: Py = Py::None;
    let mut said: Py = Py::None;
    let mut who: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The advice somebody else gave, as {\"who\": \"Your mate\", \"source\": \"my mate\"}, or None.\n\n    Two parts, and both are needed: a report (\"said\", \"says\", \"told\", \"reckons\", \"wants me\n    to\", \"i was told to\") and a clause that is advice to the speaker (\"i should\", \"me to rest\").\n    A report of a fact is somebody else's news and keeps its own reading, and a suggestion with\n    no report in it (\"i should try running\") is the person's own plan, which the composer has.\n\n    The name comes from the line where the line gives one, because \"Your mate may be onto\n    something\" is a warmer reply than the same line with nobody in it. \"my\" becomes \"Your\":\n    the bot is not in the family.\n    ");
    said = add(&add(&py_str(" "), &mcall(&py_str(" "), "join", &[norm.clone()])), &py_str(" "));
    report = mcall(&g_A_REPORTED_VERB(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&report.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_SUGGESTED_CLAUSE(), "search", &[said.clone(), mcall(&report.clone(), "start", &[])]); if py_truthy(&__b) { __b } else { mcall(&g_A_TOLD_TO(), "search", &[said.clone()]) } }))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_NOT_A_SUGGESTION(), "search", &[said.clone()])) {
    return Py::None;
    }
    before = slice_or_raise(&said.clone(), &Py::None, &mcall(&report.clone(), "start", &[]), &Py::None);
    named = Py::None;
    let __iter1 = py_iter_or_raise(&mcall(&g_A_SAID_BY(), "finditer", &[before.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    named = mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "strip", &[]);
    }
    if py_truthy(&named.clone()) {
    words = mcall(&named.clone(), "split", &[]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("my"), py_str("our")]), &geti(&words.clone(), &Py::Int(0))))) {
    let _ = py_setitem(&words.clone(), &Py::Int(0), py_str("your"));
    }
    who = mcall(&py_str(" "), "join", &[words.clone()]);
    } else {
    if py_truthy(&mcall(&g_A_SAID_BY_ANYONE(), "search", &[before.clone()])) {
    who = py_str("");
    } else {
    who = py_str("");
    }
    }
    return py_dict(vec![(py_str("who"), add(&mcall(&slice_or_raise(&who.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&who.clone(), &Py::Int(1), &Py::None, &Py::None))), (py_str("source"), { let __b = named.clone(); if py_truthy(&__b) { __b } else { py_str("") } })]);
    Py::None
}
pub fn is_a_routine(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut text: Py = Py::None;
    let _ = py_str("True when the line reports a routine the speaker keeps, rather than asking about one.\n\n    \"i walk the dog every day\", \"we eat out twice a month\", \"i water the plants every morning\":\n    the person is the one doing it, the line asks nothing, and nothing in it is a fault or a\n    feeling. A card is about the thing a line names; on a routine the thing is what they do it\n    to, and the routine is the news, so a module that would answer the thing stands aside and\n    the habit tail says the routine back (bot/memory/ack.py::_plain).\n\n    False for a question (\"how often should i walk the dog\"), for a line whose subject is\n    something else (\"my dog barks at the postman every day\"), for a symptom (\"i get headaches\n    most days\") and for a feeling (\"i cry every morning\").\n    ");
    read = { let __b = read.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("statement"), py_str("news"), Py::None]), &mcall(&read.clone(), "get", &[py_str("kind")])))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&read.clone(), "get", &[py_str("asks")])) {
    return Py::Bool(false);
    }
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&line.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&line.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&line.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } } } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { mcall(&text.clone(), "endswith", &[py_str("?")]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__ROUTINE_SUBJECT(), "match", &[text.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_ROUTINE_IS_A_FAULT(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ROUTINE_IN_THE_PAST(), "search", &[text.clone()]) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_ROUTINE_SHAPE(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_home_carried_forward(args: &[Py]) -> Py {
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
    let mut earlier_lines: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "earlier_lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: earlier_lines"))) };
    let mut carried: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut line: Py = Py::None;
    let mut read: Py = Py::None;
    let _ = py_str("The care home a line with no home in it is about, read from the conversation, or None.\n\n    \"my mum moved into a home last month\" places somebody; \"what should i bring her\" two turns\n    later is about that same visit, and on its own the only word in it that means anything is\n    \"bring\" - so the answer was the customs allowance (\"Coming into the UK you can bring 4 litres\n    of spirits\"). The line names neither a person nor a thing, so the subject is only clear from\n    the turn before it: that is what this reads, and `carried` says where the reading came from.\n    ");
    line = mcall(&stdlib_call("re", "sub", &[py_str("['\\u2019]"), py_str(""), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { a_care_home(&[line.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_CARE_HOME_BRING_ASKS(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_CARE_HOME_BRING_WORD(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_CARE_HOME_BRING_WHO(), "search", &[line.clone()]) } } } }))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[{ let __b = earlier_lines.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    earlier = __item.clone();
    read = a_care_home(&[earlier.clone()]);
    if py_truthy(&read.clone()) {
    carried = call_builtin("dict", &[read.clone()]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&carried.clone(), &py_str("shape"), py_str("bring"));
    let _ = py_setitem(&carried.clone(), &py_str("carried"), call_builtin("str", &[earlier.clone()]).unwrap_or_else(|e| exc::raise(e)));
    return carried.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn a_care_home(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut lowered: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pronoun: Py = Py::None;
    let mut relation: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The person living in a care home a line is about, or None.\n\n    `shape` says what the line is about there: \"guilt\" when it names the feeling, \"visiting\" when\n    it is about going, \"moved\" when somebody has just gone in, and \"in\" for the plain statement.\n    ");
    let mut RELATIONS: Py = crate::gen::understanding::topics::g_RELATIONS();
    line = mcall(&stdlib_call("re", "sub", &[py_str("['\\u2019]"), py_str(""), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_CARE_HOME_PLACE(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_CARE_HOME_NOT(), "search", &[line.clone()]) } } } }) {
    return Py::None;
    }
    lowered = mcall(&line.clone(), "lower", &[]);
    let __unpk1 = py_unpack(&py_tuple(vec![py_str(""), Py::None]), 2, None);
    who = __unpk1[0].clone();
    relation = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&stdlib_call("re", "finditer", &[py_str("\\b(?:my|our|your)\\s+([a-z][a-z-]*)"), lowered.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    match_ = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&RELATIONS.clone(), &mcall(&match_.clone(), "group", &[Py::Int(1)]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CARE_HOME_KIN(), &mcall(&match_.clone(), "group", &[Py::Int(1)]))) } }) {
    relation = mcall(&match_.clone(), "group", &[Py::Int(1)]);
    who = mcall(&py_str(" "), "join", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "split", &[])]);
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    let __iter3 = py_iter_or_raise(&py_tuple(vec![py_str("shes"), py_str("hes"), py_str("she"), py_str("he"), py_str("they"), py_str("theyre")]));
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[modulo(&py_str("\\b%s\\b"), &word.clone()), lowered.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    who = word.clone();
    break;
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("husband"), py_str("father"), py_str("dad"), py_str("son"), py_str("brother"), py_str("uncle"), py_str("cousin"), py_str("grandad"), py_str("grandpa"), py_str("grandfather"), py_str("granddad"), py_str("partner"), py_str("boyfriend"), py_str("friend"), py_str("male friend"), py_str("nephew"), py_str("stepdad"), py_str("stepfather")]), &relation.clone()))) {
    pronoun = py_str("he");
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("wife"), py_str("mother"), py_str("mum"), py_str("mom"), py_str("daughter"), py_str("sister"), py_str("aunt"), py_str("gran"), py_str("granny"), py_str("grandma"), py_str("grandmother"), py_str("partner"), py_str("girlfriend"), py_str("friend"), py_str("niece"), py_str("stepmum"), py_str("stepmother"), py_str("nan"), py_str("nana"), py_str("mam"), py_str("mammy"), py_str("grannie"), py_str("mother-in-law"), py_str("sister-in-law"), py_str("daughter-in-law")]), &relation.clone()))) {
    pronoun = py_str("she");
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("kids"), py_str("children"), py_str("child"), py_str("parents"), py_str("friends"), py_str("mates"), py_str("mate")]), &relation.clone()))) {
    pronoun = py_str("they");
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("he"), py_str("hes")]), &who.clone()))) {
    pronoun = py_str("he");
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("she"), py_str("shes")]), &who.clone()))) {
    pronoun = py_str("she");
    } else {
    pronoun = py_str("they");
    }
    }
    }
    }
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("kids"), py_str("children"), py_str("parents"), py_str("friends")]), &relation.clone()))) {
    pronoun = py_str("they");
    }
    if py_truthy(&mcall(&g_CARE_HOME_GUILT(), "search", &[lowered.clone()])) {
    shape = py_str("guilt");
    } else {
    if py_truthy(&mcall(&g_CARE_HOME_VISIT(), "search", &[lowered.clone()])) {
    shape = py_str("visiting");
    } else {
    if py_truthy(&mcall(&g_CARE_HOME_MOVED(), "search", &[lowered.clone()])) {
    shape = py_str("moved");
    } else {
    shape = py_str("in");
    }
    }
    }
    return py_dict(vec![(py_str("text"), mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&g_CARE_HOME_PLACE(), "search", &[line.clone()]), "group", &[Py::Int(0)]), "split", &[])])), (py_str("who"), who.clone()), (py_str("relation"), relation.clone()), (py_str("pronoun"), pronoun.clone()), (py_str("shape"), shape.clone())]);
    Py::None
}
pub fn _a_line_asks(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let _ = py_str("Whether the line asks for something to be told or checked, so a fact is what it wants.\n\n    A request to *do* something - \"i need to cancel my booking\" - is not an ask about the world,\n    so only a question counts here. The things a line was read as asking about (`asks`) and a\n    question mark stand on their own.\n    ");
    if py_truthy(&Py::Bool(py_eq(&mcall(&read.clone(), "get", &[py_str("kind")]), &py_str("question")))) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g__A_REPORTS_THE_ASK(), "search", &[said.clone()])) {
    return call_builtin("bool", &[{ let __b = Py::Bool(py_contains(&said.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__ASKS_WITH_A_QUESTION_WORD(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__ASKS_WITH_AN_AUXILIARY(), "match", &[mcall(&said.clone(), "strip", &[])]); if py_truthy(&__b) { __b } else { mcall(&g__ASKS_AFTER_A_COMMA_WORD(), "search", &[mcall(&said.clone(), "replace", &[py_str(" ,"), py_str(",")])]) } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&mcall(&read.clone(), "get", &[py_str("asks")])) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&read.clone(), "get", &[py_str("intent")]), &py_str("question")))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &py_str("?")))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g__ASKS_WITH_A_QUESTION_WORD(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__ASKS_MID_LINE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__ASKS_AFTER_A_COMMA(), "search", &[mcall(&said.clone(), "replace", &[py_str(" ,"), py_str(",")])]); if py_truthy(&__b) { __b } else { mcall(&g__ASKS_WITH_AN_AUXILIARY(), "match", &[mcall(&said.clone(), "strip", &[])]) } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _as_typed(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The line exactly as it was typed, lower case and padded, signs and digits kept.");
    said = { let __b = mcall(&line.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&line.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    said = py_str("");
    }
    return add(&add(&py_str(" "), &mcall(&mcall(&said.clone(), "strip", &[]), "lower", &[])), &py_str(" "));
    Py::None
}
pub fn _carried_on(args: &[Py]) -> Py {
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
    let mut previous: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "previous") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: previous"))) };
    let mut words: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let _ = py_str("The reading a short follow-up carries on from the last line, or None.\n\n    After \"how much is a single to leeds\", \"and a return\" and \"is it cheaper after 9\" ask the same\n    question about the same kind of thing, and read on their own they say nothing about it. The\n    carry-on needs all of: this line read as a follow-up, few words, the last line's answer still\n    standing, and a different line than the last one - so \"and the icing\" after a cake is not a\n    price, and a reading written during the same turn never carries itself on.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[previous.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&previous.clone(), "get", &[py_str("over_time")]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&read.clone(), "get", &[py_str("followup")]))); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g__A_FOLLOW_UP_WORDS()) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&previous.clone(), "get", &[py_str("text")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&previous.clone(), "get", &[py_str("text")]), &mcall(&read.clone(), "get", &[py_str("text")]))) } }) {
    return Py::None;
    }
    return py_tuple(vec![Py::Bool(true), mcall(&previous.clone(), "get", &[py_str("over_time_what")])]);
    Py::None
}
pub fn _text(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The line for matching, in the spelling the writer meant, space-padded for whole words.");
    said = { let __b = mcall(&line.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&line.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    said = py_str("");
    }
    return add(&add(&py_str(" "), &mcall(&mcall(&said.clone(), "strip", &[]), "lower", &[])), &py_str(" "));
    Py::None
}
pub fn over_time(args: &[Py]) -> Py {
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
    let mut previous: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "previous") { v.clone() }
    else { Py::None };
    let mut carrying_on: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "carrying_on") { v.clone() }
    else { Py::Bool(false) };
    let mut asking: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut found: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut text: Py = Py::None;
    let mut what: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the line asks about something that holds only for the day: (True/False/None, what).\n\n    See the module docstring. `what` is the class of changing thing (`price`, `hours`,\n    `weather`, `news`, `availability`, `timetable`, `rate`, `event`, `post`) when the answer is\n    True, and None for the other two answers. `previous` is the last turn's reading, used only\n    for a follow-up that says nothing of its own: `carrying_on` asks for that second look alone,\n    once the reading of this line is complete, and answers None when the line carries nothing on.\n    ");
    read = if py_truthy(&call_builtin("isinstance", &[read.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { read.clone() } else { py_dict(vec![]) };
    text = _text(&[if py_truthy(&call_builtin("isinstance", &[line.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { line.clone() } else { py_dict(vec![]) }]);
    asking = _a_line_asks(&[read.clone(), text.clone()]);
    words = mcall(&text.clone(), "split", &[]);
    bare = le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g__A_BARE_ASK());
    if py_truthy(&{ let __b = asking.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_HOW_IT_WORKS(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_SAYS_NOW(), "search", &[text.clone()]))) } } } }) {
    return py_tuple(vec![Py::Bool(false), Py::None]);
    }
    if py_truthy(&{ let __b = mcall(&g__A_ARITHMETIC(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_ARITHMETIC(), "search", &[_as_typed(&[line.clone()])]) } }) {
    return py_tuple(vec![Py::Bool(false), Py::None]);
    }
    if py_truthy(&{ let __b = bitand(&g__SEALED_ASKS(), &call_builtin("set", &[{ let __b = mcall(&read.clone(), "get", &[py_str("asks")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_HAS_A_NUMBER(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_AN_AMOUNT_OF_A_THING(), "search", &[text.clone()]))) } } } }) {
    return py_tuple(vec![Py::Bool(false), Py::None]);
    }
    if py_truthy(&{ let __b = mcall(&g__A_PAST_YEAR(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_HOW_OLD_IS_THE_WORLD(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_HISTORY_ITSELF(), "search", &[text.clone()]) } } } }) {
    return py_tuple(vec![Py::Bool(false), Py::None]);
    }
    if py_truthy(&mcall(&g__A_A_DIFFERENCE(), "search", &[text.clone()])) {
    return py_tuple(vec![Py::Bool(false), Py::None]);
    }
    if py_truthy(&{ let __b = mcall(&g__A_HOW_A_THING_MEASURES(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_WEATHER_HERE(), "search", &[text.clone()]))) } }) {
    return py_tuple(vec![Py::Bool(false), Py::None]);
    }
    if py_truthy(&mcall(&g__A_A_TRAFFIC_LIGHT(), "search", &[text.clone()])) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    if py_truthy(&{ let __b = mcall(&g__A_A_PLAN_ABOUT_THE_WEATHER(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_WEATHER_HERE(), "search", &[text.clone()]))) } }) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    if py_truthy(&{ let __b = mcall(&g__A_HOW_MUCH_IT_HOLDS(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_HOW_MUCH_IS_IN(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_HOW_MUCH_ENOUGH(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_HOW_MUCH_A_FEELING(), "search", &[text.clone()]) } } } } } }) {
    return py_tuple(vec![Py::Bool(false), Py::None]);
    }
    if py_truthy(&mcall(&g__A_AN_OCCASION_OF_THEIR_OWN(), "search", &[text.clone()])) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g__MONEY(), "search", &[text.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_A_CONVERSION(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_HOW_MUCH_OF_A_UNIT(), "search", &[text.clone()]) } } } }) {
    return py_tuple(vec![Py::Bool(false), Py::None]);
    }
    if py_truthy(&{ let __b = mcall(&g__A_HOW_MUCH_I_SPENT(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_MY_OWN_FIGURE(), "search", &[text.clone()]) } }) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    if py_truthy(&mcall(&g__A_THE_CLOCK(), "search", &[text.clone()])) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    if py_truthy(&mcall(&g__A_DID_YOU_SEE(), "search", &[text.clone()])) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    if py_truthy(&{ let __b = mcall(&g__A_BARE_RECORD(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asking.clone())) } }) {
    return py_tuple(vec![Py::Bool(false), Py::None]);
    }
    fresh = mcall(&g__A_FRESHENING(), "sub", &[py_str(" "), text.clone()]);
    if py_truthy(&mcall(&g__A_WORD_MEANS(), "search", &[text.clone()])) {
    return py_tuple(vec![Py::Bool(false), Py::None]);
    }
    let __iter1 = py_iter_or_raise(&g__A_BARE_LIVE());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    what = __unpk2[0].clone();
    pattern = __unpk2[1].clone();
    found = mcall(&pattern.clone(), "search", &[fresh.clone()]);
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g__A_REPORTS(), "search", &[text.clone()]))); if !py_truthy(&__b) { __b } else { le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g__A_BARE_LIVE_WORDS()) } } } }) {
    return py_tuple(vec![Py::Bool(true), what.clone()]);
    }
    }
    let __iter3 = py_iter_or_raise(&g__CHANGING());
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    what = __unpk4[0].clone();
    pattern = __unpk4[1].clone();
    found = mcall(&pattern.clone(), "search", &[fresh.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("hours"), py_str("timetable"), py_str("event")]), &what.clone())); if !py_truthy(&__b) { __b } else { mcall(&g__A_MY_OWN_PLAN(), "search", &[text.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g__A_DEFINITION(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("what is"), py_str("what are"), py_str("whats"), py_str("what's")]), &mcall(&slice_or_raise(&fresh.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), "strip", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&slice_or_raise(&fresh.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" ?!.,")]))) } } } }) {
    return py_tuple(vec![Py::Bool(false), Py::None]);
    }
    if py_truthy(&asking.clone()) {
    return py_tuple(vec![Py::Bool(true), what.clone()]);
    }
    if py_truthy(&{ let __b = bare.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_REPORTS(), "search", &[text.clone()]))) } }) {
    tail = mcall(&slice_or_raise(&fresh.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__A_DETERMINER(), &mcall(&slice_or_raise(&fresh.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), "strip", &[]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&tail.clone())); if py_truthy(&__b) { __b } else { mcall(&g__A_BARE_TAIL(), "match", &[tail.clone()]) } } } }) {
    return py_tuple(vec![Py::Bool(true), what.clone()]);
    }
    }
    }
    if py_truthy(&carrying_on.clone()) {
    return _carried_on(&[read.clone(), previous.clone(), words.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&asking.clone()))) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    if py_truthy(&bitand(&g__SEALED_ASKS(), &call_builtin("set", &[{ let __b = mcall(&read.clone(), "get", &[py_str("asks")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)))) {
    return py_tuple(vec![Py::Bool(false), Py::None]);
    }
    let __iter5 = py_iter_or_raise(&g__SETTLED());
    for __item in __iter5 {
    pattern = __item.clone();
    if py_truthy(&mcall(&pattern.clone(), "search", &[text.clone()])) {
    return py_tuple(vec![Py::Bool(false), Py::None]);
    }
    }
    return py_tuple(vec![Py::None, Py::None]);
    Py::None
}
pub fn _AServiceOnADay____init__(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut self_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "self") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: self"))) };
    let mut service: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "service") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: service"))) };
    let mut when: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let __unpk1 = py_unpack(&py_tuple(vec![service.clone(), when.clone()]), 2, None);
    let _ = py_setattr(&self_.clone(), "service", __unpk1[0].clone());
    let _ = py_setattr(&self_.clone(), "when", __unpk1[1].clone());
    Py::None
}
pub fn _AServiceOnADay__search(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut self_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "self") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: self"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut service: Py = Py::None;
    let mut when: Py = Py::None;
    service = mcall(&gattr(&self_.clone(), "service"), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(py_is(&service.clone(), &Py::None))) {
    return Py::None;
    }
    when = mcall(&gattr(&self_.clone(), "when"), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(py_is(&when.clone(), &Py::None))) {
    return Py::None;
    }
    return if py_truthy(&le(&mcall(&service.clone(), "start", &[]), &mcall(&when.clone(), "start", &[]))) { service.clone() } else { when.clone() };
    Py::None
}
pub fn _AServiceOnADay(args: &[Py]) -> Py {
    ensure_init();
    let inst = Py::Instance(Rc::new(PyInstance { class_name: "_AServiceOnADay".to_string(), attrs: Rc::new(RefCell::new(IndexMap::new())) }));
    let mut full = vec![inst.clone()]; full.extend_from_slice(args);
    _AServiceOnADay____init__(&full);
    inst
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.understanding.kinds".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABILITY_AT" => Some(g_ABILITY_AT()),
            "ABOUT_AT_A_TIME" => Some(g_ABOUT_AT_A_TIME()),
            "ABOUT_BOT" => Some(g_ABOUT_BOT()),
            "ABOUT_BOT_END" => Some(g_ABOUT_BOT_END()),
            "ACK" => Some(g_ACK()),
            "ACTS_ON_THE_SPEAKER" => Some(g_ACTS_ON_THE_SPEAKER()),
            "ACT_ON_FILLER" => Some(g_ACT_ON_FILLER()),
            "ACT_ON_LEAD" => Some(g_ACT_ON_LEAD()),
            "ADDRESSED" => Some(g_ADDRESSED()),
            "ADDRESSED_LINES" => Some(g_ADDRESSED_LINES()),
            "ADDRESS_WORDS" => Some(g_ADDRESS_WORDS()),
            "AFFIRM" => Some(g_AFFIRM()),
            "AFTER_COMPARATIVE" => Some(g_AFTER_COMPARATIVE()),
            "AFTER_HELP_WAS_GOOD" => Some(g_AFTER_HELP_WAS_GOOD()),
            "AFTER_HELP_WAS_GOOD_SAID_OUT" => Some(g_AFTER_HELP_WAS_GOOD_SAID_OUT()),
            "AGREEMENT_IDIOMS" => Some(g_AGREEMENT_IDIOMS()),
            "AGREE_AFTER_NOT" => Some(g_AGREE_AFTER_NOT()),
            "AGREE_TAIL" => Some(g_AGREE_TAIL()),
            "ALREADY_ASKED" => Some(g_ALREADY_ASKED()),
            "ALWAYS_SELF_ONLY" => Some(g_ALWAYS_SELF_ONLY()),
            "ANSWERING_A_QUOTED_ASK" => Some(g_ANSWERING_A_QUOTED_ASK()),
            "ANY_ALREADY_ASKS" => Some(g_ANY_ALREADY_ASKS()),
            "ANY_ANSWER_LINKS" => Some(g_ANY_ANSWER_LINKS()),
            "ANY_ANSWER_WANTED" => Some(g_ANY_ANSWER_WANTED()),
            "ASKING_SUBJECT" => Some(g_ASKING_SUBJECT()),
            "ASKING_VERBS" => Some(g_ASKING_VERBS()),
            "ASKS_ABOUT_A_PERSON" => Some(g_ASKS_ABOUT_A_PERSON()),
            "ASKS_IF_ANYTHING" => Some(g_ASKS_IF_ANYTHING()),
            "ASKS_IT_BACK" => Some(g_ASKS_IT_BACK()),
            "ASK_ABOUT" => Some(g_ASK_ABOUT()),
            "ASK_AUX" => Some(g_ASK_AUX()),
            "ASK_FOLLOW" => Some(g_ASK_FOLLOW()),
            "ASK_FOR_A_NAME" => Some(g_ASK_FOR_A_NAME()),
            "ASK_FRAMES" => Some(g_ASK_FRAMES()),
            "ASK_KEYS" => Some(g_ASK_KEYS()),
            "ASK_MODALS" => Some(g_ASK_MODALS()),
            "ASK_TARGET" => Some(g_ASK_TARGET()),
            "ASK_THE_ROOM" => Some(g_ASK_THE_ROOM()),
            "ASK_THE_ROOM_ANY" => Some(g_ASK_THE_ROOM_ANY()),
            "ASK_THE_ROOM_VERB" => Some(g_ASK_THE_ROOM_VERB()),
            "ASK_THE_ROOM_WANTS" => Some(g_ASK_THE_ROOM_WANTS()),
            "ASK_VERBS" => Some(g_ASK_VERBS()),
            "ASSISTANT_NAMES" => Some(g_ASSISTANT_NAMES()),
            "A_BODY_SUBJECT" => Some(g_A_BODY_SUBJECT()),
            "A_BROKEN_HEART" => Some(g_A_BROKEN_HEART()),
            "A_COMPARISON" => Some(g_A_COMPARISON()),
            "A_DATE_FIGURE" => Some(g_A_DATE_FIGURE()),
            "A_FRAME_IN_FRONT" => Some(g_A_FRAME_IN_FRONT()),
            "A_HAVING" => Some(g_A_HAVING()),
            "A_HOW_MANY_OF_THE_BOTS_OWN" => Some(g_A_HOW_MANY_OF_THE_BOTS_OWN()),
            "A_LIMIT_ABOUT_PEOPLE" => Some(g_A_LIMIT_ABOUT_PEOPLE()),
            "A_MONEY_SUBJECT" => Some(g_A_MONEY_SUBJECT()),
            "A_MONTH_PERIOD" => Some(g_A_MONTH_PERIOD()),
            "A_NOT_A_SUGGESTION" => Some(g_A_NOT_A_SUGGESTION()),
            "A_PERIOD" => Some(g_A_PERIOD()),
            "A_PERSON_AFTER" => Some(g_A_PERSON_AFTER()),
            "A_PERSON_AFTER_A_POSSESSIVE" => Some(g_A_PERSON_AFTER_A_POSSESSIVE()),
            "A_PERSON_ELSE" => Some(g_A_PERSON_ELSE()),
            "A_PERSON_IS_THE_SUBJECT" => Some(g_A_PERSON_IS_THE_SUBJECT()),
            "A_PERSON_NAMED" => Some(g_A_PERSON_NAMED()),
            "A_POSSESSED_HEART" => Some(g_A_POSSESSED_HEART()),
            "A_REPORTED_VERB" => Some(g_A_REPORTED_VERB()),
            "A_SAID_BY" => Some(g_A_SAID_BY()),
            "A_SAID_BY_ANYONE" => Some(g_A_SAID_BY_ANYONE()),
            "A_SUGGESTED_CLAUSE" => Some(g_A_SUGGESTED_CLAUSE()),
            "A_THING_NOT_A_CLAIM" => Some(g_A_THING_NOT_A_CLAIM()),
            "A_TOLD_TO" => Some(g_A_TOLD_TO()),
            "A_WHAT_ITS_LIKE" => Some(g_A_WHAT_ITS_LIKE()),
            "A_YOUR_KIN" => Some(g_A_YOUR_KIN()),
            "A_YOUR_OWN_THING" => Some(g_A_YOUR_OWN_THING()),
            "BACK_AFTER" => Some(g_BACK_AFTER()),
            "BACK_ASKING" => Some(g_BACK_ASKING()),
            "BACK_AT" => Some(g_BACK_AT()),
            "BACK_INTO" => Some(g_BACK_INTO()),
            "BACK_JOIN" => Some(g_BACK_JOIN()),
            "BACK_NOT_A_SUBJECT" => Some(g_BACK_NOT_A_SUBJECT()),
            "BARE_DEFINITION" => Some(g_BARE_DEFINITION()),
            "BARE_VERBS" => Some(g_BARE_VERBS()),
            "BEEN_SAID" => Some(g_BEEN_SAID()),
            "BEEN_WHEN" => Some(g_BEEN_WHEN()),
            "BEFORE_COURTESY" => Some(g_BEFORE_COURTESY()),
            "BEFORE_HELP_WAS_GOOD" => Some(g_BEFORE_HELP_WAS_GOOD()),
            "BEFORE_STATE" => Some(g_BEFORE_STATE()),
            "BETTER" => Some(g_BETTER()),
            "BETTER_OFF" => Some(g_BETTER_OFF()),
            "BETWEEN_STATE_AND_NOUN" => Some(g_BETWEEN_STATE_AND_NOUN()),
            "BE_FORMS" => Some(g_BE_FORMS()),
            "BODY_STATE_CUES" => Some(g_BODY_STATE_CUES()),
            "BODY_STATE_WORDS" => Some(g_BODY_STATE_WORDS()),
            "BODY_SUBJECT" => Some(g_BODY_SUBJECT()),
            "BOT_CHATS" => Some(g_BOT_CHATS()),
            "BOT_COUNTED" => Some(g_BOT_COUNTED()),
            "BOT_DAY_SHAPES" => Some(g_BOT_DAY_SHAPES()),
            "BOT_DAY_WORDS" => Some(g_BOT_DAY_WORDS()),
            "BOT_DOERS" => Some(g_BOT_DOERS()),
            "BOT_DOINGS" => Some(g_BOT_DOINGS()),
            "BOT_HAVE" => Some(g_BOT_HAVE()),
            "BOT_LIFE" => Some(g_BOT_LIFE()),
            "BOT_LIFE_ASK" => Some(g_BOT_LIFE_ASK()),
            "BOT_LIFE_FILLER" => Some(g_BOT_LIFE_FILLER()),
            "BOUT" => Some(g_BOUT()),
            "BROKEN_HEARTED" => Some(g_BROKEN_HEARTED()),
            "BROKE_A_HEART" => Some(g_BROKE_A_HEART()),
            "CAPABILITY" => Some(g_CAPABILITY()),
            "CAPABILITY_CARRIER" => Some(g_CAPABILITY_CARRIER()),
            "CAPABILITY_FILLER" => Some(g_CAPABILITY_FILLER()),
            "CARE_HOME_BRING_ASKS" => Some(g_CARE_HOME_BRING_ASKS()),
            "CARE_HOME_BRING_WHO" => Some(g_CARE_HOME_BRING_WHO()),
            "CARE_HOME_BRING_WORD" => Some(g_CARE_HOME_BRING_WORD()),
            "CARE_HOME_GUILT" => Some(g_CARE_HOME_GUILT()),
            "CARE_HOME_KIN" => Some(g_CARE_HOME_KIN()),
            "CARE_HOME_MOVED" => Some(g_CARE_HOME_MOVED()),
            "CARE_HOME_NOT" => Some(g_CARE_HOME_NOT()),
            "CARE_HOME_PLACE" => Some(g_CARE_HOME_PLACE()),
            "CARE_HOME_VISIT" => Some(g_CARE_HOME_VISIT()),
            "CHILD_NOUNS" => Some(g_CHILD_NOUNS()),
            "CHOICE" => Some(g_CHOICE()),
            "CLAIM_ENDS" => Some(g_CLAIM_ENDS()),
            "CLAIM_STEMS" => Some(g_CLAIM_STEMS()),
            "CLAUSE_HELPERS" => Some(g_CLAUSE_HELPERS()),
            "CLAUSE_OPENERS" => Some(g_CLAUSE_OPENERS()),
            "CLAUSE_VERBS" => Some(g_CLAUSE_VERBS()),
            "COMMIT_ALONE" => Some(g_COMMIT_ALONE()),
            "COMMIT_ALONE_TAIL" => Some(g_COMMIT_ALONE_TAIL()),
            "COMMIT_NO_HARM" => Some(g_COMMIT_NO_HARM()),
            "COMMIT_OPEN" => Some(g_COMMIT_OPEN()),
            "COMMIT_STOP" => Some(g_COMMIT_STOP()),
            "COMMIT_SUBJECT" => Some(g_COMMIT_SUBJECT()),
            "COMMIT_TRY" => Some(g_COMMIT_TRY()),
            "COMPARATIVE" => Some(g_COMPARATIVE()),
            "COMPARE" => Some(g_COMPARE()),
            "COMPARISON" => Some(g_COMPARISON()),
            "CONFUSED_SELF_ONLY" => Some(g_CONFUSED_SELF_ONLY()),
            "CONJUNCTIONS" => Some(g_CONJUNCTIONS()),
            "COPULA" => Some(g_COPULA()),
            "COPULAS" => Some(g_COPULAS()),
            "COPULA_BEFORE" => Some(g_COPULA_BEFORE()),
            "COPULA_STATES" => Some(g_COPULA_STATES()),
            "CORRECTION" => Some(g_CORRECTION()),
            "COUNT_FILLERS" => Some(g_COUNT_FILLERS()),
            "COUNT_OPENERS" => Some(g_COUNT_OPENERS()),
            "COURTESY_THING" => Some(g_COURTESY_THING()),
            "COURTESY_WORD" => Some(g_COURTESY_WORD()),
            "CUE_FOLLOW" => Some(g_CUE_FOLLOW()),
            "DEFINITION" => Some(g_DEFINITION()),
            "DEGREE" => Some(g_DEGREE()),
            "DENY" => Some(g_DENY()),
            "DENY_BEFORE" => Some(g_DENY_BEFORE()),
            "DENY_BETWEEN" => Some(g_DENY_BETWEEN()),
            "DESCRIBES" => Some(g_DESCRIBES()),
            "DETERMINERS" => Some(g_DETERMINERS()),
            "DOING_FILLER" => Some(g_DOING_FILLER()),
            "ECHO_CARRIERS" => Some(g_ECHO_CARRIERS()),
            "EMOTION_WORDS" => Some(g_EMOTION_WORDS()),
            "EPISODE_NOUNS" => Some(g_EPISODE_NOUNS()),
            "EVENT_NOUNS" => Some(g_EVENT_NOUNS()),
            "EVENT_WORDS" => Some(g_EVENT_WORDS()),
            "EXPERIENCE_ASK" => Some(g_EXPERIENCE_ASK()),
            "EXPERIENCE_WORDS" => Some(g_EXPERIENCE_WORDS()),
            "FALL" => Some(g_FALL()),
            "FAREWELLS" => Some(g_FAREWELLS()),
            "FAREWELL_TAIL" => Some(g_FAREWELL_TAIL()),
            "FEELING_COPULA" => Some(g_FEELING_COPULA()),
            "FEELING_CUES_BEFORE" => Some(g_FEELING_CUES_BEFORE()),
            "FEELING_FILLER" => Some(g_FEELING_FILLER()),
            "FEELING_NOT_MOOD" => Some(g_FEELING_NOT_MOOD()),
            "FEELING_SOUND_FRAME" => Some(g_FEELING_SOUND_FRAME()),
            "FELL_WORDS" => Some(g_FELL_WORDS()),
            "FILLER_AFTER" => Some(g_FILLER_AFTER()),
            "FILLER_BEFORE" => Some(g_FILLER_BEFORE()),
            "FIRST_PERSON" => Some(g_FIRST_PERSON()),
            "FOUND" => Some(g_FOUND()),
            "FRAME_NOT_A_SETUP" => Some(g_FRAME_NOT_A_SETUP()),
            "FREE_OR_BUSY" => Some(g_FREE_OR_BUSY()),
            "GENDER_WORDS" => Some(g_GENDER_WORDS()),
            "GETTING_WORSE" => Some(g_GETTING_WORSE()),
            "GIVEN_ADVICE" => Some(g_GIVEN_ADVICE()),
            "GOING_ON_WHEN" => Some(g_GOING_ON_WHEN()),
            "GOOD_EMOTION_NAMES" => Some(g_GOOD_EMOTION_NAMES()),
            "GOOD_WORD_BY_COMPARISON" => Some(g_GOOD_WORD_BY_COMPARISON()),
            "GOOD_WORD_COMPARED" => Some(g_GOOD_WORD_COMPARED()),
            "GOOD_WORD_MISSING" => Some(g_GOOD_WORD_MISSING()),
            "GRATITUDE_ASK_AGAIN" => Some(g_GRATITUDE_ASK_AGAIN()),
            "GRATITUDE_JOINERS" => Some(g_GRATITUDE_JOINERS()),
            "GRATITUDE_OBJECT" => Some(g_GRATITUDE_OBJECT()),
            "GREETINGS" => Some(g_GREETINGS()),
            "GREETING_ASK_ABOUT_SOMETHING" => Some(g_GREETING_ASK_ABOUT_SOMETHING()),
            "GREETING_ASK_SECOND_PERSON" => Some(g_GREETING_ASK_SECOND_PERSON()),
            "GREETING_ASK_STATE" => Some(g_GREETING_ASK_STATE()),
            "GREETING_ASK_SUBJECT" => Some(g_GREETING_ASK_SUBJECT()),
            "GREETING_ASK_WORDS" => Some(g_GREETING_ASK_WORDS()),
            "GREETING_TAIL" => Some(g_GREETING_TAIL()),
            "GREETING_WHAT_SAYING" => Some(g_GREETING_WHAT_SAYING()),
            "HAND_OVER_LEADS" => Some(g_HAND_OVER_LEADS()),
            "HAND_OVER_MINE" => Some(g_HAND_OVER_MINE()),
            "HAND_OVER_MODALS" => Some(g_HAND_OVER_MODALS()),
            "HAND_OVER_SUBJECTS" => Some(g_HAND_OVER_SUBJECTS()),
            "HAND_OVER_TO" => Some(g_HAND_OVER_TO()),
            "HAND_OVER_VERBS" => Some(g_HAND_OVER_VERBS()),
            "HAVE_A_BROKEN_HEART" => Some(g_HAVE_A_BROKEN_HEART()),
            "HEART_DEGREES" => Some(g_HEART_DEGREES()),
            "HEART_IN_A_STATE" => Some(g_HEART_IN_A_STATE()),
            "HEDGED_ALONE" => Some(g_HEDGED_ALONE()),
            "HEDGED_COMMIT" => Some(g_HEDGED_COMMIT()),
            "HEDGED_DOUBT" => Some(g_HEDGED_DOUBT()),
            "HEDGED_LOW" => Some(g_HEDGED_LOW()),
            "HEDGED_OK" => Some(g_HEDGED_OK()),
            "HEDGED_WRAPPER" => Some(g_HEDGED_WRAPPER()),
            "HEDGE_LEAD_ONE" => Some(g_HEDGE_LEAD_ONE()),
            "HEDGE_LEAD_TWO" => Some(g_HEDGE_LEAD_TWO()),
            "HELP_ASK" => Some(g_HELP_ASK()),
            "HELP_ASK_NOT" => Some(g_HELP_ASK_NOT()),
            "HELP_ASK_OWN_SUBJECT" => Some(g_HELP_ASK_OWN_SUBJECT()),
            "HELP_ASK_SUBJECT" => Some(g_HELP_ASK_SUBJECT()),
            "HELP_ASK_TAIL_CLAUSE" => Some(g_HELP_ASK_TAIL_CLAUSE()),
            "HELP_WAS_GOOD" => Some(g_HELP_WAS_GOOD()),
            "IDIOM_STATES" => Some(g_IDIOM_STATES()),
            "ING_TAKES_NO_OBJECT" => Some(g_ING_TAKES_NO_OBJECT()),
            "ING_THINGS" => Some(g_ING_THINGS()),
            "INTEREST_HEAD" => Some(g_INTEREST_HEAD()),
            "INTEREST_NOT" => Some(g_INTEREST_NOT()),
            "INTEREST_TAIL" => Some(g_INTEREST_TAIL()),
            "INTRODUCES" => Some(g_INTRODUCES()),
            "ITSELF_LIFE" => Some(g_ITSELF_LIFE()),
            "ITSELF_LONG" => Some(g_ITSELF_LONG()),
            "ITSELF_WHEN" => Some(g_ITSELF_WHEN()),
            "ITSELF_WHEN_TAIL" => Some(g_ITSELF_WHEN_TAIL()),
            "JOIN_OPENERS" => Some(g_JOIN_OPENERS()),
            "KEEPING_ASK" => Some(g_KEEPING_ASK()),
            "KEEPS_BUSY_SHAPE" => Some(g_KEEPS_BUSY_SHAPE()),
            "KIN" => Some(g_KIN()),
            "LABEL_ON_A_PERSON" => Some(g_LABEL_ON_A_PERSON()),
            "LAUGH" => Some(g_LAUGH()),
            "LAUGHED_OFF_AFTER" => Some(g_LAUGHED_OFF_AFTER()),
            "LAUGH_AT_THE_END" => Some(g_LAUGH_AT_THE_END()),
            "LEARNED_ON" => Some(g_LEARNED_ON()),
            "LEARNING_WORDS" => Some(g_LEARNING_WORDS()),
            "LETTER_ASKED_FOR" => Some(g_LETTER_ASKED_FOR()),
            "LETTER_ITSELF" => Some(g_LETTER_ITSELF()),
            "LETTER_RECEIVED" => Some(g_LETTER_RECEIVED()),
            "LETTER_SOMEONE_ELSES" => Some(g_LETTER_SOMEONE_ELSES()),
            "LIMIT_A_FACT" => Some(g_LIMIT_A_FACT()),
            "LIMIT_NOT_A_START" => Some(g_LIMIT_NOT_A_START()),
            "LIMIT_NOT_A_THING" => Some(g_LIMIT_NOT_A_THING()),
            "LIMIT_REFUSAL" => Some(g_LIMIT_REFUSAL()),
            "LIMIT_ROOM" => Some(g_LIMIT_ROOM()),
            "LIMIT_SHAPES" => Some(g_LIMIT_SHAPES()),
            "LIMIT_SOFT_AFTER" => Some(g_LIMIT_SOFT_AFTER()),
            "LIMIT_SOFT_BEFORE" => Some(g_LIMIT_SOFT_BEFORE()),
            "MAKER_WORDS" => Some(g_MAKER_WORDS()),
            "MANIPULATE" => Some(g_MANIPULATE()),
            "MANNERS_OPENERS" => Some(g_MANNERS_OPENERS()),
            "MANNERS_WORDS" => Some(g_MANNERS_WORDS()),
            "MATH_WORDS" => Some(g_MATH_WORDS()),
            "MEANT_APPLIED_TO" => Some(g_MEANT_APPLIED_TO()),
            "ME_ASKS" => Some(g_ME_ASKS()),
            "ME_LEAD" => Some(g_ME_LEAD()),
            "ME_TAIL" => Some(g_ME_TAIL()),
            "MOOD_NOUNS" => Some(g_MOOD_NOUNS()),
            "MOOD_TAIL" => Some(g_MOOD_TAIL()),
            "MOOD_TAIL_TAKEN" => Some(g_MOOD_TAIL_TAKEN()),
            "MOOD_TAIL_WORDS" => Some(g_MOOD_TAIL_WORDS()),
            "MOTION_WORDS" => Some(g_MOTION_WORDS()),
            "NAMING_WORDS" => Some(g_NAMING_WORDS()),
            "NEEDS_ANSWER" => Some(g_NEEDS_ANSWER()),
            "NEGATIONS" => Some(g_NEGATIONS()),
            "NEGATION_SKIP" => Some(g_NEGATION_SKIP()),
            "NEGATIVE" => Some(g_NEGATIVE()),
            "NEGATORS" => Some(g_NEGATORS()),
            "NEWS" => Some(g_NEWS()),
            "NEW_ASK_AHEAD" => Some(g_NEW_ASK_AHEAD()),
            "NEW_ASK_WORDS" => Some(g_NEW_ASK_WORDS()),
            "NEW_FRAME" => Some(g_NEW_FRAME()),
            "NOT_AFTER_THE_BOT" => Some(g_NOT_AFTER_THE_BOT()),
            "NOT_AUX" => Some(g_NOT_AUX()),
            "NOT_A_DEFINITION_HEAD" => Some(g_NOT_A_DEFINITION_HEAD()),
            "NOT_A_MOOD_THERE" => Some(g_NOT_A_MOOD_THERE()),
            "NOT_A_SUBJECT" => Some(g_NOT_A_SUBJECT()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NOT_BEFORE_THE_BOT" => Some(g_NOT_BEFORE_THE_BOT()),
            "NOT_FILLER" => Some(g_NOT_FILLER()),
            "NOT_THEIR_OWN" => Some(g_NOT_THEIR_OWN()),
            "NOT_THE_SPEAKERS_DAY" => Some(g_NOT_THE_SPEAKERS_DAY()),
            "NOUN_REQUEST_MAX" => Some(g_NOUN_REQUEST_MAX()),
            "NOUN_REQUEST_SELF" => Some(g_NOUN_REQUEST_SELF()),
            "NOUN_STATES" => Some(g_NOUN_STATES()),
            "NO_BETTER" => Some(g_NO_BETTER()),
            "NO_HARM_SUBJECT" => Some(g_NO_HARM_SUBJECT()),
            "NUMBER_WORDS" => Some(g_NUMBER_WORDS()),
            "OBJECT_MARKERS" => Some(g_OBJECT_MARKERS()),
            "OK_WORDS" => Some(g_OK_WORDS()),
            "OPENING_FRAME" => Some(g_OPENING_FRAME()),
            "OPENING_MAX" => Some(g_OPENING_MAX()),
            "OPENING_WORDS" => Some(g_OPENING_WORDS()),
            "OPENS_A_FRAME" => Some(g_OPENS_A_FRAME()),
            "OPINION" => Some(g_OPINION()),
            "OPPOSITE_PAIRS" => Some(g_OPPOSITE_PAIRS()),
            "OTHER_PEOPLE" => Some(g_OTHER_PEOPLE()),
            "OWNS_A_PERSON" => Some(g_OWNS_A_PERSON()),
            "OWN_PERIOD_FILLER" => Some(g_OWN_PERIOD_FILLER()),
            "OWN_PERIOD_HEADS" => Some(g_OWN_PERIOD_HEADS()),
            "OWN_PERIOD_NOUNS" => Some(g_OWN_PERIOD_NOUNS()),
            "OWN_PERIOD_REACH" => Some(g_OWN_PERIOD_REACH()),
            "OWN_PERIOD_STOPS" => Some(g_OWN_PERIOD_STOPS()),
            "OWN_RESERVES" => Some(g_OWN_RESERVES()),
            "PARTICLE_STATES" => Some(g_PARTICLE_STATES()),
            "PARTS_OF_ME" => Some(g_PARTS_OF_ME()),
            "PAST_ASK" => Some(g_PAST_ASK()),
            "PAST_AUX" => Some(g_PAST_AUX()),
            "PAST_ENDS" => Some(g_PAST_ENDS()),
            "PERIOD_CUES" => Some(g_PERIOD_CUES()),
            "PERSON_NOUNS" => Some(g_PERSON_NOUNS()),
            "PERSON_PREPOSITIONS" => Some(g_PERSON_PREPOSITIONS()),
            "PERSON_SUBJECT" => Some(g_PERSON_SUBJECT()),
            "PHRASE_OPENERS" => Some(g_PHRASE_OPENERS()),
            "PICKING" => Some(g_PICKING()),
            "PICKING_QUALITY" => Some(g_PICKING_QUALITY()),
            "PLACE_WORD" => Some(g_PLACE_WORD()),
            "POINTED_AT" => Some(g_POINTED_AT()),
            "POLITE" => Some(g_POLITE()),
            "POSITIVE" => Some(g_POSITIVE()),
            "POSSESSIVE_WORDS" => Some(g_POSSESSIVE_WORDS()),
            "PRAISE_SAID" => Some(g_PRAISE_SAID()),
            "PRAISING" => Some(g_PRAISING()),
            "PRAISING_VERB" => Some(g_PRAISING_VERB()),
            "PRODUCT_ASK_TAIL" => Some(g_PRODUCT_ASK_TAIL()),
            "PRODUCT_ASK_WORDS" => Some(g_PRODUCT_ASK_WORDS()),
            "PRONOUNS" => Some(g_PRONOUNS()),
            "PRONOUN_SUBJECT" => Some(g_PRONOUN_SUBJECT()),
            "QUESTION_NOUNS" => Some(g_QUESTION_NOUNS()),
            "QUESTION_STARTS" => Some(g_QUESTION_STARTS()),
            "QUESTION_WORD" => Some(g_QUESTION_WORD()),
            "QUESTION_WORDS" => Some(g_QUESTION_WORDS()),
            "RECOMMEND" => Some(g_RECOMMEND()),
            "RECOMMEND_PHRASES" => Some(g_RECOMMEND_PHRASES()),
            "REFERRING" => Some(g_REFERRING()),
            "REFUSED_PAIR" => Some(g_REFUSED_PAIR()),
            "RELATIVE_SUBJECTS" => Some(g_RELATIVE_SUBJECTS()),
            "REQUEST" => Some(g_REQUEST()),
            "ROBOT_WORDS" => Some(g_ROBOT_WORDS()),
            "ROUTINE_IN_THE_PAST" => Some(g_ROUTINE_IN_THE_PAST()),
            "ROUTINE_IS_A_FAULT" => Some(g_ROUTINE_IS_A_FAULT()),
            "ROUTINE_SHAPE" => Some(g_ROUTINE_SHAPE()),
            "RULED_OUT" => Some(g_RULED_OUT()),
            "RULED_OUT_CORRECTION" => Some(g_RULED_OUT_CORRECTION()),
            "RULED_OUT_ENDS" => Some(g_RULED_OUT_ENDS()),
            "RULED_OUT_NO_THING" => Some(g_RULED_OUT_NO_THING()),
            "SAD_PLAIN_CUES" => Some(g_SAD_PLAIN_CUES()),
            "SAID_AS_A_NEGATION" => Some(g_SAID_AS_A_NEGATION()),
            "SAYS_ABOUT_YOU" => Some(g_SAYS_ABOUT_YOU()),
            "SAYS_HOW_THEY_ARE" => Some(g_SAYS_HOW_THEY_ARE()),
            "SAYS_WHAT_ARE_YOU" => Some(g_SAYS_WHAT_ARE_YOU()),
            "SAYS_WHAT_IT_IS" => Some(g_SAYS_WHAT_IT_IS()),
            "SCALE_ASK" => Some(g_SCALE_ASK()),
            "SELF_ASK" => Some(g_SELF_ASK()),
            "SELF_FILLER" => Some(g_SELF_FILLER()),
            "SELF_ONLY_CUES" => Some(g_SELF_ONLY_CUES()),
            "SELF_ONLY_STATE" => Some(g_SELF_ONLY_STATE()),
            "SELF_WORDS" => Some(g_SELF_WORDS()),
            "SETUP_CONNECTIVES" => Some(g_SETUP_CONNECTIVES()),
            "SETUP_MOST_WORDS" => Some(g_SETUP_MOST_WORDS()),
            "SETUP_NOTHING" => Some(g_SETUP_NOTHING()),
            "SHIFT_EARLY" => Some(g_SHIFT_EARLY()),
            "SHIFT_LATE" => Some(g_SHIFT_LATE()),
            "SHIFT_NIGHT" => Some(g_SHIFT_NIGHT()),
            "SHORT_REPLIES" => Some(g_SHORT_REPLIES()),
            "SIGNING_OFF" => Some(g_SIGNING_OFF()),
            "SLIPPED_FROM" => Some(g_SLIPPED_FROM()),
            "SMALL_TALK" => Some(g_SMALL_TALK()),
            "SMALL_TALK_LEFTOVER" => Some(g_SMALL_TALK_LEFTOVER()),
            "SMALL_TALK_WORDS" => Some(g_SMALL_TALK_WORDS()),
            "SOUND_FOLLOWS" => Some(g_SOUND_FOLLOWS()),
            "SPEAKER" => Some(g_SPEAKER()),
            "SPEAKER_TAIL" => Some(g_SPEAKER_TAIL()),
            "SPENDS_ITS_TIME" => Some(g_SPENDS_ITS_TIME()),
            "SPENT_TIME" => Some(g_SPENT_TIME()),
            "STARTED_OUT" => Some(g_STARTED_OUT()),
            "STATE_AS_FEELING" => Some(g_STATE_AS_FEELING()),
            "STATE_JOINERS" => Some(g_STATE_JOINERS()),
            "STATE_NOUNS" => Some(g_STATE_NOUNS()),
            "STATE_TAIL" => Some(g_STATE_TAIL()),
            "STATE_WORDS" => Some(g_STATE_WORDS()),
            "STATING_VERB" => Some(g_STATING_VERB()),
            "STEPS" => Some(g_STEPS()),
            "STEPS_WORD" => Some(g_STEPS_WORD()),
            "STEP_ANY_OBJECT" => Some(g_STEP_ANY_OBJECT()),
            "STEP_VERBS" => Some(g_STEP_VERBS()),
            "STEP_WRONG" => Some(g_STEP_WRONG()),
            "STILL_COMING" => Some(g_STILL_COMING()),
            "STILL_WRONG" => Some(g_STILL_WRONG()),
            "STOMACH_WORDS" => Some(g_STOMACH_WORDS()),
            "STOP_ASK" => Some(g_STOP_ASK()),
            "STOP_ASK_FILLER" => Some(g_STOP_ASK_FILLER()),
            "SUBJECT_NEGATIONS" => Some(g_SUBJECT_NEGATIONS()),
            "SUBJECT_OF_ITS_OWN" => Some(g_SUBJECT_OF_ITS_OWN()),
            "SUMMONS_CALL_DONE" => Some(g_SUMMONS_CALL_DONE()),
            "SUMMONS_NOT" => Some(g_SUMMONS_NOT()),
            "SUMMONS_RANG" => Some(g_SUMMONS_RANG()),
            "SUMMONS_SAID" => Some(g_SUMMONS_SAID()),
            "SUMMONS_SEE" => Some(g_SUMMONS_SEE()),
            "SUMMONS_VERB" => Some(g_SUMMONS_VERB()),
            "SUMMONS_WANTS" => Some(g_SUMMONS_WANTS()),
            "SUMMONS_WHO" => Some(g_SUMMONS_WHO()),
            "SUMMONS_WHO_PHRASES" => Some(g_SUMMONS_WHO_PHRASES()),
            "SUMMONS_WRITTEN" => Some(g_SUMMONS_WRITTEN()),
            "SUPPLY_PREPOSITIONS" => Some(g_SUPPLY_PREPOSITIONS()),
            "SUPPLY_WORDS" => Some(g_SUPPLY_WORDS()),
            "TAG_CHECKABLE" => Some(g_TAG_CHECKABLE()),
            "TAG_ENDINGS" => Some(g_TAG_ENDINGS()),
            "TAG_PLAN" => Some(g_TAG_PLAN()),
            "TAG_RIGHT_NOT" => Some(g_TAG_RIGHT_NOT()),
            "TAG_TIME_NUMBER" => Some(g_TAG_TIME_NUMBER()),
            "TAIL_ASKS" => Some(g_TAIL_ASKS()),
            "TAIL_ASK_FILLER" => Some(g_TAIL_ASK_FILLER()),
            "TAIL_STOPS" => Some(g_TAIL_STOPS()),
            "TALK_ASK_OPENERS" => Some(g_TALK_ASK_OPENERS()),
            "TALK_ASK_VERBS" => Some(g_TALK_ASK_VERBS()),
            "TALK_HEAD" => Some(g_TALK_HEAD()),
            "TALK_OPENING" => Some(g_TALK_OPENING()),
            "TALK_TAIL" => Some(g_TALK_TAIL()),
            "TALK_TAIL_WORDS" => Some(g_TALK_TAIL_WORDS()),
            "TASTE_VERBS" => Some(g_TASTE_VERBS()),
            "TERMS_OF_ADDRESS" => Some(g_TERMS_OF_ADDRESS()),
            "THANKS" => Some(g_THANKS()),
            "THANKS_ABOUT" => Some(g_THANKS_ABOUT()),
            "THANKS_ASK_NOUNS" => Some(g_THANKS_ASK_NOUNS()),
            "THANKS_ASK_PREPS" => Some(g_THANKS_ASK_PREPS()),
            "THANKS_ASK_WORDS" => Some(g_THANKS_ASK_WORDS()),
            "THANKS_NOISE" => Some(g_THANKS_NOISE()),
            "THEIR_PROBLEM" => Some(g_THEIR_PROBLEM()),
            "THING_OR_PERSON_CUES" => Some(g_THING_OR_PERSON_CUES()),
            "TIME_AFTER_A_PARTICLE" => Some(g_TIME_AFTER_A_PARTICLE()),
            "TIME_FILLERS" => Some(g_TIME_FILLERS()),
            "TIME_MARKERS" => Some(g_TIME_MARKERS()),
            "TIPS" => Some(g_TIPS()),
            "TIPS_ABOUT" => Some(g_TIPS_ABOUT()),
            "TIPS_AMOUNT" => Some(g_TIPS_AMOUNT()),
            "TIPS_FRAME" => Some(g_TIPS_FRAME()),
            "TIPS_SUBJECT" => Some(g_TIPS_SUBJECT()),
            "TIPS_WANTED" => Some(g_TIPS_WANTED()),
            "TIP_WORDS" => Some(g_TIP_WORDS()),
            "TO_A_VERB" => Some(g_TO_A_VERB()),
            "TURNED_ROUND" => Some(g_TURNED_ROUND()),
            "VERBISH" => Some(g_VERBISH()),
            "VERB_FORMS" => Some(g_VERB_FORMS()),
            "VERB_ONLY_STATE" => Some(g_VERB_ONLY_STATE()),
            "VERB_THIRD" => Some(g_VERB_THIRD()),
            "VERDICT_ADJECTIVES" => Some(g_VERDICT_ADJECTIVES()),
            "VERDICT_ASK" => Some(g_VERDICT_ASK()),
            "VERDICT_FILLER" => Some(g_VERDICT_FILLER()),
            "VERDICT_NOUNS" => Some(g_VERDICT_NOUNS()),
            "WHEN_IN_THE_ASK" => Some(g_WHEN_IN_THE_ASK()),
            "WHERE_BOT_TAIL" => Some(g_WHERE_BOT_TAIL()),
            "WHILE_ASK" => Some(g_WHILE_ASK()),
            "WHO_A_THING" => Some(g_WHO_A_THING()),
            "WHO_TAIL" => Some(g_WHO_TAIL()),
            "WHO_TALKING" => Some(g_WHO_TALKING()),
            "WH_CONTINUES" => Some(g_WH_CONTINUES()),
            "WORK_ON_ME" => Some(g_WORK_ON_ME()),
            "WRITE" => Some(g_WRITE()),
            "YESNO_STARTS" => Some(g_YESNO_STARTS()),
            "YOU_WORDS" => Some(g_YOU_WORDS()),
            "_ASKS_AFTER_A_COMMA" => Some(g__ASKS_AFTER_A_COMMA()),
            "_ASKS_AFTER_A_COMMA_WORD" => Some(g__ASKS_AFTER_A_COMMA_WORD()),
            "_ASKS_MID_LINE" => Some(g__ASKS_MID_LINE()),
            "_ASKS_WITH_AN_AUXILIARY" => Some(g__ASKS_WITH_AN_AUXILIARY()),
            "_ASKS_WITH_A_QUESTION_WORD" => Some(g__ASKS_WITH_A_QUESTION_WORD()),
            "_AServiceOnADay" => Some(py_func("_AServiceOnADay", |args: &[Py]| _AServiceOnADay(args))),
            "_AVAILABILITY" => Some(g__AVAILABILITY()),
            "_A_AN_AMOUNT_OF_A_THING" => Some(g__A_AN_AMOUNT_OF_A_THING()),
            "_A_AN_OCCASION_OF_THEIR_OWN" => Some(g__A_AN_OCCASION_OF_THEIR_OWN()),
            "_A_ARITHMETIC" => Some(g__A_ARITHMETIC()),
            "_A_A_CONVERSION" => Some(g__A_A_CONVERSION()),
            "_A_A_DIFFERENCE" => Some(g__A_A_DIFFERENCE()),
            "_A_A_PLAN_ABOUT_THE_WEATHER" => Some(g__A_A_PLAN_ABOUT_THE_WEATHER()),
            "_A_A_TRAFFIC_LIGHT" => Some(g__A_A_TRAFFIC_LIGHT()),
            "_A_BARE_ASK" => Some(g__A_BARE_ASK()),
            "_A_BARE_LIVE" => Some(g__A_BARE_LIVE()),
            "_A_BARE_LIVE_WORDS" => Some(g__A_BARE_LIVE_WORDS()),
            "_A_BARE_RECORD" => Some(g__A_BARE_RECORD()),
            "_A_BARE_TAIL" => Some(g__A_BARE_TAIL()),
            "_A_CLASS_OF_THING" => Some(g__A_CLASS_OF_THING()),
            "_A_COUNT" => Some(g__A_COUNT()),
            "_A_DAY_OR_TIME" => Some(g__A_DAY_OR_TIME()),
            "_A_DAY_WORD" => Some(g__A_DAY_WORD()),
            "_A_DEFINITION" => Some(g__A_DEFINITION()),
            "_A_DETERMINER" => Some(g__A_DETERMINER()),
            "_A_DID_YOU_SEE" => Some(g__A_DID_YOU_SEE()),
            "_A_FOLLOW_UP_WORDS" => Some(g__A_FOLLOW_UP_WORDS()),
            "_A_FRESHENING" => Some(g__A_FRESHENING()),
            "_A_HAS_A_NUMBER" => Some(g__A_HAS_A_NUMBER()),
            "_A_HISTORY" => Some(g__A_HISTORY()),
            "_A_HISTORY_ITSELF" => Some(g__A_HISTORY_ITSELF()),
            "_A_HOW_A_THING_MEASURES" => Some(g__A_HOW_A_THING_MEASURES()),
            "_A_HOW_IT_WORKS" => Some(g__A_HOW_IT_WORKS()),
            "_A_HOW_MANY" => Some(g__A_HOW_MANY()),
            "_A_HOW_MUCH_A_FEELING" => Some(g__A_HOW_MUCH_A_FEELING()),
            "_A_HOW_MUCH_ENOUGH" => Some(g__A_HOW_MUCH_ENOUGH()),
            "_A_HOW_MUCH_IS_IN" => Some(g__A_HOW_MUCH_IS_IN()),
            "_A_HOW_MUCH_IT_HOLDS" => Some(g__A_HOW_MUCH_IT_HOLDS()),
            "_A_HOW_MUCH_I_SPENT" => Some(g__A_HOW_MUCH_I_SPENT()),
            "_A_HOW_MUCH_OF_A_UNIT" => Some(g__A_HOW_MUCH_OF_A_UNIT()),
            "_A_HOW_OLD_IS_THE_WORLD" => Some(g__A_HOW_OLD_IS_THE_WORLD()),
            "_A_LINE_THAT_NAMES_A_PLACE" => Some(g__A_LINE_THAT_NAMES_A_PLACE()),
            "_A_MADE_OF" => Some(g__A_MADE_OF()),
            "_A_MEASURE" => Some(g__A_MEASURE()),
            "_A_METHOD" => Some(g__A_METHOD()),
            "_A_MY_OWN_FIGURE" => Some(g__A_MY_OWN_FIGURE()),
            "_A_MY_OWN_PLAN" => Some(g__A_MY_OWN_PLAN()),
            "_A_PAST_YEAR" => Some(g__A_PAST_YEAR()),
            "_A_PLACE_THAT_KEEPS_HOURS" => Some(g__A_PLACE_THAT_KEEPS_HOURS()),
            "_A_POST_HOLDER" => Some(g__A_POST_HOLDER()),
            "_A_PRICE_ASK" => Some(g__A_PRICE_ASK()),
            "_A_RECORD" => Some(g__A_RECORD()),
            "_A_REPORTS" => Some(g__A_REPORTS()),
            "_A_REPORTS_THE_ASK" => Some(g__A_REPORTS_THE_ASK()),
            "_A_SAYS_NOW" => Some(g__A_SAYS_NOW()),
            "_A_SERVICE" => Some(g__A_SERVICE()),
            "_A_SERVICE_STATE" => Some(g__A_SERVICE_STATE()),
            "_A_THE_CLOCK" => Some(g__A_THE_CLOCK()),
            "_A_WEATHER_HERE" => Some(g__A_WEATHER_HERE()),
            "_A_WORD_MEANS" => Some(g__A_WORD_MEANS()),
            "_A_WORKING" => Some(g__A_WORKING()),
            "_BARE_LIVE_LEAD" => Some(g__BARE_LIVE_LEAD()),
            "_CHANGING" => Some(g__CHANGING()),
            "_EVENT" => Some(g__EVENT()),
            "_HOURS" => Some(g__HOURS()),
            "_I_SAID_IT" => Some(g__I_SAID_IT()),
            "_MONEY" => Some(g__MONEY()),
            "_MONEY_ASKED" => Some(g__MONEY_ASKED()),
            "_MONEY_FOR" => Some(g__MONEY_FOR()),
            "_MONEY_NOUN" => Some(g__MONEY_NOUN()),
            "_NEWS" => Some(g__NEWS()),
            "_NIGHT_WISHES" => Some(g__NIGHT_WISHES()),
            "_NOT_A_FITTING_THAT_OPENS" => Some(g__NOT_A_FITTING_THAT_OPENS()),
            "_NOT_OPEN_TO_SOMEBODY" => Some(g__NOT_OPEN_TO_SOMEBODY()),
            "_RATE" => Some(g__RATE()),
            "_ROUTINE_SUBJECT" => Some(g__ROUTINE_SUBJECT()),
            "_SAID_ON_ITS_OWN" => Some(g__SAID_ON_ITS_OWN()),
            "_SEALED_ASKS" => Some(g__SEALED_ASKS()),
            "_SERVICE_ON_A_DAY" => Some(g__SERVICE_ON_A_DAY()),
            "_SETTLED" => Some(g__SETTLED()),
            "_SOUND_CUES" => Some(g__SOUND_CUES()),
            "_THANKS_WORDS" => Some(g__THANKS_WORDS()),
            "_TIMETABLE" => Some(g__TIMETABLE()),
            "_UNITS" => Some(g__UNITS()),
            "_UNITS_BOUGHT" => Some(g__UNITS_BOUGHT()),
            "_WEATHER" => Some(g__WEATHER()),
            "_WISH_WORDS" => Some(g__WISH_WORDS()),
            "_a_bare_definition" => Some(py_func("_a_bare_definition", |args: &[Py]| _a_bare_definition(args))),
            "_a_body_not_a_mood" => Some(py_func("_a_body_not_a_mood", |args: &[Py]| _a_body_not_a_mood(args))),
            "_a_body_part_before" => Some(py_func("_a_body_part_before", |args: &[Py]| _a_body_part_before(args))),
            "_a_broken_heart_is_theirs" => Some(py_func("_a_broken_heart_is_theirs", |args: &[Py]| _a_broken_heart_is_theirs(args))),
            "_a_class_of_thing" => Some(py_func("_a_class_of_thing", |args: &[Py]| _a_class_of_thing(args))),
            "_a_clause_of_its_own" => Some(py_func("_a_clause_of_its_own", |args: &[Py]| _a_clause_of_its_own(args))),
            "_a_copula_before" => Some(py_func("_a_copula_before", |args: &[Py]| _a_copula_before(args))),
            "_a_count_of_chats" => Some(py_func("_a_count_of_chats", |args: &[Py]| _a_count_of_chats(args))),
            "_a_direction" => Some(py_func("_a_direction", |args: &[Py]| _a_direction(args))),
            "_a_fall" => Some(py_func("_a_fall", |args: &[Py]| _a_fall(args))),
            "_a_fall_that_is_not_one" => Some(py_func("_a_fall_that_is_not_one", |args: &[Py]| _a_fall_that_is_not_one(args))),
            "_a_feeling_can_end_here" => Some(py_func("_a_feeling_can_end_here", |args: &[Py]| _a_feeling_can_end_here(args))),
            "_a_hedged_low" => Some(py_func("_a_hedged_low", |args: &[Py]| _a_hedged_low(args))),
            "_a_letter_someone_else_got" => Some(py_func("_a_letter_someone_else_got", |args: &[Py]| _a_letter_someone_else_got(args))),
            "_a_line_asks" => Some(py_func("_a_line_asks", |args: &[Py]| _a_line_asks(args))),
            "_a_name" => Some(py_func("_a_name", |args: &[Py]| _a_name(args))),
            "_a_period_of_their_own" => Some(py_func("_a_period_of_their_own", |args: &[Py]| _a_period_of_their_own(args))),
            "_a_person_they_like" => Some(py_func("_a_person_they_like", |args: &[Py]| _a_person_they_like(args))),
            "_a_state_of_the_speaker" => Some(py_func("_a_state_of_the_speaker", |args: &[Py]| _a_state_of_the_speaker(args))),
            "_a_step_done" => Some(py_func("_a_step_done", |args: &[Py]| _a_step_done(args))),
            "_a_suggestion_from_someone" => Some(py_func("_a_suggestion_from_someone", |args: &[Py]| _a_suggestion_from_someone(args))),
            "_a_thing_named_after_the_courtesy" => Some(py_func("_a_thing_named_after_the_courtesy", |args: &[Py]| _a_thing_named_after_the_courtesy(args))),
            "_a_thing_they_own" => Some(py_func("_a_thing_they_own", |args: &[Py]| _a_thing_they_own(args))),
            "_a_verb_holds_it" => Some(py_func("_a_verb_holds_it", |args: &[Py]| _a_verb_holds_it(args))),
            "_about_a_supply" => Some(py_func("_about_a_supply", |args: &[Py]| _about_a_supply(args))),
            "_about_bot" => Some(py_func("_about_bot", |args: &[Py]| _about_bot(args))),
            "_about_bot_kind" => Some(py_func("_about_bot_kind", |args: &[Py]| _about_bot_kind(args))),
            "_acts_on_the_speaker" => Some(py_func("_acts_on_the_speaker", |args: &[Py]| _acts_on_the_speaker(args))),
            "_after_farewell" => Some(py_func("_after_farewell", |args: &[Py]| _after_farewell(args))),
            "_after_greeting" => Some(py_func("_after_greeting", |args: &[Py]| _after_greeting(args))),
            "_after_thanks" => Some(py_func("_after_thanks", |args: &[Py]| _after_thanks(args))),
            "_agrees" => Some(py_func("_agrees", |args: &[Py]| _agrees(args))),
            "_agrees_to_try" => Some(py_func("_agrees_to_try", |args: &[Py]| _agrees_to_try(args))),
            "_as_typed" => Some(py_func("_as_typed", |args: &[Py]| _as_typed(args))),
            "_ask_labels" => Some(py_func("_ask_labels", |args: &[Py]| _ask_labels(args))),
            "_ask_words" => Some(py_func("_ask_words", |args: &[Py]| _ask_words(args))),
            "_asks" => Some(py_func("_asks", |args: &[Py]| _asks(args))),
            "_asks_a_question_coming" => Some(py_func("_asks_a_question_coming", |args: &[Py]| _asks_a_question_coming(args))),
            "_asks_a_verdict" => Some(py_func("_asks_a_verdict", |args: &[Py]| _asks_a_verdict(args))),
            "_asks_about_its_experience" => Some(py_func("_asks_about_its_experience", |args: &[Py]| _asks_about_its_experience(args))),
            "_asks_about_its_own_life" => Some(py_func("_asks_about_its_own_life", |args: &[Py]| _asks_about_its_own_life(args))),
            "_asks_about_the_bot_day" => Some(py_func("_asks_about_the_bot_day", |args: &[Py]| _asks_about_the_bot_day(args))),
            "_asks_after_any" => Some(py_func("_asks_after_any", |args: &[Py]| _asks_after_any(args))),
            "_asks_at_the_end" => Some(py_func("_asks_at_the_end", |args: &[Py]| _asks_at_the_end(args))),
            "_asks_back" => Some(py_func("_asks_back", |args: &[Py]| _asks_back(args))),
            "_asks_for_confirmation" => Some(py_func("_asks_for_confirmation", |args: &[Py]| _asks_for_confirmation(args))),
            "_asks_for_help" => Some(py_func("_asks_for_help", |args: &[Py]| _asks_for_help(args))),
            "_asks_for_it_rather_than_got_it" => Some(py_func("_asks_for_it_rather_than_got_it", |args: &[Py]| _asks_for_it_rather_than_got_it(args))),
            "_asks_for_tips" => Some(py_func("_asks_for_tips", |args: &[Py]| _asks_for_tips(args))),
            "_asks_how_it_spends_its_time" => Some(py_func("_asks_how_it_spends_its_time", |args: &[Py]| _asks_how_it_spends_its_time(args))),
            "_asks_how_the_bot_sees_me" => Some(py_func("_asks_how_the_bot_sees_me", |args: &[Py]| _asks_how_the_bot_sees_me(args))),
            "_asks_if_the_bot_is_a_product" => Some(py_func("_asks_if_the_bot_is_a_product", |args: &[Py]| _asks_if_the_bot_is_a_product(args))),
            "_asks_it_back" => Some(py_func("_asks_it_back", |args: &[Py]| _asks_it_back(args))),
            "_asks_the_bot" => Some(py_func("_asks_the_bot", |args: &[Py]| _asks_the_bot(args))),
            "_asks_the_room" => Some(py_func("_asks_the_room", |args: &[Py]| _asks_the_room(args))),
            "_asks_what_is_new_with_the_bot" => Some(py_func("_asks_what_is_new_with_the_bot", |args: &[Py]| _asks_what_is_new_with_the_bot(args))),
            "_asks_what_the_bot_has_been_doing" => Some(py_func("_asks_what_the_bot_has_been_doing", |args: &[Py]| _asks_what_the_bot_has_been_doing(args))),
            "_asks_what_the_bot_is_doing" => Some(py_func("_asks_what_the_bot_is_doing", |args: &[Py]| _asks_what_the_bot_is_doing(args))),
            "_asks_what_was_meant" => Some(py_func("_asks_what_was_meant", |args: &[Py]| _asks_what_was_meant(args))),
            "_asks_where_the_bot_is" => Some(py_func("_asks_where_the_bot_is", |args: &[Py]| _asks_where_the_bot_is(args))),
            "_asks_whether_the_bot_is_about" => Some(py_func("_asks_whether_the_bot_is_about", |args: &[Py]| _asks_whether_the_bot_is_about(args))),
            "_asks_who_this_is" => Some(py_func("_asks_who_this_is", |args: &[Py]| _asks_who_this_is(args))),
            "_bare_subject" => Some(py_func("_bare_subject", |args: &[Py]| _bare_subject(args))),
            "_cant_for_cannot" => Some(py_func("_cant_for_cannot", |args: &[Py]| _cant_for_cannot(args))),
            "_capability_question" => Some(py_func("_capability_question", |args: &[Py]| _capability_question(args))),
            "_carried_on" => Some(py_func("_carried_on", |args: &[Py]| _carried_on(args))),
            "_carried_over" => Some(py_func("_carried_over", |args: &[Py]| _carried_over(args))),
            "_claim_forms" => Some(py_func("_claim_forms", |args: &[Py]| _claim_forms(args))),
            "_clause_follows" => Some(py_func("_clause_follows", |args: &[Py]| _clause_follows(args))),
            "_continues_the_sentence" => Some(py_func("_continues_the_sentence", |args: &[Py]| _continues_the_sentence(args))),
            "_copula_before" => Some(py_func("_copula_before", |args: &[Py]| _copula_before(args))),
            "_copula_just_before" => Some(py_func("_copula_just_before", |args: &[Py]| _copula_just_before(args))),
            "_counts_its_own_doings" => Some(py_func("_counts_its_own_doings", |args: &[Py]| _counts_its_own_doings(args))),
            "_echoed_state" => Some(py_func("_echoed_state", |args: &[Py]| _echoed_state(args))),
            "_ends_with_thanks" => Some(py_func("_ends_with_thanks", |args: &[Py]| _ends_with_thanks(args))),
            "_feeling_said_as_a_sound" => Some(py_func("_feeling_said_as_a_sound", |args: &[Py]| _feeling_said_as_a_sound(args))),
            "_hands_a_question_to_the_speaker" => Some(py_func("_hands_a_question_to_the_speaker", |args: &[Py]| _hands_a_question_to_the_speaker(args))),
            "_has_any" => Some(py_func("_has_any", |args: &[Py]| _has_any(args))),
            "_how_long_itself" => Some(py_func("_how_long_itself", |args: &[Py]| _how_long_itself(args))),
            "_is_person" => Some(py_func("_is_person", |args: &[Py]| _is_person(args))),
            "_last_subject" => Some(py_func("_last_subject", |args: &[Py]| _last_subject(args))),
            "_last_subject_pos" => Some(py_func("_last_subject_pos", |args: &[Py]| _last_subject_pos(args))),
            "_laughed_off" => Some(py_func("_laughed_off", |args: &[Py]| _laughed_off(args))),
            "_limit_of" => Some(py_func("_limit_of", |args: &[Py]| _limit_of(args))),
            "_names_a_person" => Some(py_func("_names_a_person", |args: &[Py]| _names_a_person(args))),
            "_names_a_thing" => Some(py_func("_names_a_thing", |args: &[Py]| _names_a_thing(args))),
            "_names_an_episode" => Some(py_func("_names_an_episode", |args: &[Py]| _names_an_episode(args))),
            "_names_somebody_else" => Some(py_func("_names_somebody_else", |args: &[Py]| _names_somebody_else(args))),
            "_noun_reading" => Some(py_func("_noun_reading", |args: &[Py]| _noun_reading(args))),
            "_object_after" => Some(py_func("_object_after", |args: &[Py]| _object_after(args))),
            "_only_the_ask" => Some(py_func("_only_the_ask", |args: &[Py]| _only_the_ask(args))),
            "_opens_question" => Some(py_func("_opens_question", |args: &[Py]| _opens_question(args))),
            "_particle_is_a_place" => Some(py_func("_particle_is_a_place", |args: &[Py]| _particle_is_a_place(args))),
            "_polarity" => Some(py_func("_polarity", |args: &[Py]| _polarity(args))),
            "_praises_the_thing" => Some(py_func("_praises_the_thing", |args: &[Py]| _praises_the_thing(args))),
            "_recalls_their_own_line" => Some(py_func("_recalls_their_own_line", |args: &[Py]| _recalls_their_own_line(args))),
            "_ruled_out_of" => Some(py_func("_ruled_out_of", |args: &[Py]| _ruled_out_of(args))),
            "_said_of_the_speaker" => Some(py_func("_said_of_the_speaker", |args: &[Py]| _said_of_the_speaker(args))),
            "_said_to_somebody" => Some(py_func("_said_to_somebody", |args: &[Py]| _said_to_somebody(args))),
            "_says_nothing_but_the_floor" => Some(py_func("_says_nothing_but_the_floor", |args: &[Py]| _says_nothing_but_the_floor(args))),
            "_says_the_help_was_good" => Some(py_func("_says_the_help_was_good", |args: &[Py]| _says_the_help_was_good(args))),
            "_says_what_they_are" => Some(py_func("_says_what_they_are", |args: &[Py]| _says_what_they_are(args))),
            "_self_ask" => Some(py_func("_self_ask", |args: &[Py]| _self_ask(args))),
            "_short_reply" => Some(py_func("_short_reply", |args: &[Py]| _short_reply(args))),
            "_skeleton" => Some(py_func("_skeleton", |args: &[Py]| _skeleton(args))),
            "_somebody_elses_heart" => Some(py_func("_somebody_elses_heart", |args: &[Py]| _somebody_elses_heart(args))),
            "_sound_cues" => Some(py_func("_sound_cues", |args: &[Py]| _sound_cues(args))),
            "_speaker_before" => Some(py_func("_speaker_before", |args: &[Py]| _speaker_before(args))),
            "_starts_with" => Some(py_func("_starts_with", |args: &[Py]| _starts_with(args))),
            "_state_as_feeling" => Some(py_func("_state_as_feeling", |args: &[Py]| _state_as_feeling(args))),
            "_state_before" => Some(py_func("_state_before", |args: &[Py]| _state_before(args))),
            "_state_cue" => Some(py_func("_state_cue", |args: &[Py]| _state_cue(args))),
            "_state_of_a_person" => Some(py_func("_state_of_a_person", |args: &[Py]| _state_of_a_person(args))),
            "_state_owner" => Some(py_func("_state_owner", |args: &[Py]| _state_owner(args))),
            "_tag_checks_a_fact" => Some(py_func("_tag_checks_a_fact", |args: &[Py]| _tag_checks_a_fact(args))),
            "_tag_tells_a_plan" => Some(py_func("_tag_tells_a_plan", |args: &[Py]| _tag_tells_a_plan(args))),
            "_takes_an_object" => Some(py_func("_takes_an_object", |args: &[Py]| _takes_an_object(args))),
            "_talk_opening" => Some(py_func("_talk_opening", |args: &[Py]| _talk_opening(args))),
            "_text" => Some(py_func("_text", |args: &[Py]| _text(args))),
            "_thanks_only" => Some(py_func("_thanks_only", |args: &[Py]| _thanks_only(args))),
            "_thanks_still_asks" => Some(py_func("_thanks_still_asks", |args: &[Py]| _thanks_still_asks(args))),
            "_the_bots_own_not" => Some(py_func("_the_bots_own_not", |args: &[Py]| _the_bots_own_not(args))),
            "_the_doer_is_the_bot" => Some(py_func("_the_doer_is_the_bot", |args: &[Py]| _the_doer_is_the_bot(args))),
            "_the_step_went_wrong" => Some(py_func("_the_step_went_wrong", |args: &[Py]| _the_step_went_wrong(args))),
            "_the_thing_a_thank_you_is_for" => Some(py_func("_the_thing_a_thank_you_is_for", |args: &[Py]| _the_thing_a_thank_you_is_for(args))),
            "_thing_turned_down" => Some(py_func("_thing_turned_down", |args: &[Py]| _thing_turned_down(args))),
            "_third_person" => Some(py_func("_third_person", |args: &[Py]| _third_person(args))),
            "_turns_the_pair_down" => Some(py_func("_turns_the_pair_down", |args: &[Py]| _turns_the_pair_down(args))),
            "_verb_after" => Some(py_func("_verb_after", |args: &[Py]| _verb_after(args))),
            "_verb_form" => Some(py_func("_verb_form", |args: &[Py]| _verb_form(args))),
            "_wants_a_recommendation" => Some(py_func("_wants_a_recommendation", |args: &[Py]| _wants_a_recommendation(args))),
            "_when_itself" => Some(py_func("_when_itself", |args: &[Py]| _when_itself(args))),
            "_whose_state" => Some(py_func("_whose_state", |args: &[Py]| _whose_state(args))),
            "_without_mood_tail" => Some(py_func("_without_mood_tail", |args: &[Py]| _without_mood_tail(args))),
            "_works_on_the_speaker" => Some(py_func("_works_on_the_speaker", |args: &[Py]| _works_on_the_speaker(args))),
            "a_bad_verdict" => Some(py_func("a_bad_verdict", |args: &[Py]| a_bad_verdict(args))),
            "a_broken_heart_shape" => Some(py_func("a_broken_heart_shape", |args: &[Py]| a_broken_heart_shape(args))),
            "a_care_home" => Some(py_func("a_care_home", |args: &[Py]| a_care_home(args))),
            "a_home_carried_forward" => Some(py_func("a_home_carried_forward", |args: &[Py]| a_home_carried_forward(args))),
            "a_stop_ask" => Some(py_func("a_stop_ask", |args: &[Py]| a_stop_ask(args))),
            "after_any_link" => Some(py_func("after_any_link", |args: &[Py]| after_any_link(args))),
            "after_greeting" => Some(py_func("after_greeting", |args: &[Py]| after_greeting(args))),
            "agreement_of" => Some(py_func("agreement_of", |args: &[Py]| agreement_of(args))),
            "asks_how_i_am" => Some(py_func("asks_how_i_am", |args: &[Py]| asks_how_i_am(args))),
            "classify" => Some(py_func("classify", |args: &[Py]| classify(args))),
            "cue_about_person" => Some(py_func("cue_about_person", |args: &[Py]| cue_about_person(args))),
            "denied" => Some(py_func("denied", |args: &[Py]| denied(args))),
            "emotion_of" => Some(py_func("emotion_of", |args: &[Py]| emotion_of(args))),
            "farewell_only" => Some(py_func("farewell_only", |args: &[Py]| farewell_only(args))),
            "feeling_by_sound" => Some(py_func("feeling_by_sound", |args: &[Py]| feeling_by_sound(args))),
            "greeting_only" => Some(py_func("greeting_only", |args: &[Py]| greeting_only(args))),
            "is_a_routine" => Some(py_func("is_a_routine", |args: &[Py]| is_a_routine(args))),
            "names_a_claim" => Some(py_func("names_a_claim", |args: &[Py]| names_a_claim(args))),
            "noun_phrase_request" => Some(py_func("noun_phrase_request", |args: &[Py]| noun_phrase_request(args))),
            "opening_line" => Some(py_func("opening_line", |args: &[Py]| opening_line(args))),
            "over_time" => Some(py_func("over_time", |args: &[Py]| over_time(args))),
            "says_how_they_are" => Some(py_func("says_how_they_are", |args: &[Py]| says_how_they_are(args))),
            "shift_worked" => Some(py_func("shift_worked", |args: &[Py]| shift_worked(args))),
            "shortage_of" => Some(py_func("shortage_of", |args: &[Py]| shortage_of(args))),
            "small_talk_only" => Some(py_func("small_talk_only", |args: &[Py]| small_talk_only(args))),
            "summons_of" => Some(py_func("summons_of", |args: &[Py]| summons_of(args))),
            "taking_up" => Some(py_func("taking_up", |args: &[Py]| taking_up(args))),
            "talk_frame" => Some(py_func("talk_frame", |args: &[Py]| talk_frame(args))),
            "the_line_after_a_courtesy" => Some(py_func("the_line_after_a_courtesy", |args: &[Py]| the_line_after_a_courtesy(args))),
            "the_manners_question" => Some(py_func("the_manners_question", |args: &[Py]| the_manners_question(args))),
            "the_setup" => Some(py_func("the_setup", |args: &[Py]| the_setup(args))),
            "two_ends_of_one_scale" => Some(py_func("two_ends_of_one_scale", |args: &[Py]| two_ends_of_one_scale(args))),
            "verdicts_a_thing" => Some(py_func("verdicts_a_thing", |args: &[Py]| verdicts_a_thing(args))),
            "without_filler" => Some(py_func("without_filler", |args: &[Py]| without_filler(args))),
            _ => None,
        }),
        set: None,
    }))
}
